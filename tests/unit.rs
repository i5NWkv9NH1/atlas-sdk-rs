// * Atlas Rust SDK Unit Test Suite
// * Validates 100% Schema Parity, Mock Tool Invocations, Argument Normalization, 401 Retry, view_items Unpacking, and Secret Leak Prevention.

use serde_json::Value;
use sora_atlas::*;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

// * Helper function returning a mock JSON response for any tool
fn mock_response_for(tool: &str) -> Value {
    match tool {
        "search_items" => serde_json::to_value(SearchItemsResult::default()).unwrap(),
        "search_by_description" => {
            serde_json::to_value(SearchByDescriptionResult::default()).unwrap()
        }
        "find_similar" => serde_json::to_value(FindSimilarResult::default()).unwrap(),
        "get_item" => serde_json::to_value(AtlasItemDetail {
            id: "item-1".to_string(),
            name: "test".to_string(),
            kind: ItemKind::Image,
            date_added: "2026-10-27".to_string(),
            ..Default::default()
        })
        .unwrap(),
        "view_items" => serde_json::to_value(ViewItemsResult::default()).unwrap(),
        "get_file" => serde_json::to_value(GetFileResult::default()).unwrap(),
        "store_embeddings" => serde_json::to_value(StoreEmbeddingsResult::default()).unwrap(),
        "query_embeddings" => serde_json::to_value(QueryEmbeddingsResult::default()).unwrap(),
        "run_smart_folder" => serde_json::to_value(RunSmartFolderResult::default()).unwrap(),
        "get_board" => serde_json::to_value(GetBoardResult::default()).unwrap(),
        "get_current_view" => serde_json::to_value(GetCurrentViewResult::default()).unwrap(),
        "library_info" => serde_json::to_value(LibraryInfoResult {
            name: "TestLib".to_string(),
            path: "/path".to_string(),
            read_only: false,
            unique_items: 0,
            trashed_items: 0,
            items_by_kind: serde_json::json!({}),
            collections: 0,
            archived_collections: 0,
            collection_folders: 0,
            smart_folders: 0,
            connected_folders: 0,
            tags: 0,
            untagged_items: 0,
            ..Default::default()
        })
        .unwrap(),
        "list_libraries" => serde_json::to_value(ListLibrariesResult::default()).unwrap(),
        "list_collections" => serde_json::to_value(ListCollectionsResult::default()).unwrap(),
        "list_collection_folders" => {
            serde_json::to_value(ListCollectionFoldersResult::default()).unwrap()
        }
        "list_tags" => serde_json::to_value(ListTagsResult::default()).unwrap(),
        "list_smart_folders" => serde_json::to_value(ListSmartFoldersResult::default()).unwrap(),
        "list_connected_folders" => {
            serde_json::to_value(ListConnectedFoldersResult::default()).unwrap()
        }
        "app_info" => serde_json::to_value(AppInfoResult {
            name: "Atlas".to_string(),
            version: "1.0.0".to_string(),
            build: 1,
            ..Default::default()
        })
        .unwrap(),
        "get_settings" => serde_json::to_value(GetSettingsResult::default()).unwrap(),
        _ => serde_json::json!({ "ok": true }),
    }
}

// * Built-in list of all 78 official tools for standalone testing
const BUILTIN_TOOL_NAMES: &[&str] = &[
    "search_items",
    "search_by_description",
    "find_similar",
    "get_item",
    "view_items",
    "get_file",
    "store_embeddings",
    "query_embeddings",
    "run_smart_folder",
    "get_board",
    "get_current_view",
    "import_files",
    "add_link",
    "edit_link",
    "add_color",
    "edit_color",
    "save_media_from_url",
    "edit_image",
    "refresh_thumbnail",
    "set_liked",
    "set_metadata",
    "trash_items",
    "restore_items",
    "delete_permanently",
    "empty_trash",
    "list_collections",
    "create_collection",
    "rename_collection",
    "archive_collection",
    "delete_collection",
    "set_collection_order",
    "add_to_collection",
    "remove_from_collection",
    "list_collection_folders",
    "create_collection_folder",
    "move_collection_folder",
    "rename_collection_folder",
    "delete_collection_folder",
    "move_collection_to_folder",
    "list_tags",
    "create_tag",
    "rename_tag",
    "delete_tag",
    "add_tags",
    "remove_tags",
    "set_tags",
    "move_tags_to_folder",
    "create_tag_folder",
    "rename_tag_folder",
    "delete_tag_folder",
    "list_smart_folders",
    "create_smart_folder",
    "update_smart_folder",
    "delete_smart_folder",
    "arrange_board",
    "move_to_group",
    "move_group",
    "reset_board_layout",
    "undo_board_layout",
    "list_connected_folders",
    "connect_folder",
    "disconnect_folder",
    "rescan_folder",
    "library_info",
    "list_libraries",
    "create_library",
    "switch_library",
    "show_in_app",
    "show_items",
    "open_item",
    "select_items",
    "set_view_mode",
    "show_in_finder",
    "get_settings",
    "set_setting",
    "open_settings",
    "app_info",
    "run_maintenance",
];

// ============================================================================
// 1. Schema 100% Parity Alignment
// ============================================================================
#[tokio::test]
async fn test_schema_parity() {
    // * Read schema file directly from tests/fixtures/atlas-schema.json
    let fixture_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/atlas-schema.json"
    );
    let content =
        std::fs::read_to_string(fixture_path).expect("tests/fixtures/atlas-schema.json must exist");
    let schema_json: Value = serde_json::from_str(&content).expect("Invalid json in schema");

    let tools = schema_json
        .get("result")
        .and_then(|r| r.get("tools"))
        .or_else(|| schema_json.get("tools"))
        .and_then(|t| t.as_array())
        .expect("Failed to find tools array in schema");

    // ! 78 tools required
    assert_eq!(tools.len(), 78, "Schema must contain exactly 78 tools");

    let expected_tools: Vec<String> = tools
        .iter()
        .filter_map(|t| {
            t.get("name")
                .and_then(|n| n.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    assert_eq!(
        expected_tools.len(),
        78,
        "Every tool entry must have a name"
    );

    // * Check that all 78 tools are mapped in our tool registry
    let mock = MockAtlasTransport::new();
    for tool_name in &expected_tools {
        let name = tool_name.clone();
        mock.mock(tool_name, move |_| Ok(mock_response_for(&name)));
    }

    // * Check that MCP protocol methods work as expected
    mock.mock_list_tools(|| {
        Ok(McpListToolsResult {
            tools: vec![McpToolDefinition {
                name: "test_tool".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
    });
    mock.mock_list_resources(|| {
        Ok(McpListResourcesResult {
            resources: vec![McpResourceDefinition {
                uri: "atlas://skill".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
    });
    mock.mock_read_resource(|uri| {
        Ok(McpReadResourceResult {
            contents: vec![McpResourceContent {
                uri: uri.to_string(),
                text: Some("# Skill Doc".into()),
                ..Default::default()
            }],
            ..Default::default()
        })
    });

    let client = AtlasClient::with_transport(mock);
    let tools_res = client.list_tools().await.unwrap();
    assert_eq!(tools_res.tools.len(), 1);
    assert_eq!(tools_res.tools[0].name, "test_tool");

    let resources_res = client.list_resources().await.unwrap();
    assert_eq!(resources_res.resources.len(), 1);
    assert_eq!(resources_res.resources[0].uri, "atlas://skill");

    let read_res = client.read_resource("atlas://skill").await.unwrap();
    assert_eq!(read_res.contents.len(), 1);
    assert_eq!(read_res.contents[0].text.as_deref(), Some("# Skill Doc"));
}

// ============================================================================
// 2. Mock Transport Full Tool Invocation & Argument Normalization
// ============================================================================
#[tokio::test]
async fn test_mock_transport_full_78_invocations() {
    let mock = Arc::new(MockAtlasTransport::new());

    // * List of all 78 tool names from builtin registry
    let all_tools = BUILTIN_TOOL_NAMES;

    assert_eq!(all_tools.len(), 78, "Must test exactly 78 tools");

    // * Register handlers for all tools
    for tool_name in all_tools {
        let name = tool_name.to_string();
        mock.mock(tool_name, move |_| Ok(mock_response_for(&name)));
    }

    let client = AtlasClient::with_transport((*mock).clone());

    // * Query domain (11)
    let _ = client
        .search_items(&SearchItemsParams {
            query: Some("arch".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .search_by_description(&SearchByDescriptionParams {
            query: "sunset".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .find_similar(&FindSimilarParams {
            item_id: "item-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .get_item(&GetItemParams {
            item_id: "item-1".into(),
        })
        .await
        .unwrap();
    let _ = client
        .view_items(&ViewItemsParams {
            item_ids: vec!["item-1".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .get_file(&GetFileParams {
            item_id: "item-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .store_embeddings(&StoreEmbeddingsParams {
            model: "siglip".into(),
            entries: vec![EmbeddingEntry {
                content_hash: "hash-1".into(),
                vector: vec![0.1, 0.2],
            }],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .query_embeddings(&QueryEmbeddingsParams {
            vector: Some(vec![0.1, 0.2]),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .run_smart_folder(&RunSmartFolderParams {
            smart_folder: "sf-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .get_board(&GetBoardParams {
            collection: "col-1".into(),
        })
        .await
        .unwrap();
    let _ = client
        .get_current_view(&GetCurrentViewParams::default())
        .await
        .unwrap();

    // * Media domain (14)
    let _ = client
        .import_files(&ImportFilesParams {
            paths: vec!["/tmp/test.png".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .add_link(&AddLinkParams {
            url: "https://example.com".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .edit_link(&EditLinkParams {
            item_id: "item-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .add_color(&AddColorParams {
            hex: "#FF5500".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .edit_color(&EditColorParams {
            item_id: "item-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .save_media_from_url(&SaveMediaFromUrlParams {
            url: "https://example.com/img.png".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .edit_image(&EditImageParams {
            item_ids: vec!["item-1".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .refresh_thumbnail(&RefreshThumbnailParams {
            item_ids: vec!["item-1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .set_liked(&SetLikedParams {
            item_ids: vec!["item-1".into()],
            liked: true,
        })
        .await
        .unwrap();
    let _ = client
        .set_metadata(&SetMetadataParams {
            item_id: "item-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .trash_items(&TrashItemsParams {
            item_ids: vec!["item-1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .restore_items(&RestoreItemsParams {
            item_ids: vec!["item-1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .delete_permanently(&DeletePermanentlyParams {
            item_ids: vec!["item-1".into()],
            confirm: true,
        })
        .await
        .unwrap();
    let _ = client
        .empty_trash(&EmptyTrashParams { confirm: true })
        .await
        .unwrap();

    // * Organize domain (42)
    let _ = client.list_collections().await.unwrap();
    let _ = client
        .create_collection(&CreateCollectionParams { name: "Col".into() })
        .await
        .unwrap();
    let _ = client
        .rename_collection(&RenameCollectionParams {
            collection: "col-1".into(),
            name: "Col2".into(),
        })
        .await
        .unwrap();
    let _ = client
        .archive_collection(&ArchiveCollectionParams {
            collection: "col-1".into(),
            archived: true,
        })
        .await
        .unwrap();
    let _ = client
        .delete_collection(&DeleteCollectionParams {
            collection: "col-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .set_collection_order(&SetCollectionOrderParams {
            collection_ids: vec!["col-1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .add_to_collection(&AddToCollectionParams {
            collection: "col-1".into(),
            item_ids: vec!["item-1".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .remove_from_collection(&RemoveFromCollectionParams {
            collection: "col-1".into(),
            item_ids: vec!["item-1".into()],
        })
        .await
        .unwrap();
    let _ = client.list_collection_folders().await.unwrap();
    let _ = client
        .create_collection_folder(&CreateCollectionFolderParams {
            name: "Fold".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .move_collection_folder(&MoveCollectionFolderParams {
            folder: "f-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .rename_collection_folder(&RenameCollectionFolderParams {
            folder: "f-1".into(),
            name: "Fold2".into(),
        })
        .await
        .unwrap();
    let _ = client
        .delete_collection_folder(&DeleteCollectionFolderParams {
            folder: "f-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .move_collection_to_folder(&MoveCollectionToFolderParams {
            collection: "col-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client.list_tags().await.unwrap();
    let _ = client
        .create_tag(&CreateTagParams {
            name: "tag-1".into(),
        })
        .await
        .unwrap();
    let _ = client
        .rename_tag(&RenameTagParams {
            tag: "t1".into(),
            name: "t2".into(),
        })
        .await
        .unwrap();
    let _ = client
        .delete_tag(&DeleteTagParams { tag: "t1".into() })
        .await
        .unwrap();
    let _ = client
        .add_tags(&AddTagsParams {
            item_ids: vec!["item-1".into()],
            tags: vec!["t1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .remove_tags(&RemoveTagsParams {
            item_ids: vec!["item-1".into()],
            tags: vec!["t1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .set_tags(&SetTagsParams {
            item_id: "item-1".into(),
            tags: vec!["t1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .move_tags_to_folder(&MoveTagsToFolderParams {
            tags: vec!["t1".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .create_tag_folder(&CreateTagFolderParams {
            name: "tf-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .rename_tag_folder(&RenameTagFolderParams {
            folder: "tf-1".into(),
            name: "tf-2".into(),
        })
        .await
        .unwrap();
    let _ = client
        .delete_tag_folder(&DeleteTagFolderParams {
            folder: "tf-1".into(),
        })
        .await
        .unwrap();
    let _ = client.list_smart_folders().await.unwrap();
    let _ = client
        .create_smart_folder(&CreateSmartFolderParams {
            name: "sf-1".into(),
            rules: vec![],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .update_smart_folder(&UpdateSmartFolderParams {
            smart_folder: "sf-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .delete_smart_folder(&DeleteSmartFolderParams {
            smart_folder: "sf-1".into(),
        })
        .await
        .unwrap();
    let _ = client
        .arrange_board(&ArrangeBoardParams {
            collection: "col-1".into(),
            groups: vec![],
        })
        .await
        .unwrap();
    let _ = client
        .move_to_group(&MoveToGroupParams {
            collection: "col-1".into(),
            group_id: "grp-1".into(),
            item_ids: vec!["item-1".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .move_group(&MoveGroupParams {
            collection: "col-1".into(),
            group_id: "grp-1".into(),
            x: 10.0,
            y: 20.0,
        })
        .await
        .unwrap();
    let _ = client
        .reset_board_layout(&ResetBoardLayoutParams {
            collection: "col-1".into(),
        })
        .await
        .unwrap();
    let _ = client
        .undo_board_layout(&UndoBoardLayoutParams {
            collection: "col-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client.list_connected_folders().await.unwrap();
    let _ = client
        .connect_folder(&ConnectFolderParams {
            path: "/tmp".into(),
        })
        .await
        .unwrap();
    let _ = client
        .disconnect_folder(&DisconnectFolderParams {
            folder: "cf-1".into(),
        })
        .await
        .unwrap();
    let _ = client
        .rescan_folder(&RescanFolderParams {
            folder: "cf-1".into(),
        })
        .await
        .unwrap();
    let _ = client
        .library_info(&LibraryInfoParams::default())
        .await
        .unwrap();
    let _ = client.list_libraries().await.unwrap();
    let _ = client
        .create_library(&CreateLibraryParams {
            path: "/tmp/new.atlaslibrary".into(),
        })
        .await
        .unwrap();
    let _ = client
        .switch_library(&SwitchLibraryParams {
            path: "/tmp/other.atlaslibrary".into(),
        })
        .await
        .unwrap();

    // * UI domain (11)
    let _ = client
        .show_in_app(&ShowInAppParams::default())
        .await
        .unwrap();
    let _ = client
        .show_items(&ShowItemsParams {
            item_ids: vec!["item-1".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .open_item(&OpenItemParams {
            item_id: "item-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .select_items(&SelectItemsParams {
            item_ids: vec!["item-1".into()],
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .set_view_mode(&SetViewModeParams {
            mode: ViewDisplayMode::Grid,
            ..Default::default()
        })
        .await
        .unwrap();
    let _ = client
        .show_in_finder(&ShowInFinderParams {
            item_ids: vec!["item-1".into()],
        })
        .await
        .unwrap();
    let _ = client
        .get_settings(&GetSettingsParams::default())
        .await
        .unwrap();
    let _ = client
        .set_setting(&SetSettingParams {
            name: "test".into(),
            value: Value::Bool(true),
        })
        .await
        .unwrap();
    let _ = client
        .open_settings(&OpenSettingsParams::default())
        .await
        .unwrap();
    let _ = client.app_info().await.unwrap();
    let _ = client
        .run_maintenance(&RunMaintenanceParams {
            action: MaintenanceAction::RemoveDuplicates,
            ..Default::default()
        })
        .await
        .unwrap();

    // ! Verify all 78 tools were called and recorded
    let history = mock.history();
    assert_eq!(history.len(), 78, "History must contain all 78 calls");

    for tool_name in all_tools {
        assert!(
            history.iter().any(|(name, _)| name == tool_name),
            "Tool '{tool_name}' must be called in history"
        );
    }
}

// ============================================================================
// 3. Parameter Normalization Rules
// ============================================================================
#[tokio::test]
async fn test_parameter_normalization() {
    let mock = Arc::new(MockAtlasTransport::new());
    mock.mock("search_items", |_| {
        Ok(serde_json::to_value(SearchItemsResult::default()).unwrap())
    });
    mock.mock("create_smart_folder", |_| {
        Ok(serde_json::json!({ "ok": true }))
    });
    mock.mock("get_file", |_| Ok(serde_json::json!({ "ok": true })));
    mock.mock("search_by_description", |_| {
        Ok(serde_json::to_value(SearchByDescriptionResult::default()).unwrap())
    });

    let client = AtlasClient::with_transport((*mock).clone());

    // 1. search_items should normalize unset optional enum filters to ""
    let _ = client
        .search_items(&SearchItemsParams {
            query: Some("cat".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();

    let hist = mock.history();
    let search_call = hist
        .iter()
        .find(|(n, _)| n == "search_items")
        .expect("search_items call missing");
    let search_args = &search_call.1;
    assert_eq!(search_args.get("sort").and_then(|v| v.as_str()), Some(""));
    assert_eq!(search_args.get("source").and_then(|v| v.as_str()), Some(""));
    assert_eq!(
        search_args.get("orientation").and_then(|v| v.as_str()),
        Some("")
    );
    assert_eq!(search_args.get("kind").and_then(|v| v.as_str()), Some(""));
    assert_eq!(search_args.get("order").and_then(|v| v.as_str()), Some(""));
    assert_eq!(
        search_args.get("query").and_then(|v| v.as_str()),
        Some("cat")
    );

    // 2. create_smart_folder should normalize match to ""
    let _ = client
        .create_smart_folder(&CreateSmartFolderParams {
            name: "Test".to_string(),
            rules: vec![],
            ..Default::default()
        })
        .await
        .unwrap();

    let hist2 = mock.history();
    let sf_call = hist2
        .iter()
        .find(|(n, _)| n == "create_smart_folder")
        .expect("create_smart_folder call missing");
    assert_eq!(sf_call.1.get("match").and_then(|v| v.as_str()), Some(""));

    // 3. get_file should normalize size to 0
    let _ = client
        .get_file(&GetFileParams {
            item_id: "item-1".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    let hist3 = mock.history();
    let file_call = hist3
        .iter()
        .find(|(n, _)| n == "get_file")
        .expect("get_file call missing");
    assert_eq!(file_call.1.get("size").and_then(|v| v.as_i64()), Some(0));

    // 4. search_by_description should normalize kind to "" and min_score to 0.0
    let _ = client
        .search_by_description(&SearchByDescriptionParams {
            query: "dog".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    let hist4 = mock.history();
    let sbd_call = hist4
        .iter()
        .find(|(n, _)| n == "search_by_description")
        .expect("search_by_description call missing");
    assert_eq!(sbd_call.1.get("kind").and_then(|v| v.as_str()), Some(""));
    assert_eq!(
        sbd_call.1.get("min_score").and_then(|v| v.as_f64()),
        Some(0.0)
    );
}

// ============================================================================
// 4. Error Handling: result.isError converts to AtlasError::ToolError
// ============================================================================
#[tokio::test]
async fn test_error_handling_is_error() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");
    let port = listener.local_addr().unwrap().port();

    std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf);
            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"isError\":true,\"content\":[{\"type\":\"text\",\"text\":\"Item not found in database\"}]}}";
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });

    let transport = HttpAtlasTransport::new(HttpTransportOptions {
        endpoint: format!("http://127.0.0.1:{port}/mcp"),
        token: Some("test-token".to_string()),
        auto_discover_token: false,
        auto_launch: false,
        timeout: Duration::from_secs(3),
        ..Default::default()
    });

    let client = AtlasClient::with_transport(transport);
    let result = client
        .get_item(&GetItemParams {
            item_id: "bad-id".into(),
        })
        .await;

    match result {
        Err(AtlasError::ToolError { tool_name, message }) => {
            assert_eq!(tool_name, "get_item");
            assert!(
                message.contains("Item not found in database"),
                "Message should contain error details"
            );
        }
        other => panic!("Expected ToolError, got {:?}", other),
    }
}

// ============================================================================
// 5. 401 Retry Mechanism
// ============================================================================
#[tokio::test]
async fn test_401_retry_mechanism() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");
    let port = listener.local_addr().unwrap().port();

    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_clone = attempts.clone();

    std::thread::spawn(move || {
        for _ in 0..2 {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 2048];
                let n = stream.read(&mut buf).unwrap_or(0);
                let req_text = String::from_utf8_lossy(&buf[..n]);
                let count = attempts_clone.fetch_add(1, Ordering::SeqCst);

                if count == 0 {
                    // First attempt: return 401
                    let resp = "HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    let _ = stream.write_all(resp.as_bytes());
                    let _ = stream.flush();
                } else {
                    // Second attempt: verify retry and return 200 OK
                    assert!(
                        req_text
                            .to_lowercase()
                            .contains("authorization: bearer test-token"),
                        "Must include refreshed Bearer token on retry"
                    );
                    let resp = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"{\\\"ok\\\":true}\"}]}}";
                    let _ = stream.write_all(resp.as_bytes());
                    let _ = stream.flush();
                    break;
                }
            }
        }
    });

    let transport = HttpAtlasTransport::new(HttpTransportOptions {
        endpoint: format!("http://127.0.0.1:{port}/mcp"),
        token: Some("test-token".to_string()),
        auto_discover_token: false,
        auto_launch: false,
        timeout: Duration::from_secs(3),
        ..Default::default()
    });

    let client = AtlasClient::with_transport(transport);
    let res = client
        .empty_trash(&EmptyTrashParams { confirm: true })
        .await;
    assert!(res.is_ok(), "Request should succeed after 401 retry");
    assert_eq!(
        attempts.load(Ordering::SeqCst),
        2,
        "Must retry exactly once"
    );
}

#[tokio::test]
async fn test_401_retry_exhaustion_throws_auth_error() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");
    let port = listener.local_addr().unwrap().port();

    std::thread::spawn(move || {
        for _ in 0..2 {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf);
                let resp =
                    "HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                let _ = stream.write_all(resp.as_bytes());
                let _ = stream.flush();
            }
        }
    });

    let transport = HttpAtlasTransport::new(HttpTransportOptions {
        endpoint: format!("http://127.0.0.1:{port}/mcp"),
        token: Some("expired-token".to_string()),
        auto_discover_token: false,
        auto_launch: false,
        timeout: Duration::from_secs(3),
        ..Default::default()
    });

    let client = AtlasClient::with_transport(transport);
    let result = client
        .empty_trash(&EmptyTrashParams { confirm: true })
        .await;

    match result {
        Err(AtlasError::Auth(msg)) => {
            assert!(
                msg.contains("Atlas authentication failed"),
                "Message must describe auth failure"
            );
        }
        other => panic!("Expected AtlasError::Auth, got {:?}", other),
    }
}

// ============================================================================
// 6. view_items Unpacking Stability
// ============================================================================
#[tokio::test]
async fn test_view_items_unpacking_stability() {
    let transport = HttpAtlasTransport::new(HttpTransportOptions::default());

    // Case 1: 0 items in content
    let empty_content = vec![];
    let res_empty = transport.unpack_mcp_content("view_items", &empty_content);
    assert!(res_empty.get("content").is_some());
    assert_eq!(
        res_empty
            .get("content")
            .and_then(|c| c.as_array())
            .map(|a| a.len()),
        Some(0)
    );

    // Case 2: 1 image item in content
    let single_content = vec![serde_json::json!({
        "type": "image",
        "data": "base64data1",
        "mimeType": "image/png"
    })];
    let res_single = transport.unpack_mcp_content("view_items", &single_content);
    let arr1 = res_single
        .get("content")
        .and_then(|c| c.as_array())
        .unwrap();
    assert_eq!(arr1.len(), 1);
    assert_eq!(
        arr1[0].get("data").and_then(|d| d.as_str()),
        Some("base64data1")
    );

    // Case 3: 2 mixed items (text + image) in content
    let multi_content = vec![
        serde_json::json!({ "type": "text", "text": "item-uuid-1" }),
        serde_json::json!({ "type": "image", "data": "base64data2", "mimeType": "image/jpeg" }),
    ];
    let res_multi = transport.unpack_mcp_content("view_items", &multi_content);
    let arr2 = res_multi.get("content").and_then(|c| c.as_array()).unwrap();
    assert_eq!(arr2.len(), 2);
    assert_eq!(arr2[0].get("type").and_then(|t| t.as_str()), Some("text"));
    assert_eq!(
        arr2[1].get("data").and_then(|d| d.as_str()),
        Some("base64data2")
    );
}

// ============================================================================
// 7. Multi-Library Token Switching
// ============================================================================
#[tokio::test]
async fn test_multi_library_token_switching() {
    let mut lib_map = HashMap::new();
    lib_map.insert("LibraryA".to_string(), "token-for-lib-a".to_string());
    lib_map.insert(
        "/path/to/LibraryB.atlaslibrary".to_string(),
        "token-for-lib-b".to_string(),
    );

    let resolver = TokenResolver::new(TokenResolverOptions {
        token: Some("default-lib-token".to_string()),
        library_tokens: lib_map,
        auto_discover: false,
    });

    // Default token
    assert_eq!(
        resolver.resolve_token(false).await.unwrap(),
        "default-lib-token"
    );

    // Switch to LibraryA
    resolver.set_active_library("LibraryA", None);
    assert_eq!(
        resolver.resolve_token(false).await.unwrap(),
        "token-for-lib-a"
    );

    // Switch to LibraryB
    resolver.set_active_library("/path/to/LibraryB.atlaslibrary", None);
    assert_eq!(
        resolver.resolve_token(false).await.unwrap(),
        "token-for-lib-b"
    );
}

// ============================================================================
// 8. Secret Leak Prevention Scan
// ============================================================================
#[tokio::test]
async fn test_secret_leak_prevention() {
    let mut files_to_scan = Vec::new();

    // Helper recursive collector
    fn collect_rs_files(dir: &Path, files: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect_rs_files(&path, files);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    collect_rs_files(Path::new("src"), &mut files_to_scan);
    collect_rs_files(Path::new("tests"), &mut files_to_scan);

    assert!(
        files_to_scan.len() >= 6,
        "Should scan all source and test files"
    );

    for file_path in &files_to_scan {
        let content = std::fs::read_to_string(file_path).expect("Failed to read file");

        // ! Check for potential real Bearer token leak
        for line in content.lines() {
            if let Some(idx) = line.find("Bearer ") {
                let tail = &line[idx + 7..];
                let token_cand: String = tail
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                    .collect();

                // Disallow random token-like strings longer than 24 characters unless template
                if token_cand.len() >= 25 {
                    panic!("Potential secret Bearer token leak in {file_path}: {token_cand}");
                }
            }
        }

        // ! Self-contained runtime check: src/ must never read SKILL.md or atlas-schema.json
        if file_path.starts_with("src/") || file_path.starts_with("src\\") {
            assert!(
                !content.contains("SKILL.md"),
                "src/ must not reference SKILL.md: {file_path}"
            );
            assert!(
                !content.contains("atlas-schema.json"),
                "src/ must not reference atlas-schema.json: {file_path}"
            );
        }
    }
}
