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
    /// Unique item ID of the imported media in the Atlas library.
    pub item_id: String,
    /// Whether the import was reported successful.
    pub imported: bool,
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

/// Helper function to decode standard base64 strings without external dependencies.
fn decode_base64_str(s: &str) -> Vec<u8> {
    let mut table = [0u8; 256];
    const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for (i, &c) in B64_CHARS.iter().enumerate() {
        table[c as usize] = i as u8;
    }

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
    /// Create a new IngestBuilder associated with an AtlasClient.
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

    /// Set in-memory binary bytes and file extension as the input source.
    pub fn bytes(mut self, data: impl Into<Vec<u8>>, ext: impl Into<String>) -> Self {
        self.source = Some(IngestSource::Bytes {
            data: data.into(),
            ext: ext.into(),
        });
        self
    }

    /// Set a base64 encoded string as the input source (automatically strips data URL headers).
    pub fn base64(mut self, b64: impl Into<String>, ext: impl Into<String>) -> Self {
        let raw = b64.into();
        let clean = if let Some(idx) = raw.find(";base64,") {
            raw[idx + 8..].trim().to_string()
        } else {
            raw.trim().to_string()
        };

        self.source = Some(IngestSource::Base64 {
            b64: clean,
            ext: ext.into(),
        });
        self
    }

    /// Set an existing local absolute file path as the input source.
    pub fn file_path(mut self, path: impl Into<String>) -> Self {
        self.source = Some(IngestSource::FilePath(path.into()));
        self
    }

    /// Set a web media URL as the input source.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.source = Some(IngestSource::Url(url.into()));
        self
    }

    /// Specify a custom filename for the ingested item.
    pub fn filename(mut self, name: impl Into<String>) -> Self {
        self.custom_filename = Some(name.into());
        self
    }

    /// Assign the item to a target collection.
    pub fn collection(mut self, col: impl Into<String>) -> Self {
        let c = col.into();
        if !self.target_collections.contains(&c) {
            self.target_collections.push(c);
        }
        self
    }

    /// Assign the item to multiple target collections.
    pub fn collections<I, S>(mut self, cols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for c in cols {
            self = self.collection(c);
        }
        self
    }

    /// Assign tags to the item.
    pub fn tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for t in tags {
            self.target_tags.push(t.into());
        }
        self
    }

    /// Set item text annotation / notes.
    pub fn annotation(mut self, note: impl Into<String>) -> Self {
        self.meta_annotation = Some(note.into());
        self
    }

    /// Set item author or creator.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.meta_author = Some(author.into());
        self
    }

    /// Set item source / origin URL.
    pub fn source_url(mut self, url: impl Into<String>) -> Self {
        self.meta_source_url = Some(url.into());
        self
    }

    /// Set item geographical or physical location.
    pub fn location(mut self, loc: impl Into<String>) -> Self {
        self.meta_location = Some(loc.into());
        self
    }

    /// Set whether the item is marked as liked / favorite.
    pub fn liked(mut self, liked: bool) -> Self {
        self.is_liked = Some(liked);
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
            message: "No media source specified. Call .bytes(), .base64(), .file_path(), or .url() first."
                .to_string(),
        })?;

        let mut temp_guard = TempDirGuard(None);
        let mut resolved_path: Option<String> = None;

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
                resolved_path = Some(file_path.to_string_lossy().to_string());
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
                resolved_path = Some(file_path.to_string_lossy().to_string());
            }
            IngestSource::FilePath(p) => {
                resolved_path = Some(p.clone());
            }
            IngestSource::Url(_) => {}
        }

        // * 2. Ingest file or download URL
        let item_id: Option<String> = if let IngestSource::Url(url) = source {
            let res = self
                .client
                .save_media_from_url(&SaveMediaFromUrlParams {
                    url: url.clone(),
                    ..Default::default()
                })
                .await?;

            let mut id = res
                .item_ids
                .as_ref()
                .and_then(|ids| ids.first().cloned())
                .or(res.item_id);

            // ? Fallback search by filename or URL if not directly returned
            if id.is_none() {
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
                id = search_res.items.first().map(|i| i.id.clone());
            }
            id
        } else {
            let path_to_import = resolved_path
                .as_ref()
                .ok_or_else(|| AtlasError::ToolError {
                    tool_name: "ingest".to_string(),
                    message: "Resolved file path is unexpectedly missing".to_string(),
                })?;

            let first_col = self.target_collections.first().cloned();
            let import_res = self
                .client
                .import_files(&ImportFilesParams {
                    paths: vec![path_to_import.clone()],
                    collection: first_col,
                    ..Default::default()
                })
                .await?;

            let mut id = import_res
                .item_ids
                .as_ref()
                .and_then(|ids| ids.first().cloned());

            // ? Fallback search by filename if duplicate skipped or not directly returned
            if id.is_none() {
                let query_term = self.custom_filename.clone().or_else(|| {
                    PathBuf::from(path_to_import)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                });

                if let Some(term) = query_term {
                    let search_res = self
                        .client
                        .search_items(&SearchItemsParams {
                            query: Some(term),
                            limit: Some(1),
                            sort: Some(ItemSortField::DateAdded),
                            order: Some(SortOrder::Desc),
                            ..Default::default()
                        })
                        .await?;
                    id = search_res.items.first().map(|i| i.id.clone());
                }
            }
            id
        };

        // ! Fatal: item ID could not be identified
        let final_item_id = item_id.ok_or_else(|| AtlasError::ToolError {
            tool_name: "ingest".to_string(),
            message: "Failed to determine item_id from Atlas import response".to_string(),
        })?;

        // * 3. Add to remaining collections
        let is_url = matches!(source, IngestSource::Url(_));
        let remaining_collections = if is_url {
            &self.target_collections[..]
        } else if self.target_collections.len() > 1 {
            &self.target_collections[1..]
        } else {
            &[]
        };

        for col in remaining_collections {
            self.client
                .add_to_collection(&AddToCollectionParams {
                    collection: col.clone(),
                    item_ids: vec![final_item_id.clone()],
                    ..Default::default()
                })
                .await?;
        }

        // * 4. Attach tags
        if !self.target_tags.is_empty() {
            self.client
                .add_tags(&AddTagsParams {
                    item_ids: vec![final_item_id.clone()],
                    tags: self.target_tags.clone(),
                })
                .await?;
        }

        // * 5. Update metadata
        if self.meta_annotation.is_some()
            || self.meta_author.is_some()
            || self.meta_source_url.is_some()
            || self.meta_location.is_some()
        {
            self.client
                .set_metadata(&SetMetadataParams {
                    item_id: final_item_id.clone(),
                    annotation: self.meta_annotation.clone(),
                    author: self.meta_author.clone(),
                    source_url: self.meta_source_url.clone(),
                    location: self.meta_location.clone(),
                })
                .await?;
        }

        // * 6. Set liked state
        if let Some(liked) = self.is_liked {
            self.client
                .set_liked(&SetLikedParams {
                    item_ids: vec![final_item_id.clone()],
                    liked,
                })
                .await?;
        }

        // Temp directory will be deleted when `temp_guard` drops at end of function
        Ok(IngestResult {
            item_id: final_item_id,
            imported: true,
            collections: self.target_collections.clone(),
            tags: self.target_tags.clone(),
        })
    }
}
