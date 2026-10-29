//! Atlas UI navigation, system settings, and app info domain methods on AtlasClient.

use crate::models::*;
use crate::transport::{AtlasTransport, Result};

use crate::AtlasClient;

impl<T: AtlasTransport> AtlasClient<T> {
    /// Navigate Atlas's window to a destination: a collection, collection folder, tag, smart folder, connected folder, or one of the built-in views (`all`, `links`, `colors`, `instagram`, `x`, `inbox`, `likes`, `trash`). Pass exactly one of `collection`, `collection_folder`, `tag`, `smart_folder`, `connected_folder`, or `view`. Brings the app to the front unless `front: false`. To show a set of individual items instead — search results, your own picks — use show_items.
    pub async fn show_in_app(&self, params: &ShowInAppParams) -> Result<ShowInAppResult> {
        // * Serialize arguments and call tool 'show_in_app'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("show_in_app", args).await?;
        let res: ShowInAppResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Put an arbitrary set of items on screen: the same results view in-app search and Find Similar use — grid, in the order you pass them, with a Back button to wherever the user was. This is how you show a result set that matches no collection (search hits, a filtered subset, the ones you picked); show_in_app only navigates to a whole collection, tag, smart folder, or built-in view. `title` names the set in the header — pass what you searched for. `select: true` also selects them so the user can act on them right away. Up to 500 items; trashed ones are skipped.
    pub async fn show_items(&self, params: &ShowItemsParams) -> Result<ShowItemsResult> {
        // * Serialize arguments and call tool 'show_items'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("show_items", args).await?;
        let res: ShowItemsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Open an item in Atlas's focused (full-size) view so the user sees it. Navigates to the item's collection first unless it's already on screen (pass `collection` to pick which one). Links open in the browser and color cards search the library by their color instead — that's what opening them means in Atlas.
    pub async fn open_item(&self, params: &OpenItemParams) -> Result<OpenItemResult> {
        // * Serialize arguments and call tool 'open_item'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("open_item", args).await?;
        let res: OpenItemResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Select items in the current grid/canvas view (replacing the selection) so the user can act on them. Items not on screen are ignored — land on them first: show_in_app for a whole collection, show_items for a set of your own (which can select them in the same call).
    pub async fn select_items(&self, params: &SelectItemsParams) -> Result<SelectItemsResult> {
        // * Serialize arguments and call tool 'select_items'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("select_items", args).await?;
        let res: SelectItemsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Switch the current view between its display modes: `grid` (packed rows), `infinite` (endless zoomable plane), and `canvas` (free-form board). Some destinations are grid-only (Trash, smart folders, tags, sources, connected folders, Links, Colors) and All Items has no canvas — the result reports the mode actually in effect.
    pub async fn set_view_mode(&self, params: &SetViewModeParams) -> Result<SetViewModeResult> {
        // * Serialize arguments and call tool 'set_view_mode'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("set_view_mode", args).await?;
        let res: SetViewModeResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Reveal items' original files in the Finder (up to 50).
    pub async fn show_in_finder(&self, params: &ShowInFinderParams) -> Result<ShowInFinderResult> {
        // * Serialize arguments and call tool 'show_in_finder'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("show_in_finder", args).await?;
        let res: ShowInFinderResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Every user-tweakable app setting with its current value, type, and allowed values or range: appearance (light/dark, window style, backgrounds, tile radius/spacing, selection color), behavior (open/hover/playback, sort orders), general, sidebar visibility, download services, update channels, MCP, and — while the Debug tab is unlocked — every Debug knob. Change them with set_setting.
    pub async fn get_settings(&self, params: &GetSettingsParams) -> Result<GetSettingsResult> {
        // * Serialize arguments and call tool 'get_settings'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("get_settings", args).await?;
        let res: GetSettingsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Change one app setting by name (from get_settings) — validated, applied live, and persisted, the same paths the Settings window drives. Numbers are clamped to their range. The MCP safety gates (mcp_enabled, allow_permanent_deletion, debug_unlocked) are read-only for agents; Debug knobs need the Debug tab unlocked.
    pub async fn set_setting(&self, params: &SetSettingParams) -> Result<SetSettingResult> {
        // * Serialize arguments and call tool 'set_setting'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("set_setting", args).await?;
        let res: SetSettingResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Open Atlas's Settings window, optionally on a specific page. The license page only exists while licensing is active and the debug page while it's unlocked.
    pub async fn open_settings(&self, params: &OpenSettingsParams) -> Result<OpenSettingsResult> {
        // * Serialize arguments and call tool 'open_settings'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("open_settings", args).await?;
        let res: OpenSettingsResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Facts about the running app: version, build, release channel (an `a` version suffix = internal alpha, `b` = beta, plain = stable), update-channel toggles, macOS version, MCP endpoint, and the open library.
    pub async fn app_info(&self) -> Result<AppInfoResult> {
        // * Call parameterless tool 'app_info'
        let val = self
            .transport
            .call_tool("app_info", serde_json::json!({}))
            .await?;
        let res: AppInfoResult = serde_json::from_value(val)?;
        Ok(res)
    }

    /// Library housekeeping. `remove_duplicates` runs File ▸ Clean up library: byte-identical files collapse onto one stored copy, a collection holding several keeps one entry (an item left in no collection goes to the Trash), and every other copy stays in place as a virtual copy, deleting the freed files; `remove_missing_originals` forgets items whose files are gone from disk; `purge_expired_trash` deletes trashed items older than `days`. Those three are permanent and require Settings ▸ Developers ▸ "Allow permanent deletion". `repair_thumbnails` regenerates missing previews and is always allowed.
    pub async fn run_maintenance(
        &self,
        params: &RunMaintenanceParams,
    ) -> Result<RunMaintenanceResult> {
        // * Serialize arguments and call tool 'run_maintenance'
        let args = serde_json::to_value(params)?;
        let val = self.transport.call_tool("run_maintenance", args).await?;
        let res: RunMaintenanceResult = serde_json::from_value(val)?;
        Ok(res)
    }
}
