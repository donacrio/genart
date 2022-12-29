mod dynamic_artwork;
mod nannou_app;
mod static_artwork;

pub use dynamic_artwork::{make_dynamic_artwork, update_dynamic, DynamicArtwork};
pub use nannou_app::{BaseModel, NannouApp, NannouAppOptions};
pub use static_artwork::{make_static_artwork, update_static, StaticApp};
