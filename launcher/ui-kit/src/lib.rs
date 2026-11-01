//! Scoria's component kit: semantic tokens, one theme, HeroUI-shaped components. It knows nothing
//! about the launcher engine, the file system or the network, so it can be tested and reused.
pub mod components;
pub mod tokens;

pub use components::*;
pub use tokens::{Colors, Radii, Spacing, Theme, Typography};

/// The intent behind a control, before it is mapped to a concrete component variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}
