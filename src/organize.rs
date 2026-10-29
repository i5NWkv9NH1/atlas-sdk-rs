//! Atlas organization, collections, tags, smart folders, and libraries domain methods on AtlasClient.

use crate::models::*;
use crate::transport::{AtlasTransport, Result};

use crate::AtlasClient;

impl<T: AtlasTransport> AtlasClient<T> {
    /// List every collection (board) in the Atlas library: id, name, sidebar folder, item count. System collections (Inbox, Likes, Trash) are marked. Per-collection counts are PLACEMENTS, not unique items — one item can live in several collections, and links / connected-folder items live outside collections entirely — so never sum them. The `library` object carries the exact unique totals.
    pub async fn list_collections(&self) -> Result<ListCollectionsResult> {
        // * Call parameterless tool 'list_collections'
        let val = self
            .transport
            .call_tool("list_collections", serde_json::json!({}))
            .await?;
        let res: ListCollectionsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Create a new collection (board). Returns its id.
    pub async fn create_collection(
        &self,
        params: &CreateCollectionParams,
    ) -> Result<CreateCollectionResult> {
        // * Serialize arguments and call tool 'create_collection'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("create_collection", args).await?;
        let res: CreateCollectionResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Rename a collection (board). System collections (Inbox, Likes, Trash) can't be renamed.
    pub async fn rename_collection(
        &self,
        params: &RenameCollectionParams,
    ) -> Result<RenameCollectionResult> {
        // * Serialize arguments and call tool 'rename_collection'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("rename_collection", args).await?;
        let res: RenameCollectionResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Put a collection away (archived) or bring it back. Archived collections keep every item but leave the sidebar and the ⌘K palette; list_collections still shows them with `archived: true`.
    pub async fn archive_collection(
        &self,
        params: &ArchiveCollectionParams,
    ) -> Result<ArchiveCollectionResult> {
        // * Serialize arguments and call tool 'archive_collection'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("archive_collection", args).await?;
        let res: ArchiveCollectionResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Delete a collection. By default its items are kept (re-filed into the Inbox if they're in no other collection). `delete_items: true` also removes items that live only in this collection — that is permanent and requires Settings ▸ Developers ▸ "Allow permanent deletion".
    pub async fn delete_collection(
        &self,
        params: &DeleteCollectionParams,
    ) -> Result<DeleteCollectionResult> {
        // * Serialize arguments and call tool 'delete_collection'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("delete_collection", args).await?;
        let res: DeleteCollectionResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Reorder the user's collections in the sidebar. Pass every collection id in the wanted order; ids you leave out keep their relative order after the listed ones.
    pub async fn set_collection_order(
        &self,
        params: &SetCollectionOrderParams,
    ) -> Result<SetCollectionOrderResult> {
        // * Serialize arguments and call tool 'set_collection_order'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("set_collection_order", args)
            .await?;
        let res: SetCollectionOrderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// File items into a collection. By default items are added (kept everywhere they already are); pass move=true to move them out of the source collections instead.
    pub async fn add_to_collection(
        &self,
        params: &AddToCollectionParams,
    ) -> Result<AddToCollectionResult> {
        // * Serialize arguments and call tool 'add_to_collection'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("add_to_collection", args).await?;
        let res: AddToCollectionResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Take items out of one collection without trashing them. Items in no other collection are re-filed into the Inbox so nothing is lost.
    pub async fn remove_from_collection(
        &self,
        params: &RemoveFromCollectionParams,
    ) -> Result<RemoveFromCollectionResult> {
        // * Serialize arguments and call tool 'remove_from_collection'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("remove_from_collection", args)
            .await?;
        let res: RemoveFromCollectionResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Sidebar folders that group collections: id, name, the collections filed directly inside each, and — since folders nest — `parent_id` (null at the top level) and `path` ("Parent ▸ Child").
    pub async fn list_collection_folders(&self) -> Result<ListCollectionFoldersResult> {
        // * Call parameterless tool 'list_collection_folders'
        let val = self
            .transport
            .call_tool("list_collection_folders", serde_json::json!({}))
            .await?;
        let res: ListCollectionFoldersResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Create a sidebar folder that groups collections — at the top level, or nested inside another folder (`parent`). Returns its id. Optionally file collections into it right away.
    pub async fn create_collection_folder(
        &self,
        params: &CreateCollectionFolderParams,
    ) -> Result<CreateCollectionFolderResult> {
        // * Serialize arguments and call tool 'create_collection_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("create_collection_folder", args)
            .await?;
        let res: CreateCollectionFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Nest a sidebar collection folder inside another folder, or move it back to the top level (omit `parent`). Everything inside it moves along. A folder can't go inside itself or one of its own folders.
    pub async fn move_collection_folder(
        &self,
        params: &MoveCollectionFolderParams,
    ) -> Result<MoveCollectionFolderResult> {
        // * Serialize arguments and call tool 'move_collection_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("move_collection_folder", args)
            .await?;
        let res: MoveCollectionFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Rename a sidebar collection folder.
    pub async fn rename_collection_folder(
        &self,
        params: &RenameCollectionFolderParams,
    ) -> Result<RenameCollectionFolderResult> {
        // * Serialize arguments and call tool 'rename_collection_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("rename_collection_folder", args)
            .await?;
        let res: RenameCollectionFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Delete a sidebar collection folder, folders nested inside it included. Its collections move up to the folder above it (or the top level) — unless `delete_collections: true`, which deletes them too (their items are kept, re-filed into the Inbox where needed).
    pub async fn delete_collection_folder(
        &self,
        params: &DeleteCollectionFolderParams,
    ) -> Result<DeleteCollectionFolderResult> {
        // * Serialize arguments and call tool 'delete_collection_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("delete_collection_folder", args)
            .await?;
        let res: DeleteCollectionFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// File a collection into a sidebar collection folder (nested folders included — pass a path like "Projekte/Clients" to pick one by its parents), or pull it back to the top level (omit `folder`).
    pub async fn move_collection_to_folder(
        &self,
        params: &MoveCollectionToFolderParams,
    ) -> Result<MoveCollectionToFolderResult> {
        // * Serialize arguments and call tool 'move_collection_to_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("move_collection_to_folder", args)
            .await?;
        let res: MoveCollectionToFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// List every tag in the library with how many items carry it. A tag filed in a sidebar tag folder carries `folder` (name) and `folder_id`; `tag_folders` lists the folders themselves with their member tags. Loose tags have no `folder`.
    pub async fn list_tags(&self) -> Result<ListTagsResult> {
        // * Call parameterless tool 'list_tags'
        let val = self
            .transport
            .call_tool("list_tags", serde_json::json!({}))
            .await?;
        let res: ListTagsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Create an empty tag (shows in the Tags list with 0 items) so it exists before anything carries it. Tagging items with add_tags creates tags implicitly — this is only for preparing a vocabulary.
    pub async fn create_tag(&self, params: &CreateTagParams) -> Result<CreateTagResult> {
        // * Serialize arguments and call tool 'create_tag'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("create_tag", args).await?;
        let res: CreateTagResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Rename a tag everywhere it's used (trashed items included). Renaming onto an existing tag merges the two.
    pub async fn rename_tag(&self, params: &RenameTagParams) -> Result<RenameTagResult> {
        // * Serialize arguments and call tool 'rename_tag'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("rename_tag", args).await?;
        let res: RenameTagResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Remove a tag from the library: strips it from every item carrying it (trashed ones too) and drops it from the tag list. Items themselves are untouched.
    pub async fn delete_tag(&self, params: &DeleteTagParams) -> Result<DeleteTagResult> {
        // * Serialize arguments and call tool 'delete_tag'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("delete_tag", args).await?;
        let res: DeleteTagResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Add one or more tags to a set of items (existing tags are kept; duplicates ignored). Batched: one library save regardless of item count.
    pub async fn add_tags(&self, params: &AddTagsParams) -> Result<AddTagsResult> {
        // * Serialize arguments and call tool 'add_tags'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("add_tags", args).await?;
        let res: AddTagsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Remove one or more tags from a set of items (case-insensitive).
    pub async fn remove_tags(&self, params: &RemoveTagsParams) -> Result<RemoveTagsResult> {
        // * Serialize arguments and call tool 'remove_tags'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("remove_tags", args).await?;
        let res: RemoveTagsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Replace an item's tags wholesale (an empty array clears them). Use add_tags / remove_tags to edit incrementally across many items.
    pub async fn set_tags(&self, params: &SetTagsParams) -> Result<SetTagsResult> {
        // * Serialize arguments and call tool 'set_tags'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("set_tags", args).await?;
        let res: SetTagsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// File tags into a sidebar tag folder, or pull them back out to the loose list (omit `folder`). A tag sits in one folder at a time, so filing moves it out of any other folder.
    pub async fn move_tags_to_folder(
        &self,
        params: &MoveTagsToFolderParams,
    ) -> Result<MoveTagsToFolderResult> {
        // * Serialize arguments and call tool 'move_tags_to_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("move_tags_to_folder", args)
            .await?;
        let res: MoveTagsToFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Create a sidebar folder that groups tags (the Tags section nests them under it). Returns its id. Optionally file existing tags into it right away; a tag sits in one folder at a time.
    pub async fn create_tag_folder(
        &self,
        params: &CreateTagFolderParams,
    ) -> Result<CreateTagFolderResult> {
        // * Serialize arguments and call tool 'create_tag_folder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("create_tag_folder", args).await?;
        let res: CreateTagFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Rename a sidebar tag folder.
    pub async fn rename_tag_folder(
        &self,
        params: &RenameTagFolderParams,
    ) -> Result<RenameTagFolderResult> {
        // * Serialize arguments and call tool 'rename_tag_folder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("rename_tag_folder", args).await?;
        let res: RenameTagFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Delete a sidebar tag folder. Its tags stay on every item and read as loose tags again — nothing is untagged.
    pub async fn delete_tag_folder(
        &self,
        params: &DeleteTagFolderParams,
    ) -> Result<DeleteTagFolderResult> {
        // * Serialize arguments and call tool 'delete_tag_folder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("delete_tag_folder", args).await?;
        let res: DeleteTagFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// List saved smart folders (rule-based live searches): id, name, match count, and their rules.
    pub async fn list_smart_folders(&self) -> Result<ListSmartFoldersResult> {
        // * Call parameterless tool 'list_smart_folders'
        let val = self
            .transport
            .call_tool("list_smart_folders", serde_json::json!({}))
            .await?;
        let res: ListSmartFoldersResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Save a rule-based live search as a smart folder. `match` any/all of the rules must be true (`expect: false` inverts the whole set). Rule format is the same one list_smart_folders returns under `definition.rules`, e.g. {"field":"type","op":"is","text":"video"}, {"field":"tag","op":"contains","text":"logo"}, {"field":"dateAdded","op":"inLast","number":7,"unit":"days"}, {"field":"fileSize","op":"greaterThan","number":5,"unit":"MB"}, {"field":"collection","op":"isIn","refID":"{collection id}"}.
    pub async fn create_smart_folder(
        &self,
        params: &CreateSmartFolderParams,
    ) -> Result<CreateSmartFolderResult> {
        // * Serialize arguments and call tool 'create_smart_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("create_smart_folder", args)
            .await?;
        let res: CreateSmartFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Change a smart folder's name, match mode, expect flag, or rules (rules replace the whole set). Only the fields you pass change.
    pub async fn update_smart_folder(
        &self,
        params: &UpdateSmartFolderParams,
    ) -> Result<UpdateSmartFolderResult> {
        // * Serialize arguments and call tool 'update_smart_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("update_smart_folder", args)
            .await?;
        let res: UpdateSmartFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Delete a smart folder (just the saved search — no items are touched).
    pub async fn delete_smart_folder(
        &self,
        params: &DeleteSmartFolderParams,
    ) -> Result<DeleteSmartFolderResult> {
        // * Serialize arguments and call tool 'delete_smart_folder'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("delete_smart_folder", args)
            .await?;
        let res: DeleteSmartFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Rearrange a collection's canvas: each entry creates a group of the given items at (x, y) world coordinates (groups flow their items in a cluster around their center; a spacing of ~400–800 units between group centers reads well). Items must already be in the collection; items not mentioned keep their current groups. The user can undo this from the canvas.
    pub async fn arrange_board(&self, params: &ArrangeBoardParams) -> Result<ArrangeBoardResult> {
        // * Serialize arguments and call tool 'arrange_board'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("arrange_board", args).await?;
        let res: ArrangeBoardResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Move items into an existing canvas group (from get_board) at a position in its order. Items leave whatever group they were in.
    pub async fn move_to_group(&self, params: &MoveToGroupParams) -> Result<MoveToGroupResult> {
        // * Serialize arguments and call tool 'move_to_group'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("move_to_group", args).await?;
        let res: MoveToGroupResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Move a canvas group to new (x, y) world coordinates.
    pub async fn move_group(&self, params: &MoveGroupParams) -> Result<MoveGroupResult> {
        // * Serialize arguments and call tool 'move_group'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("move_group", args).await?;
        let res: MoveGroupResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Undo all hand arrangement on a board: every item back into one cluster at the origin, following the grid sort order again.
    pub async fn reset_board_layout(
        &self,
        params: &ResetBoardLayoutParams,
    ) -> Result<ResetBoardLayoutResult> {
        // * Serialize arguments and call tool 'reset_board_layout'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("reset_board_layout", args).await?;
        let res: ResetBoardLayoutResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Step a board's layout back (or forward with `redo: true`) through its arrangement history — the same undo the canvas offers.
    pub async fn undo_board_layout(
        &self,
        params: &UndoBoardLayoutParams,
    ) -> Result<UndoBoardLayoutResult> {
        // * Serialize arguments and call tool 'undo_board_layout'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("undo_board_layout", args).await?;
        let res: UndoBoardLayoutResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Folders on disk that Atlas watches (the sidebar's Folders section): id, name, path, whether it's mounted on this Mac, and how many library items come from it. The iPhone folder (iCloud) is included. Items from connected folders carry `connected_folder_id`.
    pub async fn list_connected_folders(&self) -> Result<ListConnectedFoldersResult> {
        // * Call parameterless tool 'list_connected_folders'
        let val = self
            .transport
            .call_tool("list_connected_folders", serde_json::json!({}))
            .await?;
        let res: ListConnectedFoldersResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Watch a folder on this Mac: its media shows up in the sidebar's Folders section and becomes searchable (files stay where they are). Scans it right away.
    pub async fn connect_folder(
        &self,
        params: &ConnectFolderParams,
    ) -> Result<ConnectFolderResult> {
        // * Serialize arguments and call tool 'connect_folder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("connect_folder", args).await?;
        let res: ConnectFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Stop watching a connected folder. Files on disk are untouched; the folder's items leave the library, including their places in collections and the Trash. The iPhone folder can't be disconnected.
    pub async fn disconnect_folder(
        &self,
        params: &DisconnectFolderParams,
    ) -> Result<DisconnectFolderResult> {
        // * Serialize arguments and call tool 'disconnect_folder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("disconnect_folder", args).await?;
        let res: DisconnectFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Re-index a connected folder now: picks up new files, drops ones deleted on disk. Returns the item count.
    pub async fn rescan_folder(&self, params: &RescanFolderParams) -> Result<RescanFolderResult> {
        // * Serialize arguments and call tool 'rescan_folder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("rescan_folder", args).await?;
        let res: RescanFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Which library is open and its totals: name, path, read-only state, unique item counts by kind, trash size, tag / collection / smart folder / connected folder counts, how much of the library the visual-similarity index covers. Pass `include_bytes: true` to also total the originals on disk (walks every file — slow on big libraries).
    pub async fn library_info(&self, params: &LibraryInfoParams) -> Result<LibraryInfoResult> {
        // * Serialize arguments and call tool 'library_info'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("library_info", args).await?;
        let res: LibraryInfoResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Every library Atlas knows on this Mac (the open one is marked). Use switch_library to open another.
    pub async fn list_libraries(&self) -> Result<ListLibrariesResult> {
        // * Call parameterless tool 'list_libraries'
        let val = self
            .transport
            .call_tool("list_libraries", serde_json::json!({}))
            .await?;
        let res: ListLibrariesResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Create a new, empty Atlas library at a path (a folder ending in .atlaslibrary is created) and switch to it.
    pub async fn create_library(
        &self,
        params: &CreateLibraryParams,
    ) -> Result<CreateLibraryResult> {
        // * Serialize arguments and call tool 'create_library'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("create_library", args).await?;
        let res: CreateLibraryResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Open another Atlas library (from list_libraries, or any path to a .atlaslibrary folder). The whole app switches — every later tool call sees that library.
    pub async fn switch_library(
        &self,
        params: &SwitchLibraryParams,
    ) -> Result<SwitchLibraryResult> {
        // * Serialize arguments and call tool 'switch_library'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("switch_library", args).await?;
        let res: SwitchLibraryResult = serde_json::from_value(val)?;
        // * Synchronize active library on the transport token resolver
        self.transport.set_active_library(&params.path);
        Ok(res)
    }
}
