//! Atlas media ingestion and mutation domain methods on AtlasClient.

use crate::models::*;
use crate::transport::{AtlasTransport, Result};

use crate::AtlasClient;

impl<T: AtlasTransport> AtlasClient<T> {
    /// Import media files from absolute paths on this Mac into a collection (defaults to the Inbox). Files are copied into the library; originals are untouched. Returns how many were imported, skipped as duplicates (`skipped_duplicate`) or skipped as unsupported types, plus `failed` (path + reason) for files macOS couldn't read, such as AV1 or VP9 videos.
    pub async fn import_files(&self, params: &ImportFilesParams) -> Result<ImportFilesResult> {
        // * Serialize arguments and call tool 'import_files'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("import_files", args).await?;
        let res: ImportFilesResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Save a URL as a link card (no media download). Files it into the given collection, or just into the library.
    pub async fn add_link(&self, params: &AddLinkParams) -> Result<AddLinkResult> {
        // * Serialize arguments and call tool 'add_link'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("add_link", args).await?;
        let res: AddLinkResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Edit a link card's title, description, or URL. Only the fields you pass change; empty title/description clear the field.
    pub async fn edit_link(&self, params: &EditLinkParams) -> Result<EditLinkResult> {
        // * Serialize arguments and call tool 'edit_link'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("edit_link", args).await?;
        let res: EditLinkResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Save a HEX color as a color card (e.g. #FF6B35, also 3/4/8-digit forms).
    pub async fn add_color(&self, params: &AddColorParams) -> Result<AddColorResult> {
        // * Serialize arguments and call tool 'add_color'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("add_color", args).await?;
        let res: AddColorResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Change a color card's HEX value.
    pub async fn edit_color(&self, params: &EditColorParams) -> Result<EditColorResult> {
        // * Serialize arguments and call tool 'edit_color'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("edit_color", args).await?;
        let res: EditColorResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Download the media behind a web URL and import it — a direct image/video URL, or a social post (X/Twitter, Instagram, Pinterest, …) whose photos and videos are fetched, every carousel slide included. Falls back to saving a link card for social posts with no downloadable media. Slow tool — it performs real downloads.
    pub async fn save_media_from_url(
        &self,
        params: &SaveMediaFromUrlParams,
    ) -> Result<SaveMediaFromUrlResult> {
        // * Serialize arguments and call tool 'save_media_from_url'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("save_media_from_url", args)
            .await?;
        let res: SaveMediaFromUrlResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Crop or rotate images without touching the imported file (it is kept; `revert: true` puts it back). `rotate` turns the picture a quarter turn "left" or "right" on top of its current edit; `crop` replaces the kept region with a rectangle in the TURNED picture's unit square — x, y, width, height in 0…1, top-left origin — so {"x":0, "y":0, "width":1, "height":0.5} keeps the top half. Rotate and crop can be combined. Managed still images only: connected-folder files, vectors and animations are skipped. Returns each image's new pixel size.
    pub async fn edit_image(&self, params: &EditImageParams) -> Result<EditImageResult> {
        // * Serialize arguments and call tool 'edit_image'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("edit_image", args).await?;
        let res: EditImageResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Regenerate the thumbnails of items whose files changed on disk (edited in another app, or a connected-folder file replaced), so tiles and the open item show the current picture. Links and colors have no thumbnail to refresh and are skipped.
    pub async fn refresh_thumbnail(
        &self,
        params: &RefreshThumbnailParams,
    ) -> Result<RefreshThumbnailResult> {
        // * Serialize arguments and call tool 'refresh_thumbnail'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("refresh_thumbnail", args).await?;
        let res: RefreshThumbnailResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Like or unlike items (membership in the Likes collection).
    pub async fn set_liked(&self, params: &SetLikedParams) -> Result<SetLikedResult> {
        // * Serialize arguments and call tool 'set_liked'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("set_liked", args).await?;
        let res: SetLikedResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Set an item's annotation (note), author, location, or source URL. Only the fields you pass change; pass an empty string to clear a field.
    pub async fn set_metadata(&self, params: &SetMetadataParams) -> Result<SetMetadataResult> {
        // * Serialize arguments and call tool 'set_metadata'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("set_metadata", args).await?;
        let res: SetMetadataResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Move items to the Trash. Recoverable — restore_items puts them back; nothing is permanently deleted by this tool.
    pub async fn trash_items(&self, params: &TrashItemsParams) -> Result<TrashItemsResult> {
        // * Serialize arguments and call tool 'trash_items'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("trash_items", args).await?;
        let res: TrashItemsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Restore trashed items — puts them back into the collections they were trashed from (Put Back). Find trashed items with search_items collection:"Trash".
    pub async fn restore_items(&self, params: &RestoreItemsParams) -> Result<RestoreItemsResult> {
        // * Serialize arguments and call tool 'restore_items'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("restore_items", args).await?;
        let res: RestoreItemsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Permanently delete items that are already in the Trash. Irreversible. Requires Settings ▸ Developers ▸ "Allow permanent deletion". Items not in the Trash are skipped — trash_items them first.
    pub async fn delete_permanently(
        &self,
        params: &DeletePermanentlyParams,
    ) -> Result<DeletePermanentlyResult> {
        // * Serialize arguments and call tool 'delete_permanently'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("delete_permanently", args).await?;
        let res: DeletePermanentlyResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Permanently delete everything in the Trash. Irreversible (a metadata snapshot is kept, the files are not). Requires Settings ▸ Developers ▸ "Allow permanent deletion".
    pub async fn empty_trash(&self, params: &EmptyTrashParams) -> Result<EmptyTrashResult> {
        // * Serialize arguments and call tool 'empty_trash'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("empty_trash", args).await?;
        let res: EmptyTrashResult = serde_json::from_value(val)?;
        Ok(res)
    }
}
