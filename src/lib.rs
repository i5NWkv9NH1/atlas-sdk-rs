//! # atlas-sdk
//!
//! Strongly-typed local Model Context Protocol (MCP) SDK for the macOS **Atlas** visual library app (Unofficial).
//!
//! Built for **Rust 2024 Edition** with zero runtime macro overhead.

#![allow(clippy::collapsible_if)]

pub mod ingest;
pub mod media;
pub mod models;
pub mod organize;
pub mod query;
pub mod transport;
pub mod ui;

use std::sync::Arc;

pub use ingest::{IngestBuilder, IngestResult};
pub use models::*;
pub use transport::{
    AtlasError, AtlasTransport, HttpAtlasTransport, HttpTransportOptions, MockAtlasTransport,
    Result, TokenResolver, TokenResolverOptions, normalize_params,
};

/// High-level strongly-typed client for interacting with the Atlas MCP server.
///
/// Generic over the transport implementation (defaults to `HttpAtlasTransport`).
#[derive(Clone, Debug)]
pub struct AtlasClient<T: AtlasTransport = HttpAtlasTransport> {
    transport: Arc<T>,
}

impl Default for AtlasClient<HttpAtlasTransport> {
    fn default() -> Self {
        Self::new()
    }
}

impl AtlasClient<HttpAtlasTransport> {
    /// Create a new AtlasClient with default local HTTP transport.
    pub fn new() -> Self {
        Self {
            transport: Arc::new(HttpAtlasTransport::new(HttpTransportOptions::default())),
        }
    }

    /// Create an AtlasClient with custom HTTP transport options.
    pub fn with_options(options: HttpTransportOptions) -> Self {
        Self {
            transport: Arc::new(HttpAtlasTransport::new(options)),
        }
    }
}

impl<T: AtlasTransport> AtlasClient<T> {
    /// Create an AtlasClient wrapping a custom transport (e.g. `MockAtlasTransport`).
    pub fn with_transport(transport: T) -> Self {
        Self {
            transport: Arc::new(transport),
        }
    }

    /// Access the underlying transport implementation.
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Create a fluent, chainable builder for one-shot compound media ingestion.
    pub fn ingest(&self) -> IngestBuilder<'_, T> {
        IngestBuilder::new(self)
    }

    /// Server health check ping.
    pub async fn ping(&self) -> Result<bool> {
        self.transport.ping().await
    }

    /// List available tools and their schemas from the Atlas MCP server.
    pub async fn list_tools(&self) -> Result<crate::models::McpListToolsResult> {
        self.transport.list_tools().await
    }

    /// List available resources from the Atlas MCP server.
    pub async fn list_resources(&self) -> Result<crate::models::McpListResourcesResult> {
        self.transport.list_resources().await
    }

    /// Read the content of an MCP resource by URI (e.g. `atlas://skill` or `atlas://item/<id>`).
    pub async fn read_resource(&self, uri: &str) -> Result<crate::models::McpReadResourceResult> {
        self.transport.read_resource(uri).await
    }
}
