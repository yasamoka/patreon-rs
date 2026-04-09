//! Patreon API data models.
//!
//! Patreon uses the JSON:API format. Responses generally follow this structure:
//! - `data`: primary resource data
//! - `included`: related resources
//! - `links`: pagination links
//! - `meta`: metadata

mod attributes;
mod resources;
mod response;

pub use attributes::*;
pub use resources::*;
pub use response::*;
