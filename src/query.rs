//! Atlas query, search, and retrieval domain methods on AtlasClient.

use crate::models::*;
use crate::transport::{AtlasTransport, Result};

use crate::AtlasClient;

impl<T: AtlasTransport> AtlasClient<T> {
    /// Search library items. All filters are optional and combine with AND. With NO filters, `total_matches` is the exact count of all non-trashed items in the library — use that for "how many items" questions. `query` matches name, link title/description, tags, annotation, author, location, and source URL, case-insensitively — it reads METADATA only, so for "find pictures of X" use search_by_description, which matches what items actually show. Scope with `collection`, `collection_folder`, `smart_folder`, `connected_folder`, or `source` (instagram / x). Filter by kind, tags (all / any), format, size, dimensions, duration, orientation, liked, untagged, date ranges (ISO 8601). Sort by date_added (default), date_created, name, bytes, duration, width, height, or random. Trashed items are excluded unless you search `collection: "Trash"`. For library-wide totals (how many images / videos / items) library_info answers in one cheap call. Enum filters accept "" for "no filter"; 0 on an optional number also means "not set". Returns compact metadata; `verbose: true` adds paths, notes and file facts; use view_items to actually see items.
    pub async fn search_items(&self, params: &SearchItemsParams) -> Result<SearchItemsResult> {
        // * Serialize arguments and call tool 'search_items'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("search_items", args).await?;
        let res: SearchItemsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Find items by what they SHOW, from a plain-language description — "a red sports car at night", "hand-drawn arrow sketch", "brutalist concrete facade". Atlas embeds the description on this Mac with the same model that indexed every thumbnail, and returns the closest items, best first, each with a cosine `score`. Reach for this whenever the request is about what's IN the picture rather than what it's called: search_items only matches names, tags, notes and URLs, so it misses everything nobody happened to label. Narrow with `kind` / `collection`. Colors and preview-less links are never indexed. Pass the ids to show_items to put the results on the user's screen. Needs Settings ▸ General ▸ "Semantic search" on; the first query after launch loads the text model and can take a few seconds.
    pub async fn search_by_description(
        &self,
        params: &SearchByDescriptionParams,
    ) -> Result<SearchByDescriptionResult> {
        // * Serialize arguments and call tool 'search_by_description'
        let args = serde_json::to_value(params)?;
        let val = self
            .transport
            .call_tool("search_by_description", args)
            .await?;
        let res: SearchByDescriptionResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Find items visually similar to a given item, using on-device image embeddings (Apple Vision feature prints). Purely visual — no text understanding. The index builds in the background after launch; results note how much of the library is indexed.
    pub async fn find_similar(&self, params: &FindSimilarParams) -> Result<FindSimilarResult> {
        // * Serialize arguments and call tool 'find_similar'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("find_similar", args).await?;
        let res: FindSimilarResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Full metadata for one item by id, including which collections it belongs to.
    pub async fn get_item(&self, params: &GetItemParams) -> Result<AtlasItemDetail> {
        // * Serialize arguments and call tool 'get_item'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("get_item", args).await?;
        let res: AtlasItemDetail = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Look at items: returns each item's thumbnail as an image (preceded by a text line naming the item id) so you can see the actual content. Up to 20 items per call. `size` caps the longest edge in pixels (default 512, max 1024) — keep it small when scanning many items.
    pub async fn view_items(&self, params: &ViewItemsParams) -> Result<ViewItemsResult> {
        // * Serialize arguments and call tool 'view_items'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("view_items", args).await?;
        let res: ViewItemsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Read an item's original file. Images come back as an image block (downscaled to `size` px on the long edge, default 2048, max 4096 — pass `size: 0` for the untouched bytes, any format); videos and other non-image files return their path, size, and MIME type (read them with a file tool — the path is local to this Mac). `page` picks a carousel slide (0 = cover). The same bytes are available as the MCP resource atlas://item/{id}/original.
    pub async fn get_file(&self, params: &GetFileParams) -> Result<GetFileResult> {
        // * Serialize arguments and call tool 'get_file'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("get_file", args).await?;
        let res: GetFileResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Store semantic-embedding vectors for library items, keyed by `content_hash` (from search_items / get_item file facts). `model` names the embedding model that produced them (e.g. "siglip2-base-patch16-256") — vectors from different models don't mix, so storing under a new model id DISCARDS the previously stored index. Vectors are L2-normalized on the way in and persisted with the library. Send items in batches (max 1000 per call). Entries whose hash is unknown to the library are still stored (the item may sync in later); use `prune: true` on a final batch to drop vectors whose hash no longer matches any item.
    pub async fn store_embeddings(
        &self,
        params: &StoreEmbeddingsParams,
    ) -> Result<StoreEmbeddingsResult> {
        // * Serialize arguments and call tool 'store_embeddings'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("store_embeddings", args).await?;
        let res: StoreEmbeddingsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Semantic search over the vectors stored with store_embeddings, for clients that bring their own model: pass a query vector (a text or image embedding from the SAME model) and get the closest items back, best first, scored by cosine similarity. To search Atlas's own on-device index from plain language — no model needed — use search_by_description instead. Trashed items are excluded. With no vector, returns index status only (model, count, dimension). Combine with select_items / show_in_app to surface the hits in the Atlas window.
    pub async fn query_embeddings(
        &self,
        params: &QueryEmbeddingsParams,
    ) -> Result<QueryEmbeddingsResult> {
        // * Serialize arguments and call tool 'query_embeddings'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("query_embeddings", args).await?;
        let res: QueryEmbeddingsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// The items a saved smart folder currently matches (newest first). Paged like search_items.
    pub async fn run_smart_folder(
        &self,
        params: &RunSmartFolderParams,
    ) -> Result<RunSmartFolderResult> {
        // * Serialize arguments and call tool 'run_smart_folder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("run_smart_folder", args).await?;
        let res: RunSmartFolderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// A collection's canvas layout: its groups (spatial clusters) with positions and the ordered items inside each.
    pub async fn get_board(&self, params: &GetBoardParams) -> Result<GetBoardResult> {
        // * Serialize arguments and call tool 'get_board'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("get_board", args).await?;
        let res: GetBoardResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// What the user is looking at right now: the collection (if any), the items on screen (ids, newest-first is not implied — this is display order), and which are selected. Use it to act on "these" / "the selected ones".
    pub async fn get_current_view(
        &self,
        params: &GetCurrentViewParams,
    ) -> Result<GetCurrentViewResult> {
        // * Serialize arguments and call tool 'get_current_view'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("get_current_view", args).await?;
        let res: GetCurrentViewResult = serde_json::from_value(val)?;
        Ok(res)
    }
}
