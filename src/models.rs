//! Strongly-typed models, enumerations, parameters, and results for the Atlas MCP API.
//!
//! Generated to match official Atlas MCP tool schema with zero runtime overhead.

use serde::{Deserialize, Serialize};

// ============================================================================
// 1. Fundamental Enumerations
// ============================================================================

/// Supported library item kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ItemKind {
    #[default]
    Image,
    Video,
    Gif,
    Link,
    Color,
}

/// Item kinds supported by semantic description search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchDescriptionKind {
    #[default]
    Image,
    Video,
    Gif,
    Link,
}

/// Sort fields for library queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ItemSortField {
    #[default]
    DateAdded,
    DateCreated,
    Name,
    Bytes,
    Duration,
    Width,
    Height,
    Random,
}

/// Sort ordering direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}

/// Supported social network sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SocialSource {
    #[default]
    Instagram,
    X,
}

/// Media aspect ratio orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MediaOrientation {
    #[default]
    Portrait,
    Landscape,
    Square,
}

/// Image rotation direction (90-degree steps).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RotateDirection {
    #[default]
    Left,
    Right,
}

/// Smart folder match mode: match all rules or any rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SmartFolderMatch {
    #[default]
    All,
    Any,
}

/// Target field for smart folder rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SmartFolderRuleField {
    #[default]
    Name,
    Type,
    Format,
    FileSize,
    Width,
    Height,
    Orientation,
    Duration,
    DateAdded,
    DateCreated,
    Tag,
    Url,
    Liked,
    Collection,
    SourceFolder,
}

/// Comparison operator for smart folder rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SmartFolderRuleOp {
    #[default]
    Is,
    IsNot,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    Equals,
    Before,
    After,
    InLast,
    HasAny,
    HasNone,
    IsIn,
    IsNotIn,
}

/// Numeric units for smart folder rule thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SmartFolderUnit {
    #[default]
    KB,
    MB,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "months")]
    Months,
}

/// Built-in application preset views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BuiltinAppView {
    #[default]
    All,
    Links,
    Colors,
    Instagram,
    X,
    Inbox,
    Likes,
    Trash,
}

/// Item list display modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ViewDisplayMode {
    #[default]
    Grid,
    Infinite,
    Canvas,
}

/// Application settings section identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SettingsSection {
    #[default]
    General,
    Appearance,
    Behavior,
    Services,
    Sidebar,
    Updates,
    Mcp,
    Debug,
}

/// Settings window tab pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SettingsPage {
    #[default]
    General,
    Appearance,
    Shortcuts,
    Collections,
    Importing,
    Media,
    Galleries,
    Services,
    Extensions,
    Mcp,
    Updates,
    License,
    About,
    Experimental,
    Debug,
    Library,
    Developers,
}

/// Library maintenance action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceAction {
    #[default]
    RemoveDuplicates,
    RemoveMissingOriginals,
    PurgeExpiredTrash,
    RepairThumbnails,
}

// ============================================================================
// 2. Common Domain Entities
// ============================================================================

/// Normalized crop rectangle (0 to 1 range, top-left origin).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct CropRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Single smart folder rule criteria.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartFolderRule {
    pub field: SmartFolderRuleField,
    pub op: SmartFolderRuleOp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<SmartFolderUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "refID")]
    pub ref_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Complete smart folder definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartFolderDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "match")]
    pub match_mode: SmartFolderMatch,
    pub expect: bool,
    pub rules: Vec<SmartFolderRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
}

/// Canvas group layout item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BoardGroupInput {
    pub item_ids: Vec<String>,
    pub x: f64,
    pub y: f64,
}

/// Vector embedding entry keyed by content_hash.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmbeddingEntry {
    pub content_hash: String,
    pub vector: Vec<f64>,
}

/// Standard generic operation response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SimpleOkResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Summary metadata for a library item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasItemSummary {
    pub id: String,
    pub name: String,
    pub kind: ItemKind,
    pub date_added: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
}

/// Collection reference tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasItemCollectionRef {
    pub id: String,
    pub name: String,
}

/// Multi-page album page item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasItemPage {
    pub page: i64,
    pub path: String,
    pub width: f64,
    pub height: f64,
}

/// Comprehensive item details returned by get_item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasItemDetail {
    pub id: String,
    pub name: String,
    pub kind: ItemKind,
    pub date_added: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<AtlasItemCollectionRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<AtlasItemPage>>,
}

/// Type alias for AtlasItemDetail.
pub type AtlasItem = AtlasItemDetail;

/// Content item block returned by view_items (text description or base64 image).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasImageItem {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// Known Atlas library entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasLibraryEntry {
    pub name: String,
    pub path: String,
    pub exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
}

/// Collection summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasCollection {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(alias = "count")]
    pub item_count: i64,
    #[serde(alias = "is_builtin", skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(alias = "folder_name", skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
}

/// Collection folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasCollectionFolder {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<AtlasCollection>>,
}

/// Tag information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasTag {
    #[serde(alias = "tag", default)]
    pub name: String,
    #[serde(default)]
    pub count: i64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub folder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub folder_id: Option<String>,
}

/// Tag folder information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasTagFolder {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
}

/// Smart folder summary entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasSmartFolderEntry {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<serde_json::Value>,
}

/// Connected filesystem folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AtlasConnectedFolder {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(alias = "is_mounted")]
    pub mounted: bool,
    #[serde(alias = "count")]
    pub item_count: i64,
}

/// Board spatial group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AtlasBoardGroup {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub items: Vec<AtlasItemSummary>,
}

/// Similar item result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SimilarItemResult {
    pub id: String,
    pub distance: f64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Semantic search result item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SemanticSearchResultItem {
    pub id: String,
    pub score: f64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Application setting item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AtlasSettingItem {
    #[serde(alias = "key", default)]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

// ============================================================================
// 3. Parameters and Results for 78 MCP Tools
// ============================================================================

/// Parameters for the `library_info` tool.
/// Which library is open and its totals: name, path, read-only state, unique item counts by kind, trash size, tag / collection / smart folder / connected folder counts, how much of the library the visual-similarity index covers. Pass `include_bytes: true` to also total the originals on disk (walks every file — slow on big libraries).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LibraryInfoParams {
    /// Sum the size of every original (slow)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub include_bytes: Option<bool>,
}

/// Result returned by the `library_info` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LibraryInfoResult {
    pub name: String,
    pub path: String,
    pub read_only: bool,
    pub unique_items: i64,
    pub trashed_items: i64,
    pub items_by_kind: serde_json::Value,
    pub collections: i64,
    pub archived_collections: i64,
    pub collection_folders: i64,
    pub smart_folders: i64,
    pub connected_folders: i64,
    pub tags: i64,
    pub untagged_items: i64,
    pub liked_items: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `list_libraries` tool.
/// Every library Atlas knows on this Mac (the open one is marked). Use switch_library to open another.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ListLibrariesParams {}

/// Result returned by the `list_libraries` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ListLibrariesResult {
    pub libraries: Vec<AtlasLibraryEntry>,
}

/// Parameters for the `list_collections` tool.
/// List every collection (board) in the Atlas library: id, name, sidebar folder, item count. System collections (Inbox, Likes, Trash) are marked. Per-collection counts are PLACEMENTS, not unique items — one item can live in several collections, and links / connected-folder items live outside collections entirely — so never sum them. The `library` object carries the exact unique totals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ListCollectionsParams {}

/// Result returned by the `list_collections` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListCollectionsResult {
    pub collections: Vec<AtlasCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `list_collection_folders` tool.
/// Sidebar folders that group collections: id, name, the collections filed directly inside each, and — since folders nest — `parent_id` (null at the top level) and `path` ("Parent ▸ Child").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ListCollectionFoldersParams {}

/// Result returned by the `list_collection_folders` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListCollectionFoldersResult {
    #[serde(alias = "folders")]
    pub collection_folders: Vec<AtlasCollectionFolder>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `list_tags` tool.
/// List every tag in the library with how many items carry it. A tag filed in a sidebar tag folder carries `folder` (name) and `folder_id`; `tag_folders` lists the folders themselves with their member tags. Loose tags have no `folder`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ListTagsParams {}

/// Result returned by the `list_tags` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListTagsResult {
    pub tags: Vec<AtlasTag>,
    #[serde(alias = "folders")]
    pub tag_folders: Vec<AtlasTagFolder>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `list_smart_folders` tool.
/// List saved smart folders (rule-based live searches): id, name, match count, and their rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ListSmartFoldersParams {}

/// Result returned by the `list_smart_folders` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListSmartFoldersResult {
    pub smart_folders: Vec<AtlasSmartFolderEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `list_connected_folders` tool.
/// Folders on disk that Atlas watches (the sidebar's Folders section): id, name, path, whether it's mounted on this Mac, and how many library items come from it. The iPhone folder (iCloud) is included. Items from connected folders carry `connected_folder_id`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ListConnectedFoldersParams {}

/// Result returned by the `list_connected_folders` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListConnectedFoldersResult {
    pub connected_folders: Vec<AtlasConnectedFolder>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `search_items` tool.
/// Search library items. All filters are optional and combine with AND. With NO filters, `total_matches` is the exact count of all non-trashed items in the library — use that for "how many items" questions. `query` matches name, link title/description, tags, annotation, author, location, and source URL, case-insensitively — it reads METADATA only, so for "find pictures of X" use search_by_description, which matches what items actually show. Scope with `collection`, `collection_folder`, `smart_folder`, `connected_folder`, or `source` (instagram / x). Filter by kind, tags (all / any), format, size, dimensions, duration, orientation, liked, untagged, date ranges (ISO 8601). Sort by date_added (default), date_created, name, bytes, duration, width, height, or random. Trashed items are excluded unless you search `collection: "Trash"`. For library-wide totals (how many images / videos / items) library_info answers in one cheap call. Enum filters accept "" for "no filter"; 0 on an optional number also means "not set". Returns compact metadata; `verbose: true` adds paths, notes and file facts; use view_items to actually see items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchItemsParams {
    /// Smart folder id or name — only its matches
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smart_folder: Option<String>,
    /// Collection id or name ("Trash" searches trashed items)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
    /// The max_bytes parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_bytes: Option<i64>,
    /// ISO 8601 — the media's own capture/creation date
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub created_after: Option<String>,
    /// Items must carry every listed tag
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
    /// The min_width parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_width: Option<i64>,
    /// Seconds, videos only
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_duration: Option<i64>,
    /// ISO 8601 date/time
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub added_after: Option<String>,
    /// The min_height parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_height: Option<i64>,
    /// The max_duration parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_duration: Option<i64>,
    /// Only items added in the last N days
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub added_within_days: Option<i64>,
    /// Author field contains (case-insensitive)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<String>,
    /// Default date_added. Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sort: Option<ItemSortField>,
    /// Items saved from that network. Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<SocialSource>,
    /// ISO 8601 date/time
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub added_before: Option<String>,
    /// Free-text filter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub query: Option<String>,
    /// The max_width parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_width: Option<i64>,
    /// The has_annotation parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub has_annotation: Option<bool>,
    /// Collection folder id or name — every collection inside it
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection_folder: Option<String>,
    /// Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub orientation: Option<MediaOrientation>,
    /// The max_height parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_height: Option<i64>,
    /// Connected folder id or name — only items from it
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub connected_folder: Option<String>,
    /// Only items with no tags
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub untagged: Option<bool>,
    /// Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kind: Option<ItemKind>,
    /// Max results, default 50, max 200
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<i64>,
    /// The created_before parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub created_before: Option<String>,
    /// Items must carry at least one listed tag
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub any_tags: Option<Vec<String>>,
    /// File extension, e.g. png, jpg, mp4, heic
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub format: Option<String>,
    /// The min_bytes parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_bytes: Option<i64>,
    /// Default desc (newest / largest first). Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub order: Option<SortOrder>,
    /// The offset parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub offset: Option<i64>,
    /// The liked parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub liked: Option<bool>,
    /// Full metadata per item (paths, notes, file facts)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub verbose: Option<bool>,
}

/// Result returned by the `search_items` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchItemsResult {
    pub items: Vec<AtlasItemSummary>,
    #[serde(alias = "total")]
    pub total_matches: i64,
    pub offset: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `run_smart_folder` tool.
/// The items a saved smart folder currently matches (newest first). Paged like search_items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RunSmartFolderParams {
    /// The offset parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub offset: Option<i64>,
    /// Default 50, max 200
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<i64>,
    /// Smart folder id or name
    pub smart_folder: String,
}

/// Result returned by the `run_smart_folder` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RunSmartFolderResult {
    pub items: Vec<AtlasItemSummary>,
    #[serde(alias = "total")]
    pub total_matches: i64,
    pub offset: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_folder: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `get_item` tool.
/// Full metadata for one item by id, including which collections it belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetItemParams {
    /// The item_id parameter
    pub item_id: String,
}

/// Result returned by the `get_item` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetItemResult {
    #[serde(flatten)]
    pub item: AtlasItemDetail,
}

/// Parameters for the `view_items` tool.
/// Look at items: returns each item's thumbnail as an image (preceded by a text line naming the item id) so you can see the actual content. Up to 20 items per call. `size` caps the longest edge in pixels (default 512, max 1024) — keep it small when scanning many items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ViewItemsParams {
    /// Max pixel size of the longest edge (128–1024, default 512)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size: Option<i64>,
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `view_items` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ViewItemsResult {
    pub content: Vec<AtlasImageItem>,
}

/// Parameters for the `get_file` tool.
/// Read an item's original file. Images come back as an image block (downscaled to `size` px on the long edge, default 2048, max 4096 — pass `size: 0` for the untouched bytes, any format); videos and other non-image files return their path, size, and MIME type (read them with a file tool — the path is local to this Mac). `page` picks a carousel slide (0 = cover). The same bytes are available as the MCP resource atlas://item/{id}/original.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetFileParams {
    /// Max long-edge pixels for images; 0 = original bytes
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size: Option<i64>,
    /// The item_id parameter
    pub item_id: String,
    /// Carousel page, 0-based (default 0)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub page: Option<i64>,
}

/// Result returned by the `get_file` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GetFileResult {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<AtlasImageItem>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `get_board` tool.
/// A collection's canvas layout: its groups (spatial clusters) with positions and the ordered items inside each.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetBoardParams {
    /// Collection id or name
    pub collection: String,
}

/// Result returned by the `get_board` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GetBoardResult {
    #[serde(alias = "manual_layout")]
    pub arranged: bool,
    pub groups: Vec<AtlasBoardGroup>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `find_similar` tool.
/// Find items visually similar to a given item, using on-device image embeddings (Apple Vision feature prints). Purely visual — no text understanding. The index builds in the background after launch; results note how much of the library is indexed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FindSimilarParams {
    /// The item_id parameter
    pub item_id: String,
    /// Max results, default 20, max 50
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<i64>,
}

/// Result returned by the `find_similar` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FindSimilarResult {
    #[serde(alias = "items")]
    pub results: Vec<SimilarItemResult>,
    #[serde(alias = "similarity_index", default)]
    pub indexed_items: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `get_current_view` tool.
/// What the user is looking at right now: the collection (if any), the items on screen (ids, newest-first is not implied — this is display order), and which are selected. Use it to act on "these" / "the selected ones".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetCurrentViewParams {
    /// Max items to list, default 100, max 500
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<i64>,
}

/// Result returned by the `get_current_view` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GetCurrentViewResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<serde_json::Value>,
    pub mode: ViewDisplayMode,
    pub item_count: i64,
    pub items: Vec<AtlasItemSummary>,
    pub selected_item_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `add_link` tool.
/// Save a URL as a link card (no media download). Files it into the given collection, or just into the library.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AddLinkParams {
    /// Optional collection id or name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
    /// The url parameter
    pub url: String,
}

/// Result returned by the `add_link` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AddLinkResult {
    pub ok: bool,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub url: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `add_color` tool.
/// Save a HEX color as a color card (e.g. #FF6B35, also 3/4/8-digit forms).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AddColorParams {
    /// The hex parameter
    pub hex: String,
    /// Optional collection id or name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
}

/// Result returned by the `add_color` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AddColorResult {
    pub ok: bool,
    pub item_id: String,
    pub hex: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `save_media_from_url` tool.
/// Download the media behind a web URL and import it — a direct image/video URL, or a social post (X/Twitter, Instagram, Pinterest, …) whose photos and videos are fetched, every carousel slide included. Falls back to saving a link card for social posts with no downloadable media. Slow tool — it performs real downloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SaveMediaFromUrlParams {
    /// The url parameter
    pub url: String,
    /// Optional collection id or name; defaults to the Inbox
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
}

/// Result returned by the `save_media_from_url` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SaveMediaFromUrlResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_count: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `import_files` tool.
/// Import media files from absolute paths on this Mac into a collection (defaults to the Inbox). Files are copied into the library; originals are untouched. Returns how many were imported, skipped as duplicates (`skipped_duplicate`) or skipped as unsupported types, plus `failed` (path + reason) for files macOS couldn't read, such as AV1 or VP9 videos.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportFilesParams {
    /// Optional collection id or name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
    /// The paths parameter
    pub paths: Vec<String>,
    /// The y parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub y: Option<f64>,
    /// Optional canvas position (world coordinates) to place the imports at
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub x: Option<f64>,
}

/// Result returned by the `import_files` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ImportFilesResult {
    pub ok: bool,
    pub imported: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_duplicate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_unsupported: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `add_tags` tool.
/// Add one or more tags to a set of items (existing tags are kept; duplicates ignored). Batched: one library save regardless of item count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AddTagsParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
    /// The tags parameter
    pub tags: Vec<String>,
}

/// Result returned by the `add_tags` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AddTagsResult {
    pub ok: bool,
    pub items: i64,
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `remove_tags` tool.
/// Remove one or more tags from a set of items (case-insensitive).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoveTagsParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
    /// The tags parameter
    pub tags: Vec<String>,
}

/// Result returned by the `remove_tags` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RemoveTagsResult {
    pub ok: bool,
    pub items: i64,
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `set_tags` tool.
/// Replace an item's tags wholesale (an empty array clears them). Use add_tags / remove_tags to edit incrementally across many items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetTagsParams {
    /// The item_id parameter
    pub item_id: String,
    /// The tags parameter
    pub tags: Vec<String>,
}

/// Result returned by the `set_tags` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SetTagsResult {
    pub ok: bool,
    pub item_id: String,
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `set_metadata` tool.
/// Set an item's annotation (note), author, location, or source URL. Only the fields you pass change; pass an empty string to clear a field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetMetadataParams {
    /// The annotation parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub annotation: Option<String>,
    /// The location parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub location: Option<String>,
    /// The source_url parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source_url: Option<String>,
    /// The item_id parameter
    pub item_id: String,
    /// The author parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<String>,
}

/// Result returned by the `set_metadata` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SetMetadataResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `edit_link` tool.
/// Edit a link card's title, description, or URL. Only the fields you pass change; empty title/description clear the field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EditLinkParams {
    /// The description parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    /// The url parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    /// The title parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,
    /// The item_id parameter
    pub item_id: String,
}

/// Result returned by the `edit_link` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EditLinkResult {
    pub ok: bool,
    #[serde(alias = "id", default)]
    pub item_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `edit_color` tool.
/// Change a color card's HEX value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EditColorParams {
    /// The hex parameter
    pub hex: String,
    /// The item_id parameter
    pub item_id: String,
}

/// Result returned by the `edit_color` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EditColorResult {
    pub ok: bool,
    pub item_id: String,
    pub hex: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `add_to_collection` tool.
/// File items into a collection. By default items are added (kept everywhere they already are); pass move=true to move them out of the source collections instead.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AddToCollectionParams {
    /// Move instead of add (default false)
    #[serde(rename = "move")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#move: Option<bool>,
    /// The item_ids parameter
    pub item_ids: Vec<String>,
    /// Target collection id or name
    pub collection: String,
}

/// Result returned by the `add_to_collection` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AddToCollectionResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `remove_from_collection` tool.
/// Take items out of one collection without trashing them. Items in no other collection are re-filed into the Inbox so nothing is lost.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoveFromCollectionParams {
    /// Collection id or name
    pub collection: String,
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `remove_from_collection` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RemoveFromCollectionResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `set_liked` tool.
/// Like or unlike items (membership in the Likes collection).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetLikedParams {
    /// The liked parameter
    pub liked: bool,
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `set_liked` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SetLikedResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `edit_image` tool.
/// Crop or rotate images without touching the imported file (it is kept; `revert: true` puts it back). `rotate` turns the picture a quarter turn "left" or "right" on top of its current edit; `crop` replaces the kept region with a rectangle in the TURNED picture's unit square — x, y, width, height in 0…1, top-left origin — so {"x":0, "y":0, "width":1, "height":0.5} keeps the top half. Rotate and crop can be combined. Managed still images only: connected-folder files, vectors and animations are skipped. Returns each image's new pixel size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EditImageParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
    /// The kept region of the turned picture, in 0…1 units
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub crop: Option<CropRect>,
    /// A quarter turn, applied before the crop
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rotate: Option<RotateDirection>,
    /// Back to the imported picture (ignores rotate / crop)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub revert: Option<bool>,
}

/// Result returned by the `edit_image` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EditImageResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `refresh_thumbnail` tool.
/// Regenerate the thumbnails of items whose files changed on disk (edited in another app, or a connected-folder file replaced), so tiles and the open item show the current picture. Links and colors have no thumbnail to refresh and are skipped.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RefreshThumbnailParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `refresh_thumbnail` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RefreshThumbnailResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `trash_items` tool.
/// Move items to the Trash. Recoverable — restore_items puts them back; nothing is permanently deleted by this tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TrashItemsParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `trash_items` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TrashItemsResult {
    pub ok: bool,
    pub trashed: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `restore_items` tool.
/// Restore trashed items — puts them back into the collections they were trashed from (Put Back). Find trashed items with search_items collection:"Trash".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreItemsParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `restore_items` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RestoreItemsResult {
    pub ok: bool,
    pub restored: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `create_collection` tool.
/// Create a new collection (board). Returns its id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateCollectionParams {
    /// The name parameter
    pub name: String,
}

/// Result returned by the `create_collection` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateCollectionResult {
    pub ok: bool,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `rename_collection` tool.
/// Rename a collection (board). System collections (Inbox, Likes, Trash) can't be renamed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RenameCollectionParams {
    /// Collection id or name
    pub collection: String,
    /// The name parameter
    pub name: String,
}

/// Result returned by the `rename_collection` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RenameCollectionResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `archive_collection` tool.
/// Put a collection away (archived) or bring it back. Archived collections keep every item but leave the sidebar and the ⌘K palette; list_collections still shows them with `archived: true`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ArchiveCollectionParams {
    /// true = archive, false = unarchive
    pub archived: bool,
    /// Collection id or name
    pub collection: String,
}

/// Result returned by the `archive_collection` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ArchiveCollectionResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub archived: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `delete_collection` tool.
/// Delete a collection. By default its items are kept (re-filed into the Inbox if they're in no other collection). `delete_items: true` also removes items that live only in this collection — that is permanent and requires Settings ▸ Developers ▸ "Allow permanent deletion".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteCollectionParams {
    /// Collection id or name
    pub collection: String,
    /// Also delete items only found here (default false)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete_items: Option<bool>,
}

/// Result returned by the `delete_collection` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeleteCollectionResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_items: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `set_collection_order` tool.
/// Reorder the user's collections in the sidebar. Pass every collection id in the wanted order; ids you leave out keep their relative order after the listed ones.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetCollectionOrderParams {
    /// The collection_ids parameter
    pub collection_ids: Vec<String>,
}

/// Result returned by the `set_collection_order` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SetCollectionOrderResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `move_collection_to_folder` tool.
/// File a collection into a sidebar collection folder (nested folders included — pass a path like "Projekte/Clients" to pick one by its parents), or pull it back to the top level (omit `folder`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveCollectionToFolderParams {
    /// Collection id or name
    pub collection: String,
    /// Collection folder id, name or path ("Parent/Child"); omit for top level
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub folder: Option<String>,
}

/// Result returned by the `move_collection_to_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MoveCollectionToFolderResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `create_collection_folder` tool.
/// Create a sidebar folder that groups collections — at the top level, or nested inside another folder (`parent`). Returns its id. Optionally file collections into it right away.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateCollectionFolderParams {
    /// The name parameter
    pub name: String,
    /// Folder id, name or path ("Parent/Child") to nest the new folder inside; omit for top level
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent: Option<String>,
    /// Collections (ids or names) to file inside
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection_ids: Option<Vec<String>>,
}

/// Result returned by the `create_collection_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateCollectionFolderResult {
    pub ok: bool,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `move_collection_folder` tool.
/// Nest a sidebar collection folder inside another folder, or move it back to the top level (omit `parent`). Everything inside it moves along. A folder can't go inside itself or one of its own folders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveCollectionFolderParams {
    /// Folder id, name or path ("Parent/Child")
    pub folder: String,
    /// Destination folder id, name or path; omit for top level
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent: Option<String>,
}

/// Result returned by the `move_collection_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MoveCollectionFolderResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `rename_collection_folder` tool.
/// Rename a sidebar collection folder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RenameCollectionFolderParams {
    /// Folder id or name
    pub folder: String,
    /// The name parameter
    pub name: String,
}

/// Result returned by the `rename_collection_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RenameCollectionFolderResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `delete_collection_folder` tool.
/// Delete a sidebar collection folder, folders nested inside it included. Its collections move up to the folder above it (or the top level) — unless `delete_collections: true`, which deletes them too (their items are kept, re-filed into the Inbox where needed).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteCollectionFolderParams {
    /// Default false
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete_collections: Option<bool>,
    /// Folder id or name
    pub folder: String,
}

/// Result returned by the `delete_collection_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeleteCollectionFolderResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `create_smart_folder` tool.
/// Save a rule-based live search as a smart folder. `match` any/all of the rules must be true (`expect: false` inverts the whole set). Rule format is the same one list_smart_folders returns under `definition.rules`, e.g. {"field":"type","op":"is","text":"video"}, {"field":"tag","op":"contains","text":"logo"}, {"field":"dateAdded","op":"inLast","number":7,"unit":"days"}, {"field":"fileSize","op":"greaterThan","number":5,"unit":"MB"}, {"field":"collection","op":"isIn","refID":"{collection id}"}.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateSmartFolderParams {
    /// Default all. Pass "" (empty string) when not filtering by this.
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#match: Option<SmartFolderMatch>,
    /// The name parameter
    pub name: String,
    /// The rules parameter
    pub rules: Vec<SmartFolderRule>,
    /// Default true; false = NOT (rules)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expect: Option<bool>,
}

/// Result returned by the `create_smart_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateSmartFolderResult {
    pub ok: bool,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `update_smart_folder` tool.
/// Change a smart folder's name, match mode, expect flag, or rules (rules replace the whole set). Only the fields you pass change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSmartFolderParams {
    /// The expect parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expect: Option<bool>,
    /// The rules parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rules: Option<Vec<SmartFolderRule>>,
    /// The name parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Pass "" (empty string) when not filtering by this.
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#match: Option<SmartFolderMatch>,
    /// Smart folder id or name
    pub smart_folder: String,
}

/// Result returned by the `update_smart_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UpdateSmartFolderResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `delete_smart_folder` tool.
/// Delete a smart folder (just the saved search — no items are touched).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteSmartFolderParams {
    /// Smart folder id or name
    pub smart_folder: String,
}

/// Result returned by the `delete_smart_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeleteSmartFolderResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `rename_tag` tool.
/// Rename a tag everywhere it's used (trashed items included). Renaming onto an existing tag merges the two.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RenameTagParams {
    /// The new spelling
    pub name: String,
    /// The tag parameter
    pub tag: String,
}

/// Result returned by the `rename_tag` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RenameTagResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tag: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `delete_tag` tool.
/// Remove a tag from the library: strips it from every item carrying it (trashed ones too) and drops it from the tag list. Items themselves are untouched.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteTagParams {
    /// The tag parameter
    pub tag: String,
}

/// Result returned by the `delete_tag` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeleteTagResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_items: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `create_tag` tool.
/// Create an empty tag (shows in the Tags list with 0 items) so it exists before anything carries it. Tagging items with add_tags creates tags implicitly — this is only for preparing a vocabulary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateTagParams {
    /// The name parameter
    pub name: String,
}

/// Result returned by the `create_tag` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateTagResult {
    pub ok: bool,
    pub name: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `create_tag_folder` tool.
/// Create a sidebar folder that groups tags (the Tags section nests them under it). Returns its id. Optionally file existing tags into it right away; a tag sits in one folder at a time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateTagFolderParams {
    /// Tags (names) to file inside
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
    /// The name parameter
    pub name: String,
}

/// Result returned by the `create_tag_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateTagFolderResult {
    pub ok: bool,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `rename_tag_folder` tool.
/// Rename a sidebar tag folder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RenameTagFolderParams {
    /// Tag folder id or name
    pub folder: String,
    /// The name parameter
    pub name: String,
}

/// Result returned by the `rename_tag_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RenameTagFolderResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `delete_tag_folder` tool.
/// Delete a sidebar tag folder. Its tags stay on every item and read as loose tags again — nothing is untagged.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteTagFolderParams {
    /// Tag folder id or name
    pub folder: String,
}

/// Result returned by the `delete_tag_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeleteTagFolderResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `move_tags_to_folder` tool.
/// File tags into a sidebar tag folder, or pull them back out to the loose list (omit `folder`). A tag sits in one folder at a time, so filing moves it out of any other folder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveTagsToFolderParams {
    /// Tag names
    pub tags: Vec<String>,
    /// Tag folder id or name; omit to unfile
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub folder: Option<String>,
}

/// Result returned by the `move_tags_to_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MoveTagsToFolderResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `arrange_board` tool.
/// Rearrange a collection's canvas: each entry creates a group of the given items at (x, y) world coordinates (groups flow their items in a cluster around their center; a spacing of ~400–800 units between group centers reads well). Items must already be in the collection; items not mentioned keep their current groups. The user can undo this from the canvas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ArrangeBoardParams {
    /// Collection id or name
    pub collection: String,
    /// The groups parameter
    pub groups: Vec<BoardGroupInput>,
}

/// Result returned by the `arrange_board` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ArrangeBoardResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `move_to_group` tool.
/// Move items into an existing canvas group (from get_board) at a position in its order. Items leave whatever group they were in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveToGroupParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
    /// The group_id parameter
    pub group_id: String,
    /// Insert position inside the group (default: end)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub index: Option<i64>,
    /// Collection id or name
    pub collection: String,
}

/// Result returned by the `move_to_group` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MoveToGroupResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `move_group` tool.
/// Move a canvas group to new (x, y) world coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveGroupParams {
    /// Collection id or name
    pub collection: String,
    /// The group_id parameter
    pub group_id: String,
    /// The y parameter
    pub y: f64,
    /// The x parameter
    pub x: f64,
}

/// Result returned by the `move_group` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MoveGroupResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `reset_board_layout` tool.
/// Undo all hand arrangement on a board: every item back into one cluster at the origin, following the grid sort order again.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResetBoardLayoutParams {
    /// Collection id or name
    pub collection: String,
}

/// Result returned by the `reset_board_layout` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ResetBoardLayoutResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `undo_board_layout` tool.
/// Step a board's layout back (or forward with `redo: true`) through its arrangement history — the same undo the canvas offers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UndoBoardLayoutParams {
    /// Collection id or name
    pub collection: String,
    /// Redo instead of undo
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub redo: Option<bool>,
}

/// Result returned by the `undo_board_layout` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UndoBoardLayoutResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `connect_folder` tool.
/// Watch a folder on this Mac: its media shows up in the sidebar's Folders section and becomes searchable (files stay where they are). Scans it right away.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectFolderParams {
    /// Absolute path to a directory
    pub path: String,
}

/// Result returned by the `connect_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConnectFolderResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `disconnect_folder` tool.
/// Stop watching a connected folder. Files on disk are untouched; the folder's items leave the library, including their places in collections and the Trash. The iPhone folder can't be disconnected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DisconnectFolderParams {
    /// Connected folder id or name
    pub folder: String,
}

/// Result returned by the `disconnect_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DisconnectFolderResult {
    pub ok: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `rescan_folder` tool.
/// Re-index a connected folder now: picks up new files, drops ones deleted on disk. Returns the item count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RescanFolderParams {
    /// Connected folder id or name
    pub folder: String,
}

/// Result returned by the `rescan_folder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RescanFolderResult {
    pub ok: bool,
    pub item_count: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `switch_library` tool.
/// Open another Atlas library (from list_libraries, or any path to a .atlaslibrary folder). The whole app switches — every later tool call sees that library.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SwitchLibraryParams {
    /// The path parameter
    pub path: String,
}

/// Result returned by the `switch_library` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SwitchLibraryResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub path: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `create_library` tool.
/// Create a new, empty Atlas library at a path (a folder ending in .atlaslibrary is created) and switch to it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateLibraryParams {
    /// The path parameter
    pub path: String,
}

/// Result returned by the `create_library` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateLibraryResult {
    pub ok: bool,
    pub path: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `search_by_description` tool.
/// Find items by what they SHOW, from a plain-language description — "a red sports car at night", "hand-drawn arrow sketch", "brutalist concrete facade". Atlas embeds the description on this Mac with the same model that indexed every thumbnail, and returns the closest items, best first, each with a cosine `score`. Reach for this whenever the request is about what's IN the picture rather than what it's called: search_items only matches names, tags, notes and URLs, so it misses everything nobody happened to label. Narrow with `kind` / `collection`. Colors and preview-less links are never indexed. Pass the ids to show_items to put the results on the user's screen. Needs Settings ▸ General ▸ "Semantic search" on; the first query after launch loads the text model and can take a few seconds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchByDescriptionParams {
    /// Only items of this kind. Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kind: Option<SearchDescriptionKind>,
    /// Drop hits below this cosine — default 0.04 (what in-app search uses); 0 means unset
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_score: Option<f64>,
    /// Only items in this collection (id or name)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
    /// Plain-language description of what the item shows
    pub query: String,
    /// Max results, default 30, max 100
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<i64>,
}

/// Result returned by the `search_by_description` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchByDescriptionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<serde_json::Value>,
    pub query: String,
    pub results: Vec<SemanticSearchResultItem>,
    #[serde(alias = "total_indexed", default)]
    pub returned: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `store_embeddings` tool.
/// Store semantic-embedding vectors for library items, keyed by `content_hash` (from search_items / get_item file facts). `model` names the embedding model that produced them (e.g. "siglip2-base-patch16-256") — vectors from different models don't mix, so storing under a new model id DISCARDS the previously stored index. Vectors are L2-normalized on the way in and persisted with the library. Send items in batches (max 1000 per call). Entries whose hash is unknown to the library are still stored (the item may sync in later); use `prune: true` on a final batch to drop vectors whose hash no longer matches any item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StoreEmbeddingsParams {
    /// Vectors to store, keyed by the item's content_hash
    pub entries: Vec<EmbeddingEntry>,
    /// Afterwards, drop stored vectors whose hash matches no current item
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub prune: Option<bool>,
    /// Embedding model id these vectors came from
    pub model: String,
}

/// Result returned by the `store_embeddings` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StoreEmbeddingsResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_vectors: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `query_embeddings` tool.
/// Semantic search over the vectors stored with store_embeddings, for clients that bring their own model: pass a query vector (a text or image embedding from the SAME model) and get the closest items back, best first, scored by cosine similarity. To search Atlas's own on-device index from plain language — no model needed — use search_by_description instead. Trashed items are excluded. With no vector, returns index status only (model, count, dimension). Combine with select_items / show_in_app to surface the hits in the Atlas window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryEmbeddingsParams {
    /// Drop results whose cosine falls below this (model-specific scale)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_score: Option<f64>,
    /// Max results, default 20, max 100
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<i64>,
    /// Query embedding (same model + dimension as the stored vectors)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vector: Option<Vec<f64>>,
    /// Embedding model id the query vector came from
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub model: Option<String>,
}

/// Result returned by the `query_embeddings` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct QueryEmbeddingsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `show_in_app` tool.
/// Navigate Atlas's window to a destination: a collection, collection folder, tag, smart folder, connected folder, or one of the built-in views (`all`, `links`, `colors`, `instagram`, `x`, `inbox`, `likes`, `trash`). Pass exactly one of `collection`, `collection_folder`, `tag`, `smart_folder`, `connected_folder`, or `view`. Brings the app to the front unless `front: false`. To show a set of individual items instead — search results, your own picks — use show_items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShowInAppParams {
    /// Bring Atlas to the front (default true)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub front: Option<bool>,
    /// Collection folder id or name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection_folder: Option<String>,
    /// Collection id or name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
    /// The tag parameter
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<String>,
    /// Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub view: Option<BuiltinAppView>,
    /// Connected folder id or name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub connected_folder: Option<String>,
    /// Smart folder id or name
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smart_folder: Option<String>,
}

/// Result returned by the `show_in_app` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ShowInAppResult {
    pub ok: bool,
    pub showing: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `show_items` tool.
/// Put an arbitrary set of items on screen: the same results view in-app search and Find Similar use — grid, in the order you pass them, with a Back button to wherever the user was. This is how you show a result set that matches no collection (search hits, a filtered subset, the ones you picked); show_in_app only navigates to a whole collection, tag, smart folder, or built-in view. `title` names the set in the header — pass what you searched for. `select: true` also selects them so the user can act on them right away. Up to 500 items; trashed ones are skipped.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShowItemsParams {
    /// Bring Atlas to the front (default true)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub front: Option<bool>,
    /// Items to show, in the order they should appear
    pub item_ids: Vec<String>,
    /// What to call this set in the header, e.g. the query behind it
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,
    /// Also select the items (default false)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub select: Option<bool>,
}

/// Result returned by the `show_items` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ShowItemsResult {
    pub ok: bool,
    pub shown: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `open_item` tool.
/// Open an item in Atlas's focused (full-size) view so the user sees it. Navigates to the item's collection first unless it's already on screen (pass `collection` to pick which one). Links open in the browser and color cards search the library by their color instead — that's what opening them means in Atlas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenItemParams {
    /// Collection (id or name) to show it in
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collection: Option<String>,
    /// The item_id parameter
    pub item_id: String,
    /// Bring Atlas to the front (default true)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub front: Option<bool>,
}

/// Result returned by the `open_item` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OpenItemResult {
    pub ok: bool,
    pub opened: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `select_items` tool.
/// Select items in the current grid/canvas view (replacing the selection) so the user can act on them. Items not on screen are ignored — land on them first: show_in_app for a whole collection, show_items for a set of your own (which can select them in the same call).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SelectItemsParams {
    /// Bring Atlas to the front (default true)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub front: Option<bool>,
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `select_items` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SelectItemsResult {
    pub ok: bool,
    pub selected: i64,
    pub not_on_screen: i64,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `set_view_mode` tool.
/// Switch the current view between its display modes: `grid` (packed rows), `infinite` (endless zoomable plane), and `canvas` (free-form board). Some destinations are grid-only (Trash, smart folders, tags, sources, connected folders, Links, Colors) and All Items has no canvas — the result reports the mode actually in effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetViewModeParams {
    /// The mode parameter
    pub mode: ViewDisplayMode,
    /// Bring Atlas to the front (default true)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub front: Option<bool>,
}

/// Result returned by the `set_view_mode` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SetViewModeResult {
    pub ok: bool,
    pub mode: ViewDisplayMode,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `show_in_finder` tool.
/// Reveal items' original files in the Finder (up to 50).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShowInFinderParams {
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `show_in_finder` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ShowInFinderResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revealed: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `get_settings` tool.
/// Every user-tweakable app setting with its current value, type, and allowed values or range: appearance (light/dark, window style, backgrounds, tile radius/spacing, selection color), behavior (open/hover/playback, sort orders), general, sidebar visibility, download services, update channels, MCP, and — while the Debug tab is unlocked — every Debug knob. Change them with set_setting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetSettingsParams {
    /// Only this section. Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub section: Option<SettingsSection>,
}

/// Result returned by the `get_settings` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GetSettingsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub settings: std::collections::HashMap<String, Vec<AtlasSettingItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `set_setting` tool.
/// Change one app setting by name (from get_settings) — validated, applied live, and persisted, the same paths the Settings window drives. Numbers are clamped to their range. The MCP safety gates (mcp_enabled, allow_permanent_deletion, debug_unlocked) are read-only for agents; Debug knobs need the Debug tab unlocked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetSettingParams {
    /// The new value; booleans also accept "true"/"false", numbers accept digit strings
    pub value: serde_json::Value,
    /// Setting name from get_settings
    pub name: String,
}

/// Result returned by the `set_setting` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SetSettingResult {
    pub ok: bool,
    pub name: String,
    pub value: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `open_settings` tool.
/// Open Atlas's Settings window, optionally on a specific page. The license page only exists while licensing is active and the debug page while it's unlocked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenSettingsParams {
    /// Pass "" (empty string) when not filtering by this.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub page: Option<SettingsPage>,
}

/// Result returned by the `open_settings` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OpenSettingsResult {
    pub ok: bool,
    pub opened: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `app_info` tool.
/// Facts about the running app: version, build, release channel (an `a` version suffix = internal alpha, `b` = beta, plain = stable), update-channel toggles, macOS version, MCP endpoint, and the open library.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppInfoParams {}

/// Result returned by the `app_info` tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AppInfoResult {
    pub name: String,
    pub version: String,
    pub build: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_unlocked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `empty_trash` tool.
/// Permanently delete everything in the Trash. Irreversible (a metadata snapshot is kept, the files are not). Requires Settings ▸ Developers ▸ "Allow permanent deletion".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyTrashParams {
    /// Must be true
    pub confirm: bool,
}

/// Result returned by the `empty_trash` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmptyTrashResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purged: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `delete_permanently` tool.
/// Permanently delete items that are already in the Trash. Irreversible. Requires Settings ▸ Developers ▸ "Allow permanent deletion". Items not in the Trash are skipped — trash_items them first.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletePermanentlyParams {
    /// Must be true
    pub confirm: bool,
    /// The item_ids parameter
    pub item_ids: Vec<String>,
}

/// Result returned by the `delete_permanently` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeletePermanentlyResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Parameters for the `run_maintenance` tool.
/// Library housekeeping. `remove_duplicates` runs File ▸ Clean up library: byte-identical files collapse onto one stored copy, a collection holding several keeps one entry (an item left in no collection goes to the Trash), and every other copy stays in place as a virtual copy, deleting the freed files; `remove_missing_originals` forgets items whose files are gone from disk; `purge_expired_trash` deletes trashed items older than `days`. Those three are permanent and require Settings ▸ Developers ▸ "Allow permanent deletion". `repair_thumbnails` regenerates missing previews and is always allowed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RunMaintenanceParams {
    /// The action parameter
    pub action: MaintenanceAction,
    /// purge_expired_trash: delete items trashed more than N days ago (default 30)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub days: Option<i64>,
}

/// Result returned by the `run_maintenance` tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RunMaintenanceResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freed_count: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

// ============================================================================
// MCP Native Protocol Types (tools/list, resources/list, resources/read)
// ============================================================================

/// Definition of an individual MCP tool returned by `tools/list`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct McpToolDefinition {
    /// Name of the tool.
    pub name: String,
    /// Human-readable description of the tool.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    /// JSON Schema describing input parameters.
    #[serde(
        rename = "inputSchema",
        alias = "input_schema",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub input_schema: Option<serde_json::Value>,
    /// Additional dynamic fields.
    #[serde(flatten, default)]
    pub extra: serde_json::Value,
}

/// Response payload of `tools/list` JSON-RPC method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct McpListToolsResult {
    /// List of available tools.
    pub tools: Vec<McpToolDefinition>,
    /// Optional pagination cursor.
    #[serde(
        rename = "nextCursor",
        alias = "next_cursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_cursor: Option<String>,
    /// Additional dynamic fields.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Definition of an MCP resource returned by `resources/list`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct McpResourceDefinition {
    /// Unique URI of the resource.
    pub uri: String,
    /// Human-readable name of the resource.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Description of what the resource contains.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    /// MIME type of the resource content.
    #[serde(
        rename = "mimeType",
        alias = "mime_type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mime_type: Option<String>,
    /// Additional dynamic fields.
    #[serde(flatten, default)]
    pub extra: serde_json::Value,
}

/// Response payload of `resources/list` JSON-RPC method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct McpListResourcesResult {
    /// List of available resources.
    pub resources: Vec<McpResourceDefinition>,
    /// Optional pagination cursor.
    #[serde(
        rename = "nextCursor",
        alias = "next_cursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_cursor: Option<String>,
    /// Additional dynamic fields.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Content payload item returned by `resources/read`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct McpResourceContent {
    /// URI of the read resource.
    pub uri: String,
    /// MIME type of the content.
    #[serde(
        rename = "mimeType",
        alias = "mime_type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mime_type: Option<String>,
    /// Text content when the resource is text-based.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub text: Option<String>,
    /// Base64 encoded blob when the resource is binary.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blob: Option<String>,
    /// Additional dynamic fields.
    #[serde(flatten, default)]
    pub extra: serde_json::Value,
}

/// Response payload of `resources/read` JSON-RPC method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct McpReadResourceResult {
    /// List of resource contents.
    pub contents: Vec<McpResourceContent>,
    /// Additional dynamic fields.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
