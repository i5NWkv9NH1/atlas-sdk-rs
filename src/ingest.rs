//! One-shot compound media ingestion builder for Atlas.
//!
//! Provides a fluent builder pattern to ingest media from bytes, base64 strings,
//! local file paths, or remote URLs while atomically configuring tags, collections,
//! and metadata.

use crate::AtlasClient;
use crate::models::*;
use crate::transport::{AtlasError, AtlasTransport, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Result returned by `IngestBuilder::send()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct IngestResult {
    /// Unique item ID of the imported media (first item ID).
    pub item_id: String,
    /// All unique item IDs of the imported media.
    #[serde(default)]
    pub item_ids: Vec<String>,
    /// Whether the import was reported successful.
    pub imported: bool,
    /// Whether the item was skipped because it is an existing duplicate in the library.
    #[serde(default)]
    pub skipped_duplicate: bool,
    /// Target collections the item was assigned to.
    #[serde(default)]
    pub collections: Vec<String>,
    /// Tags assigned to the item.
    #[serde(default)]
    pub tags: Vec<String>,
}

enum IngestSource {
    Bytes { data: Vec<u8>, ext: String },
    Base64 { b64: String, ext: String },
    FilePath(String),
    FilePaths(Vec<String>),
    Url(String),
}

/// RAII guard to guarantee cleanup of temporary ingestion directories.
struct TempDirGuard(Option<PathBuf>);

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        if let Some(ref path) = self.0 {
            let _ = std::fs::remove_dir_all(path);
        }
    }
}

/// Helper function to decode standard and URL-safe base64 strings without external dependencies.
fn decode_base64_str(s: &str) -> Vec<u8> {
    let mut table = [0u8; 256];
    const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for (i, &c) in B64_CHARS.iter().enumerate() {
        table[c as usize] = i as u8;
    }
    // Also support URL-safe base64: '-' maps to 62 (same as '+'), '_' maps to 63 (same as '/')
    table[b'-' as usize] = 62;
    table[b'_' as usize] = 63;

    let mut out = Vec::new();
    let bytes: Vec<u8> = s
        .bytes()
        .filter(|b| !b.is_ascii_whitespace() && *b != b'=')
        .collect();

    for chunk in bytes.chunks(4) {
        let b0 = table[chunk[0] as usize];
        let b1 = if chunk.len() > 1 {
            table[chunk[1] as usize]
        } else {
            0
        };
        let b2 = if chunk.len() > 2 {
            table[chunk[2] as usize]
        } else {
            0
        };
        let b3 = if chunk.len() > 3 {
            table[chunk[3] as usize]
        } else {
            0
        };

        if chunk.len() >= 2 {
            out.push((b0 << 2) | (b1 >> 4));
        }
        if chunk.len() >= 3 {
            out.push(((b1 & 0x0f) << 4) | (b2 >> 2));
        }
        if chunk.len() >= 4 {
            out.push(((b2 & 0x03) << 6) | b3);
        }
    }

    out
}

/// Fluent builder for composite one-shot ingestion.
pub struct IngestBuilder<'a, T: AtlasTransport> {
    client: &'a AtlasClient<T>,
    source: Option<IngestSource>,
    custom_filename: Option<String>,
    target_collections: Vec<String>,
    target_tags: Vec<String>,
    meta_annotation: Option<String>,
    meta_author: Option<String>,
    meta_source_url: Option<String>,
    meta_location: Option<String>,
    is_liked: Option<bool>,
    canvas_x: Option<f64>,
    canvas_y: Option<f64>,
}

impl<'a, T: AtlasTransport> IngestBuilder<'a, T> {
    /// Create a new IngestBuilder instance.
    pub fn new(client: &'a AtlasClient<T>) -> Self {
        Self {
            client,
            source: None,
            custom_filename: None,
            target_collections: Vec::new(),
            target_tags: Vec::new(),
            meta_annotation: None,
            meta_author: None,
            meta_source_url: None,
            meta_location: None,
            is_liked: None,
            canvas_x: None,
            canvas_y: None,
        }
    }

    /// Set binary bytes as input source with file extension.
    pub fn bytes(mut self, data: impl Into<Vec<u8>>, ext: impl Into<String>) -> Self {
        self.source = Some(IngestSource::Bytes {
            data: data.into(),
            ext: ext.into(),
        });
        self
    }

    /// Set base64 string as input source (automatically strips data URL headers and detects MIME).
    pub fn base64(mut self, b64: impl Into<String>, ext: impl Into<String>) -> Self {
        let raw = b64.into();
        let (clean_b64, detected_ext) = if raw.starts_with("data:") {
            if let Some(idx) = raw.find(";base64,") {
                let mime = raw[5..idx].to_lowercase();
                let ext_from_mime = match mime.as_str() {
                    "image/webp" => "webp",
                    "image/gif" => "gif",
                    "video/mp4" => "mp4",
                    "video/quicktime" => "mov",
                    "image/jpeg" | "image/jpg" => "jpg",
                    "image/svg+xml" => "svg",
                    "image/heic" | "image/heif" => "heic",
                    "image/avif" => "avif",
                    "image/png" => "png",
                    _ => mime.split('/').nth(1).unwrap_or("png"),
                };
                let body = &raw[idx + 8..];
                (body.trim().to_string(), Some(ext_from_mime.to_string()))
            } else {
                (raw.clone(), None)
            }
        } else {
            (raw.trim().to_string(), None)
        };

        let ext_str = ext.into();
        let final_ext = if !ext_str.is_empty() && ext_str != "png" {
            ext_str
        } else if let Some(m_ext) = detected_ext {
            m_ext
        } else if !ext_str.is_empty() {
            ext_str
        } else {
            "png".to_string()
        };

        self.source = Some(IngestSource::Base64 {
            b64: clean_b64,
            ext: final_ext,
        });
        self
    }

    /// Set an existing local absolute file path as input source.
    pub fn file_path(mut self, path: impl Into<String>) -> Self {
        self.source = Some(IngestSource::FilePath(path.into()));
        self
    }

    /// Set multiple existing local absolute file paths as input source for batch ingestion.
    pub fn file_paths(mut self, paths: Vec<String>) -> Self {
        self.source = Some(IngestSource::FilePaths(paths));
        self
    }

    /// Set web media URL as input source.
    pub fn url(mut self, media_url: impl Into<String>) -> Self {
        self.source = Some(IngestSource::Url(media_url.into()));
        self
    }

    /// Specify a custom filename for the ingested item (cleans slashes to prevent subpath errors).
    pub fn filename(mut self, name: impl Into<String>) -> Self {
        let raw = name.into();
        self.custom_filename = Some(raw.replace(['/', '\\'], "_"));
        self
    }

    /// Assign item to a single collection.
    pub fn collection(mut self, col: impl Into<String>) -> Self {
        let name = col.into();
        if !self.target_collections.contains(&name) {
            self.target_collections.push(name);
        }
        self
    }

    /// Assign item to multiple collections.
    pub fn collections(mut self, cols: Vec<String>) -> Self {
        for col in cols {
            if !self.target_collections.contains(&col) {
                self.target_collections.push(col);
            }
        }
        self
    }

    /// Attach a tag to the ingested item.
    pub fn tag(mut self, t: impl Into<String>) -> Self {
        let name = t.into();
        if !self.target_tags.contains(&name) {
            self.target_tags.push(name);
        }
        self
    }

    /// Attach multiple tags to the ingested item.
    pub fn tags(mut self, ts: Vec<String>) -> Self {
        for t in ts {
            if !self.target_tags.contains(&t) {
                self.target_tags.push(t);
            }
        }
        self
    }

    /// Set item text annotation / description.
    pub fn annotation(mut self, note: impl Into<String>) -> Self {
        self.meta_annotation = Some(note.into());
        self
    }

    /// Set item author name.
    pub fn author(mut self, name: impl Into<String>) -> Self {
        self.meta_author = Some(name.into());
        self
    }

    /// Set item source URL.
    pub fn source_url(mut self, url: impl Into<String>) -> Self {
        self.meta_source_url = Some(url.into());
        self
    }

    /// Set item location metadata.
    pub fn location(mut self, loc: impl Into<String>) -> Self {
        self.meta_location = Some(loc.into());
        self
    }

    /// Set item favorite / liked status.
    pub fn liked(mut self, is_liked: bool) -> Self {
        self.is_liked = Some(is_liked);
        self
    }

    /// Set board canvas position coordinates.
    pub fn canvas_position(mut self, x: f64, y: f64) -> Self {
        self.canvas_x = Some(x);
        self.canvas_y = Some(y);
        self
    }

    /// Execute the compound ingestion pipeline and return the created item summary.
    pub async fn send(&self) -> Result<IngestResult> {
        let source = self.source.as_ref().ok_or_else(|| AtlasError::ToolError {
            tool_name: "ingest".to_string(),
            message: "No media source specified. Call .bytes(), .base64(), .file_path(), .file_paths(), or .url() first."
                .to_string(),
        })?;

        let mut temp_guard = TempDirGuard(None);
        let mut paths_to_import: Vec<String> = Vec::new();

        // * 1. Prepare temporary disk file if in-memory bytes or base64 was provided
        match source {
            IngestSource::Bytes { data, ext } => {
                let clean_ext = ext.trim_start_matches('.');
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0);
                let temp_dir = PathBuf::from(format!(
                    "/tmp/atlas-ingest-{}-{}",
                    std::process::id(),
                    nanos
                ));

                tokio::fs::create_dir_all(&temp_dir).await?;
                temp_guard.0 = Some(temp_dir.clone());

                let file_name = match self.custom_filename.as_ref() {
                    Some(name) => {
                        if name.contains('.') {
                            name.clone()
                        } else {
                            format!("{name}.{clean_ext}")
                        }
                    }
                    None => format!("ingest-{nanos}.{clean_ext}"),
                };

                let file_path = temp_dir.join(file_name);
                tokio::fs::write(&file_path, data).await?;
                paths_to_import.push(file_path.to_string_lossy().to_string());
            }
            IngestSource::Base64 { b64, ext } => {
                let clean_ext = ext.trim_start_matches('.');
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0);
                let temp_dir = PathBuf::from(format!(
                    "/tmp/atlas-ingest-{}-{}",
                    std::process::id(),
                    nanos
                ));

                tokio::fs::create_dir_all(&temp_dir).await?;
                temp_guard.0 = Some(temp_dir.clone());

                let file_name = match self.custom_filename.as_ref() {
                    Some(name) => {
                        if name.contains('.') {
                            name.clone()
                        } else {
                            format!("{name}.{clean_ext}")
                        }
                    }
                    None => format!("ingest-{nanos}.{clean_ext}"),
                };

                let file_path = temp_dir.join(file_name);
                let decoded = decode_base64_str(b64);
                tokio::fs::write(&file_path, decoded).await?;
                paths_to_import.push(file_path.to_string_lossy().to_string());
            }
            IngestSource::FilePath(p) => {
                paths_to_import.push(p.clone());
            }
            IngestSource::FilePaths(ps) => {
                paths_to_import.extend(ps.clone());
            }
            IngestSource::Url(_) => {}
        }

        let first_col = self.target_collections.first().cloned();

        // * 2. Ingest file or download URL
        let (all_item_ids, is_imported, is_duplicate) = if let IngestSource::Url(url) = source {
            let res = self
                .client
                .save_media_from_url(&SaveMediaFromUrlParams {
                    url: url.clone(),
                    collection: first_col,
                })
                .await?;

            let mut ids = Vec::new();
            if let Some(res_ids) = res.item_ids {
                ids.extend(res_ids);
            } else if let Some(id) = res.item_id {
                ids.push(id);
            }

            // ? Fallback search by filename or URL if not directly returned
            if ids.is_empty() {
                let search_res = self
                    .client
                    .search_items(&SearchItemsParams {
                        query: Some(self.custom_filename.clone().unwrap_or_else(|| url.clone())),
                        limit: Some(1),
                        sort: Some(ItemSortField::DateAdded),
                        order: Some(SortOrder::Desc),
                        ..Default::default()
                    })
                    .await?;
                if let Some(first_item) = search_res.items.first() {
                    ids.push(first_item.id.clone());
                }
            }
            let imported = !ids.is_empty();
            (ids, imported, false)
        } else {
            if paths_to_import.is_empty() {
                return Err(AtlasError::ToolError {
                    tool_name: "ingest".to_string(),
                    message: "No file paths provided to import".to_string(),
                });
            }

            let import_res = self
                .client
                .import_files(&ImportFilesParams {
                    paths: paths_to_import.clone(),
                    collection: first_col,
                    x: self.canvas_x,
                    y: self.canvas_y,
                })
                .await?;

            // ! 1. Check for failed array
            if let Some(failed_list) = &import_res.failed {
                if let Some(first_fail) = failed_list.first() {
                    let path = first_fail
                        .get("path")
                        .and_then(|p| p.as_str())
                        .unwrap_or("unknown");
                    let reason = first_fail
                        .get("reason")
                        .and_then(|r| r.as_str())
                        .unwrap_or("unknown error");
                    return Err(AtlasError::ToolError {
                        tool_name: "import_files".to_string(),
                        message: format!("Import failed for '{path}': {reason}"),
                    });
                }
            }

            // ! 2. Check for unsupported format
            if import_res.skipped_unsupported.unwrap_or(0) > 0 && import_res.imported == 0 {
                return Err(AtlasError::ToolError {
                    tool_name: "import_files".to_string(),
                    message: "File format is unsupported by Atlas".to_string(),
                });
            }

            let duplicate = import_res.skipped_duplicate.unwrap_or(0) > 0;
            let mut ids = import_res.item_ids.unwrap_or_default();
            let imported = import_res.imported > 0 || (!ids.is_empty() && !duplicate);

            // ? 3. Fallback search by filename only if imported > 0 or skipped_duplicate > 0
            if ids.is_empty() && (imported || duplicate) {
                let query_term = self.custom_filename.clone().or_else(|| {
                    paths_to_import.first().and_then(|p| {
                        PathBuf::from(p)
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                    })
                });

                if let Some(term) = query_term {
                    let search_res = self
                        .client
                        .search_items(&SearchItemsParams {
                            query: Some(term),
                            limit: Some(if paths_to_import.len() > 1 {
                                paths_to_import.len() as i64
                            } else {
                                1
                            }),
                            sort: Some(ItemSortField::DateAdded),
                            order: Some(SortOrder::Desc),
                            ..Default::default()
                        })
                        .await?;
                    for item in search_res.items {
                        ids.push(item.id);
                    }
                }
            }
            (ids, imported, duplicate)
        };

        // ! Fatal: item ID could not be identified
        if all_item_ids.is_empty() {
            return Err(AtlasError::ToolError {
                tool_name: "ingest".to_string(),
                message: "Failed to determine item_id from Atlas import response".to_string(),
            });
        }

        let primary_item_id = all_item_ids[0].clone();

        // * 3. Add to remaining collections
        let additional_collections = if self.target_collections.len() > 1 {
            &self.target_collections[1..]
        } else {
            &[]
        };

        for col in additional_collections {
            self.client
                .add_to_collection(&AddToCollectionParams {
                    collection: col.clone(),
                    item_ids: all_item_ids.clone(),
                    r#move: None,
                })
                .await?;
        }

        // * 4. Attach tags to ALL items
        if !self.target_tags.is_empty() {
            self.client
                .add_tags(&AddTagsParams {
                    item_ids: all_item_ids.clone(),
                    tags: self.target_tags.clone(),
                })
                .await?;
        }

        // * 5. Update metadata for ALL items
        if self.meta_annotation.is_some()
            || self.meta_author.is_some()
            || self.meta_source_url.is_some()
            || self.meta_location.is_some()
        {
            for id in &all_item_ids {
                self.client
                    .set_metadata(&SetMetadataParams {
                        item_id: id.clone(),
                        annotation: self.meta_annotation.clone(),
                        author: self.meta_author.clone(),
                        source_url: self.meta_source_url.clone(),
                        location: self.meta_location.clone(),
                    })
                    .await?;
            }
        }

        // * 6. Set liked state for ALL items
        if let Some(liked) = self.is_liked {
            self.client
                .set_liked(&SetLikedParams {
                    item_ids: all_item_ids.clone(),
                    liked,
                })
                .await?;
        }

        Ok(IngestResult {
            item_id: primary_item_id,
            item_ids: all_item_ids,
            imported: is_imported,
            skipped_duplicate: is_duplicate,
            collections: self.target_collections.clone(),
            tags: self.target_tags.clone(),
        })
    }
}
