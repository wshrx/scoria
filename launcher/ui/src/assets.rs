//! Fonts, brand art and icons the launcher bundles. Original Scoria assets only.
/// The bundled OFL fonts: Medeo Sans for text, Medeo Mono for versions and numbers.
pub(crate) const FONTS: [&[u8]; 6] = [
    include_bytes!("../assets/fonts/MedeoSans-Regular.ttf"),
    include_bytes!("../assets/fonts/MedeoSans-Medium.ttf"),
    include_bytes!("../assets/fonts/MedeoSans-SemiBold.ttf"),
    include_bytes!("../assets/fonts/MedeoSans-Bold.ttf"),
    include_bytes!("../assets/fonts/MedeoMono-Regular.ttf"),
    include_bytes!("../assets/fonts/MedeoMono-Medium.ttf"),
];
pub(crate) const MARK: &[u8] = include_bytes!("../assets/scoria-mark.png");
pub(crate) const BACKDROP: &[u8] = include_bytes!("../assets/backdrop.jpg");
pub(crate) const BACKDROP_LIGHT: &[u8] = include_bytes!("../assets/backdrop-light.jpg");
pub(crate) const ICON_PLAY: &[u8] = include_bytes!("../assets/icons/play.svg");
pub(crate) const ICON_MODS: &[u8] = include_bytes!("../assets/icons/mods.svg");
pub(crate) const ICON_SERVERS: &[u8] = include_bytes!("../assets/icons/servers.svg");
pub(crate) const ICON_SETTINGS: &[u8] = include_bytes!("../assets/icons/settings.svg");
pub(crate) const ICON_SEARCH: &[u8] = include_bytes!("../assets/icons/search.svg");
pub(crate) const ICON_WIN_MIN: &[u8] = include_bytes!("../assets/icons/win-min.svg");
pub(crate) const ICON_WIN_MAX: &[u8] = include_bytes!("../assets/icons/win-max.svg");
pub(crate) const ICON_WIN_CLOSE: &[u8] = include_bytes!("../assets/icons/win-close.svg");
pub(crate) const ICON_MENU: &[u8] = include_bytes!("../assets/icons/menu.svg");
