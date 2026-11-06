//! One file per screen. They all add methods to `Shell` in `app`, so a screen can use the
//! launcher state without the window passing a dozen arguments around.

mod chrome;
mod home;
mod mods;
mod render;
mod servers;
mod settings;
