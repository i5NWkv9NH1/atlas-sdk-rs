//! Atlas MCP transport and network layer.
//!
//! Provides JSON-RPC 2.0 communication over HTTP, multi-library token discovery,
//! silent application auto-launch, 401 refresh retries, and offline mock transport.

use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;

// ============================================================================
// 1. Strongly-Typed Error Hierarchy
// ============================================================================

/// Errors emitted by the Atlas SDK.
#[derive(Error, Debug)]
pub enum AtlasError {
    /// Authentication or authorization failure (e.g. invalid token, HTTP 401).
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Network connectivity or unreachable endpoint error.
    #[error("Connection error: {0}")]
    Connection(String),

    /// Top-level JSON-RPC protocol error returned by the server.
    #[error("JSON-RPC error ({code}): {message}")]
    Rpc {
        code: i64,
        message: String,
        data: Option<serde_json::Value>,
    },

    /// Business execution error emitted by an Atlas tool (`result.isError == true`).
    #[error("Atlas tool '{tool_name}' failed: {message}")]
    ToolError { tool_name: String, message: String },

    /// Non-success HTTP status code returned by the endpoint.
    #[error("HTTP error ({0}): {1}")]
    Http(u16, String),

    /// HTTP request or client error.
    #[error("HTTP client error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Serde JSON serialization or deserialization failure.
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Filesystem or process I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Standard Result type alias for Atlas SDK operations.
pub type Result<T> = std::result::Result<T, AtlasError>;

// ============================================================================
// 2. Parameter Normalization
// ============================================================================

/// Normalizes tool arguments to conform to MCP schema expectations.
///
/// Supplies empty strings `""` for unset optional enum filters and `0` for numeric filters,
/// while stripping null fields.
pub fn normalize_params(tool: &str, value: serde_json::Value) -> serde_json::Value {
    let mut map = match value {
        serde_json::Value::Object(m) => m,
        _ => serde_json::Map::new(),
    };

    match tool {
        "search_items" => {
            map.entry("sort").or_insert_with(|| serde_json::json!(""));
            map.entry("source").or_insert_with(|| serde_json::json!(""));
            map.entry("orientation")
                .or_insert_with(|| serde_json::json!(""));
            map.entry("kind").or_insert_with(|| serde_json::json!(""));
            map.entry("order").or_insert_with(|| serde_json::json!(""));
        }
        "create_smart_folder" | "update_smart_folder" => {
            map.entry("match").or_insert_with(|| serde_json::json!(""));
        }
        "search_by_description" => {
            map.entry("kind").or_insert_with(|| serde_json::json!(""));
            map.entry("min_score")
                .or_insert_with(|| serde_json::json!(0.0));
        }
        "get_file" => {
            map.entry("size").or_insert_with(|| serde_json::json!(0));
        }
        "show_in_app" => {
            map.entry("view").or_insert_with(|| serde_json::json!(""));
        }
        "get_settings" => {
            map.entry("section")
                .or_insert_with(|| serde_json::json!(""));
        }
        "open_settings" => {
            map.entry("page").or_insert_with(|| serde_json::json!(""));
        }
        _ => {}
    }

    // ! Remove explicit null fields to ensure clean JSON serialization over JSON-RPC
    map.retain(|_, v| !v.is_null());

    serde_json::Value::Object(map)
}

// ============================================================================
// 3. Multi-Library Token Resolver
// ============================================================================

/// Options for configuring token discovery and library-specific keys.
#[derive(Debug, Clone, Default)]
pub struct TokenResolverOptions {
    /// Explicit Bearer token.
    pub token: Option<String>,
    /// Table mapping library paths or names to explicit Bearer tokens.
    pub library_tokens: HashMap<String, String>,
    /// Whether to enable automatic filesystem and CLI token discovery.
    pub auto_discover: bool,
}

/// Resolves active authentication tokens with multi-library routing and disk discovery.
#[derive(Debug)]
pub struct TokenResolver {
    explicit_token: Option<String>,
    library_tokens: RwLock<HashMap<String, String>>,
    auto_discover: bool,
    current_library: RwLock<Option<String>>,
    cached_token: RwLock<Option<String>>,
}

impl TokenResolver {
    /// Create a new TokenResolver with options.
    pub fn new(options: TokenResolverOptions) -> Self {
        Self {
            explicit_token: options.token.map(|s| s.trim().to_string()),
            library_tokens: RwLock::new(options.library_tokens),
            auto_discover: options.auto_discover,
            current_library: RwLock::new(None),
            cached_token: RwLock::new(None),
        }
    }

    /// Update active library path, optionally registering its token.
    pub fn set_active_library(&self, name_or_path: &str, token: Option<&str>) {
        if let Ok(mut current) = self.current_library.write() {
            *current = Some(name_or_path.to_string());
        }
        if let Some(t) = token {
            if let Ok(mut tokens) = self.library_tokens.write() {
                tokens.insert(name_or_path.to_string(), t.trim().to_string());
            }
        }
        self.clear_cache();
    }

    /// Clear in-memory token cache to force re-discovery.
    pub fn clear_cache(&self) {
        if let Ok(mut cached) = self.cached_token.write() {
            *cached = None;
        }
    }

    /// Resolve an active Bearer token according to multi-library priority rules.
    pub async fn resolve_token(&self, force_refresh: bool) -> Result<String> {
        if !force_refresh {
            if let Ok(cached) = self.cached_token.read() {
                if let Some(ref t) = *cached {
                    return Ok(t.clone());
                }
            }
        }

        // * Check active library specific token
        let current_lib = self.current_library.read().ok().and_then(|c| c.clone());
        if let Some(ref lib) = current_lib {
            if let Ok(tokens) = self.library_tokens.read() {
                if let Some(t) = tokens.get(lib) {
                    self.set_cached(t);
                    return Ok(t.clone());
                }
            }

            // ? If current_library is a filesystem path, check <path>/.mcp-token directly
            if self.auto_discover {
                let token_file = format!("{lib}/.mcp-token");
                if let Ok(t) = tokio::fs::read_to_string(&token_file).await {
                    let trimmed = t.trim().to_string();
                    if !trimmed.is_empty() {
                        self.set_cached(&trimmed);
                        return Ok(trimmed);
                    }
                }
            }
        }

        // * Check explicit default token
        if let Some(ref t) = self.explicit_token {
            self.set_cached(t);
            return Ok(t.clone());
        }

        // * Check ATLAS_MCP_TOKEN environment variable
        if let Ok(env_token) = std::env::var("ATLAS_MCP_TOKEN") {
            let trimmed = env_token.trim().to_string();
            if !trimmed.is_empty() {
                self.set_cached(&trimmed);
                return Ok(trimmed);
            }
        }

        // * Perform dynamic discovery from CLI and user directories
        if self.auto_discover {
            if let Some(discovered) = self.discover_local_token().await {
                self.set_cached(&discovered);
                return Ok(discovered);
            }
        }

        // ! No valid token found
        Err(AtlasError::Auth(
            "No Atlas MCP token found. Specify token in options, set ATLAS_MCP_TOKEN, or ensure an Atlas library with .mcp-token is open.".to_string(),
        ))
    }

    fn set_cached(&self, token: &str) {
        if let Ok(mut cached) = self.cached_token.write() {
            *cached = Some(token.to_string());
        }
    }

    /// Inspect active library path via 'atlas status' and dynamically scan library folders.
    async fn discover_local_token(&self) -> Option<String> {
        // * 1. Try 'atlas status' CLI commands
        for cmd in ["/opt/homebrew/bin/atlas", "atlas"] {
            if let Ok(output) = tokio::process::Command::new(cmd)
                .arg("status")
                .output()
                .await
            {
                if output.status.success() {
                    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                        if let Some(lib_path) = json
                            .get("library")
                            .and_then(|l| l.get("path"))
                            .and_then(|p| p.as_str())
                        {
                            // Check registered library tokens
                            if let Ok(tokens) = self.library_tokens.read() {
                                if let Some(t) = tokens.get(lib_path) {
                                    return Some(t.clone());
                                }
                            }
                            // Read .mcp-token from disk
                            let token_path = format!("{lib_path}/.mcp-token");
                            if let Ok(t) = tokio::fs::read_to_string(&token_path).await {
                                let trimmed = t.trim().to_string();
                                if !trimmed.is_empty() {
                                    return Some(trimmed);
                                }
                            }
                        }
                    }
                }
            }
        }

        // * 2. Dynamically scan Pictures and Documents for .atlaslibrary / .atlas bundles
        if let Ok(home) = std::env::var("HOME") {
            for base_dir_name in ["Pictures", "Documents"] {
                let base_path = format!("{home}/{base_dir_name}");
                let path = Path::new(&base_path);
                if let Ok(mut entries) = tokio::fs::read_dir(path).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.ends_with(".atlaslibrary") || name.ends_with(".atlas") {
                            let token_file = entry.path().join(".mcp-token");
                            if let Ok(token_text) = tokio::fs::read_to_string(&token_file).await {
                                let trimmed = token_text.trim().to_string();
                                if !trimmed.is_empty() {
                                    return Some(trimmed);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

// ============================================================================
// 4. AtlasTransport Trait
// ============================================================================

/// Common asynchronous contract for MCP transport layers.
pub trait AtlasTransport: Send + Sync {
    /// Execute an MCP tool call and return unwrapped JSON result.
    fn call_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> impl std::future::Future<Output = Result<serde_json::Value>> + Send;

    /// Health check ping to verify server connectivity.
    fn ping(&self) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Update active library on transport level.
    fn set_active_library(&self, _name_or_path: &str) {}
}

// ============================================================================
// 5. HttpAtlasTransport
// ============================================================================

/// Configuration options for HttpAtlasTransport.
#[derive(Debug, Clone)]
pub struct HttpTransportOptions {
    /// Endpoint URL (defaults to http://127.0.0.1:50473/mcp).
    pub endpoint: String,
    /// Explicit Bearer token.
    pub token: Option<String>,
    /// Library specific token mapping table.
    pub library_tokens: HashMap<String, String>,
    /// Whether to enable token discovery.
    pub auto_discover_token: bool,
    /// Automatically launch Atlas on macOS if connection is refused.
    pub auto_launch: bool,
    /// Request timeout duration.
    pub timeout: Duration,
}

impl Default for HttpTransportOptions {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:50473/mcp".to_string(),
            token: None,
            library_tokens: HashMap::new(),
            auto_discover_token: true,
            auto_launch: true,
            timeout: Duration::from_secs(15),
        }
    }
}

/// Production HTTP transport connecting to local Atlas MCP server.
#[derive(Debug)]
pub struct HttpAtlasTransport {
    endpoint: String,
    auto_launch: bool,
    client: reqwest::Client,
    token_resolver: Arc<TokenResolver>,
    rpc_id: AtomicI64,
}

impl HttpAtlasTransport {
    /// Create a new HttpAtlasTransport with options.
    pub fn new(options: HttpTransportOptions) -> Self {
        let resolver = TokenResolver::new(TokenResolverOptions {
            token: options.token,
            library_tokens: options.library_tokens,
            auto_discover: options.auto_discover_token,
        });

        let client = reqwest::Client::builder()
            .timeout(options.timeout)
            .build()
            .unwrap_or_default();

        Self {
            endpoint: options.endpoint,
            auto_launch: options.auto_launch,
            client,
            token_resolver: Arc::new(resolver),
            rpc_id: AtomicI64::new(1),
        }
    }

    /// Access the underlying TokenResolver.
    pub fn token_resolver(&self) -> &Arc<TokenResolver> {
        &self.token_resolver
    }

    /// Update active library on token resolver.
    pub fn set_active_library(&self, name_or_path: &str, token: Option<&str>) {
        self.token_resolver.set_active_library(name_or_path, token);
    }

    /// Send a raw JSON-RPC 2.0 POST request.
    async fn execute_raw_rpc(
        &self,
        method: &str,
        params: serde_json::Value,
        token: Option<&str>,
    ) -> Result<reqwest::Response> {
        let id = self.rpc_id.fetch_add(1, Ordering::SeqCst);
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let mut req = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("Connection", "close")
            .json(&body);

        if let Some(t) = token {
            req = req.header("Authorization", format!("Bearer {t}"));
        }

        req.send().await.map_err(|e| {
            AtlasError::Connection(format!(
                "Failed to connect to Atlas MCP server at {}: {}",
                self.endpoint, e
            ))
        })
    }

    /// Silent background launch of Atlas application on macOS with token-authenticated ping polling.
    async fn wake_atlas_app(&self, token: Option<&str>) -> Result<()> {
        let _ = tokio::process::Command::new("open")
            .args(["-g", "-a", "Atlas"])
            .spawn()
            .map_err(|e| AtlasError::Connection(format!("Failed to launch Atlas: {e}")))?;

        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(5);

        while start.elapsed() < timeout {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let active_token = match token {
                Some(t) => Some(t.to_string()),
                None => self.token_resolver.resolve_token(false).await.ok(),
            };
            if let Ok(res) = self
                .execute_raw_rpc("ping", serde_json::json!({}), active_token.as_deref())
                .await
            {
                if res.status().is_success() {
                    return Ok(());
                }
            }
        }

        Err(AtlasError::Connection(
            "Timeout waiting for Atlas to launch (5s timeout).".to_string(),
        ))
    }

    /// Unwrap MCP content payload into appropriate JSON format.
    pub fn unpack_mcp_content(
        &self,
        tool_name: &str,
        content: &[serde_json::Value],
    ) -> serde_json::Value {
        if content.is_empty() {
            return if tool_name == "view_items" {
                serde_json::json!({ "content": [] })
            } else {
                serde_json::json!({})
            };
        }

        // * For view_items, always preserve the { content } array structure to conform to ViewItemsResult
        if tool_name == "view_items" {
            return serde_json::json!({ "content": content });
        }

        // * Single text item
        if content.len() == 1 {
            let first = &content[0];
            if first.get("type").and_then(|t| t.as_str()) == Some("text") {
                if let Some(text) = first.get("text").and_then(|t| t.as_str()) {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
                        return parsed;
                    }
                    return serde_json::json!(text);
                }
            }
            if first.get("type").and_then(|t| t.as_str()) == Some("image") {
                return serde_json::json!({
                    "type": "image",
                    "data": first.get("data").unwrap_or(&serde_json::json!("")),
                    "mimeType": first.get("mimeType").unwrap_or(&serde_json::json!("image/jpeg")),
                });
            }
        }

        // * Mixed text and image (e.g. get_file)
        if content.len() == 2 {
            let text_part = content
                .iter()
                .find(|c| c.get("type").and_then(|t| t.as_str()) == Some("text"));
            let image_part = content
                .iter()
                .find(|c| c.get("type").and_then(|t| t.as_str()) == Some("image"));

            if let (Some(text_val), Some(img_val)) = (text_part, image_part) {
                let mut map = match text_val.get("text").and_then(|t| t.as_str()) {
                    Some(s) => serde_json::from_str::<serde_json::Value>(s)
                        .ok()
                        .and_then(|v| v.as_object().cloned())
                        .unwrap_or_default(),
                    None => serde_json::Map::new(),
                };

                map.insert(
                    "image".to_string(),
                    serde_json::json!({
                        "type": "image",
                        "data": img_val.get("data").unwrap_or(&serde_json::json!("")),
                        "mimeType": img_val.get("mimeType").unwrap_or(&serde_json::json!("image/jpeg")),
                    }),
                );
                return serde_json::Value::Object(map);
            }
        }

        serde_json::json!({ "content": content })
    }
}

impl AtlasTransport for HttpAtlasTransport {
    fn set_active_library(&self, name_or_path: &str) {
        self.token_resolver.set_active_library(name_or_path, None);
    }

    async fn ping(&self) -> Result<bool> {
        let token = self.token_resolver.resolve_token(false).await.ok();
        match self
            .execute_raw_rpc("ping", serde_json::json!({}), token.as_deref())
            .await
        {
            Ok(res) => Ok(res.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    async fn call_tool(&self, name: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        let normalized_args = normalize_params(name, args);
        let token = self.token_resolver.resolve_token(false).await?;

        let params = serde_json::json!({
            "name": name,
            "arguments": normalized_args,
        });

        // * Try sending request with auto-launch if connection fails
        let mut response = match self
            .execute_raw_rpc("tools/call", params.clone(), Some(&token))
            .await
        {
            Ok(res) => res,
            Err(first_err) => {
                if !self.auto_launch {
                    return Err(first_err);
                }
                self.wake_atlas_app(Some(&token)).await?;
                self.execute_raw_rpc("tools/call", params.clone(), Some(&token))
                    .await?
            }
        };

        // ! Handle HTTP 401 Unauthorized: clear cache, refresh token, retry once
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            self.token_resolver.clear_cache();
            let refreshed_token = self.token_resolver.resolve_token(true).await?;
            let retry_res = self
                .execute_raw_rpc("tools/call", params, Some(&refreshed_token))
                .await?;

            if retry_res.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(AtlasError::Auth(
                    "Atlas authentication failed (HTTP 401). Access token is invalid or expired."
                        .to_string(),
                ));
            }
            response = retry_res;
        }

        let status = response.status();
        if !status.is_success() {
            let err_text = response.text().await.unwrap_or_default();
            return Err(AtlasError::Http(status.as_u16(), err_text));
        }

        let json: serde_json::Value = response.json().await?;

        // ! Check top-level JSON-RPC protocol error
        if let Some(err_obj) = json.get("error") {
            let code = err_obj.get("code").and_then(|c| c.as_i64()).unwrap_or(-1);
            let message = err_obj
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown RPC error")
                .to_string();
            return Err(AtlasError::Rpc {
                code,
                message,
                data: err_obj.get("data").cloned(),
            });
        }

        let result = json.get("result");
        if let Some(res) = result {
            // ! Check tool-level business error
            if res.get("isError").and_then(|e| e.as_bool()) == Some(true) {
                let first_text = res
                    .get("content")
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|item| item.get("text"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Atlas tool error")
                    .to_string();
                return Err(AtlasError::ToolError {
                    tool_name: name.to_string(),
                    message: first_text,
                });
            }

            if let Some(content_arr) = res.get("content").and_then(|c| c.as_array()) {
                return Ok(self.unpack_mcp_content(name, content_arr));
            }
        }

        if name == "view_items" {
            Ok(serde_json::json!({ "content": [] }))
        } else {
            Ok(serde_json::json!({}))
        }
    }
}

// ============================================================================
// 6. MockAtlasTransport
// ============================================================================

type MockHandlerFn = Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value> + Send + Sync>;

/// In-memory mock transport for deterministic, isolated unit testing.
#[derive(Clone)]
pub struct MockAtlasTransport {
    handlers: Arc<RwLock<HashMap<String, MockHandlerFn>>>,
    call_history: Arc<RwLock<Vec<(String, serde_json::Value)>>>,
}

impl Default for MockAtlasTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MockAtlasTransport {
    /// Create a new MockAtlasTransport.
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            call_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a mock response callback for a tool.
    pub fn mock<F>(&self, tool: &str, handler: F)
    where
        F: Fn(serde_json::Value) -> Result<serde_json::Value> + Send + Sync + 'static,
    {
        if let Ok(mut map) = self.handlers.write() {
            map.insert(tool.to_string(), Box::new(handler));
        }
    }

    /// Clear recorded call history.
    pub fn clear_history(&self) {
        if let Ok(mut hist) = self.call_history.write() {
            hist.clear();
        }
    }

    /// Retrieve clone of recorded call history.
    pub fn history(&self) -> Vec<(String, serde_json::Value)> {
        self.call_history
            .read()
            .map(|h| h.clone())
            .unwrap_or_default()
    }
}

impl AtlasTransport for MockAtlasTransport {
    async fn ping(&self) -> Result<bool> {
        Ok(true)
    }

    async fn call_tool(&self, name: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        let normalized = normalize_params(name, args);
        if let Ok(mut hist) = self.call_history.write() {
            hist.push((name.to_string(), normalized.clone()));
        }

        let handlers = self.handlers.read().map_err(|_| {
            AtlasError::Connection("Failed to acquire lock on mock handlers".into())
        })?;

        if let Some(handler) = handlers.get(name) {
            handler(normalized)
        } else {
            Err(AtlasError::ToolError {
                tool_name: name.to_string(),
                message: format!("No mock handler registered for tool '{name}'"),
            })
        }
    }
}
