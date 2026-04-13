//! Patreon API data models.
//!
//! Patreon uses the JSON:API format. Responses generally follow this structure:
//! - `data`: primary resource data
//! - `included`: related resources
//! - `links`: pagination links
//! - `meta`: metadata

mod attributes;
mod fields;
mod included;
mod includes;
mod queries;
mod resources;
mod response;

pub use attributes::*;
pub use fields::*;
pub use included::*;
pub use includes::*;
pub use queries::*;
pub use resources::*;
pub use response::*;
