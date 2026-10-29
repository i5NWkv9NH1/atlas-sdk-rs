// * Atlas Rust SDK Live E2E Test Suite
// * Strictly adheres to Section 5.2 safety redlines: non-destructive, isolated temporary test item only.

use sora_atlas::*;
use std::path::Path;

const TEST_IMAGE_PATH: &str = "/tmp/atlas-rs-e2e-test.png";

// * Minimal valid 1x1 PNG base64 representation
const TINY_PNG_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

// ============================================================================
// 1. Safe Read-Only Queries
// ============================================================================
#[tokio::test]
async fn test_live_read_only_queries() {
    let client = AtlasClient::new();

    // ! Mandatory assertion: Atlas MCP server must be running and responsive
    let ping_ok = client.ping().await.expect("Failed to execute ping");
    assert!(
        ping_ok,
        "Atlas server must be live and responsive on 127.0.0.1:50473/mcp"
    );

    // 1. App Info
    let app_info = client.app_info().await.expect("Failed to fetch app_info");
    assert_eq!(app_info.name, "Atlas");
    assert!(!app_info.version.is_empty());
    assert!(app_info.build > 0);

    // 2. Library Info
    let lib_info = client
        .library_info(&LibraryInfoParams::default())
        .await
        .expect("Failed to fetch library_info");
    assert!(!lib_info.name.is_empty());
    assert!(lib_info.unique_items > 0);

    // 3. List Libraries
    let libraries = client
        .list_libraries()
        .await
        .expect("Failed to list libraries");
    assert!(!libraries.libraries.is_empty());

    // 4. List Collections
    let collections = client
        .list_collections()
        .await
        .expect("Failed to list collections");
    let _ = collections.collections;

    // 5. List Tags
    let tags = client.list_tags().await.expect("Failed to list tags");
    let _ = tags.tags;
}

// ============================================================================
// 2. Isolated Temporary Ingestion, Mutation, and Teardown Verification
// ============================================================================
#[tokio::test]
async fn test_live_isolated_ingestion_and_teardown() {
    let client = AtlasClient::new();

    // ! Mandatory assertion: Atlas MCP server must be running
    let ping_ok = client
        .ping()
        .await
        .expect("Failed to ping Atlas MCP server");
    assert!(ping_ok, "Atlas server must be running");

    // * Step 1: Decode and write temporary 1x1 PNG file to disk
    // Minimal standard base64 decoding
    fn decode_base64(s: &str) -> Vec<u8> {
        let mut table = [0u8; 256];
        const B64_CHARS: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
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

            out.push((b0 << 2) | (b1 >> 4));
            if chunk.len() > 2 {
                out.push((b1 << 4) | (b2 >> 2));
            }
            if chunk.len() > 3 {
                out.push((b2 << 6) | b3);
            }
        }
        out
    }

    let png_bytes = decode_base64(TINY_PNG_BASE64);
    std::fs::write(TEST_IMAGE_PATH, &png_bytes).expect("Failed to write temporary test PNG");
    assert!(Path::new(TEST_IMAGE_PATH).exists());

    let mut test_item_id: Option<String> = None;

    let test_run = async {
        // * Step 2: Import temporary file into active library
        let import_res = client
            .import_files(&ImportFilesParams {
                paths: vec![TEST_IMAGE_PATH.to_string()],
                ..Default::default()
            })
            .await
            .expect("Failed to import test file");
        assert!(import_res.ok);
        assert!(import_res.imported >= 1, "Must import at least 1 file");

        // * Step 3: Capture created testItemId from import result or search
        let mut extracted_id = import_res
            .item_ids
            .as_ref()
            .and_then(|ids| ids.first())
            .cloned();

        if extracted_id.is_none() {
            let recent = client
                .search_items(&SearchItemsParams {
                    query: Some("atlas-rs-e2e-test.png".to_string()),
                    limit: Some(1),
                    sort: Some(ItemSortField::DateAdded),
                    order: Some(SortOrder::Desc),
                    ..Default::default()
                })
                .await
                .expect("Failed to search imported test item");

            assert!(
                !recent.items.is_empty(),
                "Must locate newly imported test image item"
            );
            extracted_id = Some(recent.items[0].id.clone());
        }

        let item_id = extracted_id.expect("Failed to capture imported test item ID");
        test_item_id = Some(item_id.clone());

        // * Step 4: Mutate metadata and tags strictly on testItemId
        let meta_res = client
            .set_metadata(&SetMetadataParams {
                item_id: item_id.clone(),
                annotation: Some("atlas-rs-e2e-verification-ok".to_string()),
                ..Default::default()
            })
            .await
            .expect("Failed to set test item metadata");
        assert!(meta_res.ok);

        let tag_res = client
            .add_tags(&AddTagsParams {
                item_ids: vec![item_id.clone()],
                tags: vec!["rs-e2e-test-tag".to_string()],
            })
            .await
            .expect("Failed to add test tag");
        assert!(tag_res.ok);

        // * Step 5: Read back item details and verify mutated state
        let item_detail = client
            .get_item(&GetItemParams {
                item_id: item_id.clone(),
            })
            .await
            .expect("Failed to get test item detail");

        assert_eq!(item_detail.id, item_id);
        assert_eq!(
            item_detail.annotation.as_deref(),
            Some("atlas-rs-e2e-verification-ok")
        );
        let tags = item_detail.tags.unwrap_or_default();
        assert!(
            tags.contains(&"rs-e2e-test-tag".to_string()),
            "Item tags must contain mutated test tag"
        );
    };

    test_run.await;

    // * Step 6: Safe Teardown: Trash test item and delete temporary disk file
    if let Some(ref id) = test_item_id {
        let trash_res = client
            .trash_items(&TrashItemsParams {
                item_ids: vec![id.clone()],
            })
            .await;
        if let Ok(res) = trash_res {
            assert!(res.ok, "Trash items should succeed in teardown");
        }
    }

    let _ = std::fs::remove_file(TEST_IMAGE_PATH);
}
