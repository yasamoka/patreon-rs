//! Patreon API response types.
//!
//! JSON:API response wrappers.

use serde::{Deserialize, Serialize};
use url::Url;

/// JSON:API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<D> {
    /// Primary data.
    pub data: D,
    /// Included related resources.
    pub included: Option<Vec<serde_json::Value>>,
    /// Pagination links.
    pub links: Option<PaginationLinks>,
    /// Metadata.
    pub meta: Option<serde_json::Value>,
}

/// Pagination links.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationLinks {
    /// First page.
    pub first: Option<Url>,
    /// Previous page.
    pub prev: Option<Url>,
    /// Next page.
    pub next: Option<Url>,
    /// Last page.
    pub last: Option<Url>,
    /// Current page.
    #[serde(rename = "self")]
    pub self_link: Option<Url>,
}

/// Pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationMeta {
    /// Total count.
    pub count: Option<i32>,
}

/// Single resource response.
pub type SingleResponse<T> = ApiResponse<T>;

/// List resource response.
pub type ListResponse<T> = ApiResponse<Vec<T>>;

/// API error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error list.
    pub errors: Vec<ApiErrorDetail>,
}

/// API error detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    /// Error code.
    pub code: Option<i32>,
    /// HTTP status code (string).
    pub status: Option<String>,
    /// Error title.
    pub title: Option<String>,
    /// Error detail.
    pub detail: Option<String>,
    /// Error code name.
    pub code_name: Option<String>,
    /// Error ID.
    pub id: Option<String>,
}
