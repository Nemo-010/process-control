//! The app's colours and egui style.
//!
//! A palette is a set of background, text and accent colours. The app ships
//! a few: its own, and ones that follow the Adwaita and Breeze themes so it
//! sits naturally next to GTK and Qt programs on the same desktop.

use eframe::egui::{self, Color32, Stroke};

/// An RGB colour.
type Rgb = (u8, u8, u8);

/// One set of colours for the window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Palette {
    /// Window and panel background.
    pub background: Rgb,
    /// Background of raised areas, such as windows and menus.
    pub panel: Rgb,
    /// Background of the graph strips and other recessed areas.
    pub recess: Rgb,
    /// Background of empty icons and other placeholders.
    pub placeholder: Rgb,
    /// Inactive widget fill.
    pub widget: Rgb,
    /// Widget fill under the pointer.
    pub hovered: Rgb,
    /// Widget fill while pressed.
    pub active: Rgb,
    /// Border of windows and separators.
    pub border: Rgb,
    /// Body text.
    pub text: Color32,
    /// Secondary text, such as the PID column.
    pub weak_text: Color32,
    /// The colour that marks the selected row and the graphs.
    pub accent: Color32,
    /// Whether the palette is light, which decides egui's own defaults.
    pub light: bool,
}

impl Palette {
    /// The app's own palette: a blue-black background with a mint accent.
    ///
    /// It is the default, and the one the icon is drawn for.
    pub const fn default_theme() -> Self {
        Self {
            background: (0x0c, 0x12, 0x18),
            panel: (0x13, 0x1b, 0x24),
            recess: (0x08, 0x0d, 0x12),
            placeholder: (0x1a, 0x24, 0x2f),
            widget: (0x23, 0x2f, 0x3c),
            hovered: (0x2c, 0x3a, 0x49),
            active: (0x35, 0x46, 0x57),
            border: (0x23, 0x2e, 0x3a),
            text: Color32::from_rgb(0xe4, 0xec, 0xf3),
            weak_text: Color32::from_rgb(0x8e, 0x9f, 0xb0),
            accent: Color32::from_rgb(0x4c, 0xd8, 0xa4),
            light: false,
        }
    }

    /// Adwaita dark, the GNOME theme.
    pub const fn adwaita_dark() -> Self {
        Self {
            background: (0x24, 0x24, 0x24),
            panel: (0x30, 0x30, 0x30),
            recess: (0x1e, 0x1e, 0x1e),
            placeholder: (0x3a, 0x3a, 0x3a),
            widget: (0x40, 0x40, 0x40),
            hovered: (0x4a, 0x4a, 0x4a),
            active: (0x55, 0x55, 0x55),
            border: (0x1b, 0x1b, 0x1b),
            text: Color32::from_rgb(0xff, 0xff, 0xff),
            weak_text: Color32::from_rgb(0xb0, 0xb0, 0xb0),
            accent: Color32::from_rgb(0x35, 0x84, 0xe4),
            light: false,
        }
    }

    /// Adwaita light, the GNOME theme in its light variant.
    pub const fn adwaita_light() -> Self {
        Self {
            background: (0xfa, 0xfa, 0xfa),
            panel: (0xf2, 0xf2, 0xf2),
            recess: (0xff, 0xff, 0xff),
            placeholder: (0xe8, 0xe8, 0xe8),
            widget: (0xe1, 0xe1, 0xe1),
            hovered: (0xd6, 0xd6, 0xd6),
            active: (0xc8, 0xc8, 0xc8),
            border: (0xdc, 0xdc, 0xdc),
            text: Color32::from_rgb(0x2e, 0x2e, 0x2e),
            weak_text: Color32::from_rgb(0x6d, 0x6d, 0x6d),
            accent: Color32::from_rgb(0x35, 0x84, 0xe4),
            light: true,
        }
    }

    /// Breeze dark, the KDE Plasma theme.
    pub const fn breeze_dark() -> Self {
        Self {
            background: (0x1b, 0x1e, 0x20),
            panel: (0x27, 0x2a, 0x2d),
            recess: (0x14, 0x16, 0x18),
            placeholder: (0x32, 0x36, 0x39),
            widget: (0x3b, 0x40, 0x43),
            hovered: (0x45, 0x4a, 0x4e),
            active: (0x50, 0x55, 0x59),
            border: (0x11, 0x13, 0x15),
            text: Color32::from_rgb(0xfc, 0xfc, 0xfc),
            weak_text: Color32::from_rgb(0xa8, 0xad, 0xb0),
            accent: Color32::from_rgb(0x3d, 0xae, 0xe9),
            light: false,
        }
    }

    /// Breeze light, the KDE Plasma theme in its light variant.
    pub const fn breeze_light() -> Self {
        Self {
            background: (0xef, 0xf0, 0xf1),
            panel: (0xf7, 0xf8, 0xf9),
            recess: (0xff, 0xff, 0xff),
            placeholder: (0xe3, 0xe5, 0xe6),
            widget: (0xdc, 0xdf, 0xe1),
            hovered: (0xd0, 0xd4, 0xd7),
            active: (0xc4, 0xc9, 0xcc),
            border: (0xcb, 0xcf, 0xd2),
            text: Color32::from_rgb(0x23, 0x26, 0x29),
            weak_text: Color32::from_rgb(0x5c, 0x62, 0x66),
            accent: Color32::from_rgb(0x3d, 0xae, 0xe9),
            light: true,
        }
    }

    /// Nord, the arctic blue palette.
    pub const fn nord() -> Self {
        Self {
            background: (0x2e, 0x34, 0x40),
            panel: (0x3b, 0x42, 0x52),
            recess: (0x27, 0x2c, 0x36),
            placeholder: (0x43, 0x4c, 0x5e),
            widget: (0x43, 0x4c, 0x5e),
            hovered: (0x4c, 0x56, 0x6a),
            active: (0x5a, 0x65, 0x7d),
            border: (0x20, 0x24, 0x2c),
            text: Color32::from_rgb(0xec, 0xef, 0xf4),
            weak_text: Color32::from_rgb(0xa7, 0xb1, 0xc2),
            accent: Color32::from_rgb(0x88, 0xc0, 0xd0),
            light: false,
        }
    }

    /// Catppuccin Mocha, the dark flavour.
    pub const fn catppuccin_mocha() -> Self {
        Self {
            background: (0x1e, 0x1e, 0x2e),
            panel: (0x18, 0x18, 0x25),
            recess: (0x11, 0x11, 0x1b),
            placeholder: (0x31, 0x32, 0x44),
            widget: (0x31, 0x32, 0x44),
            hovered: (0x45, 0x47, 0x5a),
            active: (0x58, 0x5b, 0x70),
            border: (0x11, 0x11, 0x1b),
            text: Color32::from_rgb(0xcd, 0xd6, 0xf4),
            weak_text: Color32::from_rgb(0xa6, 0xad, 0xc8),
            accent: Color32::from_rgb(0x89, 0xb4, 0xfa),
            light: false,
        }
    }

    /// Catppuccin Latte, the light flavour.
    pub const fn catppuccin_latte() -> Self {
        Self {
            background: (0xef, 0xf1, 0xf5),
            panel: (0xe6, 0xe9, 0xef),
            recess: (0xdc, 0xe0, 0xe8),
            placeholder: (0xcc, 0xd0, 0xda),
            widget: (0xcc, 0xd0, 0xda),
            hovered: (0xbc, 0xc0, 0xcc),
            active: (0xac, 0xb0, 0xbe),
            border: (0xbc, 0xc0, 0xcc),
            text: Color32::from_rgb(0x4c, 0x4f, 0x69),
            weak_text: Color32::from_rgb(0x6c, 0x6f, 0x85),
            accent: Color32::from_rgb(0x1e, 0x66, 0xf5),
            light: true,
        }
    }

    /// Solarized dark, built on base03 and base02.
    pub const fn solarized_dark() -> Self {
        Self {
            background: (0x00, 0x2b, 0x36),
            panel: (0x07, 0x36, 0x42),
            recess: (0x00, 0x1e, 0x26),
            placeholder: (0x0d, 0x4b, 0x5c),
            widget: (0x07, 0x36, 0x42),
            hovered: (0x0e, 0x51, 0x62),
            active: (0x15, 0x5a, 0x6b),
            border: (0x00, 0x25, 0x2e),
            text: Color32::from_rgb(0x93, 0xa1, 0xa1),
            weak_text: Color32::from_rgb(0x65, 0x7b, 0x83),
            accent: Color32::from_rgb(0x26, 0x8b, 0xd2),
            light: false,
        }
    }

    /// Solarized light, built on base3 and base2.
    pub const fn solarized_light() -> Self {
        Self {
            background: (0xfd, 0xf6, 0xe3),
            panel: (0xee, 0xe8, 0xd5),
            recess: (0xff, 0xfb, 0xf0),
            placeholder: (0xe4, 0xdd, 0xc9),
            widget: (0xee, 0xe8, 0xd5),
            hovered: (0xe0, 0xd9, 0xc6),
            active: (0xd3, 0xcc, 0xb8),
            border: (0xdd, 0xd6, 0xc1),
            text: Color32::from_rgb(0x65, 0x7b, 0x83),
            weak_text: Color32::from_rgb(0x93, 0xa1, 0xa1),
            accent: Color32::from_rgb(0x26, 0x8b, 0xd2),
            light: true,
        }
    }
}

/// A palette the user can pick, with the name shown in the View menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Theme {
    /// The app's own palette.
    #[default]
    Default,
    /// Adwaita, light.
    AdwaitaLight,
    /// Adwaita, dark.
    AdwaitaDark,
    /// Breeze, light.
    BreezeLight,
    /// Breeze, dark.
    BreezeDark,
    /// Nord, the arctic blue palette.
    Nord,
    /// Catppuccin Mocha, the dark flavour.
    CatppuccinMocha,
    /// Catppuccin Latte, the light flavour.
    CatppuccinLatte,
    /// Solarized, dark.
    SolarizedDark,
    /// Solarized, light.
    SolarizedLight,
}

impl Theme {
    /// Every theme, in the order the View menu lists them.
    pub const ALL: [Theme; 10] = [
        Theme::Default,
        Theme::AdwaitaLight,
        Theme::AdwaitaDark,
        Theme::BreezeLight,
        Theme::BreezeDark,
        Theme::Nord,
        Theme::CatppuccinMocha,
        Theme::CatppuccinLatte,
        Theme::SolarizedDark,
        Theme::SolarizedLight,
    ];

    /// The name shown in the menu.
    pub fn name(self) -> &'static str {
        match self {
            Theme::Default => "Default",
            Theme::AdwaitaLight => "Adwaita Light",
            Theme::AdwaitaDark => "Adwaita Dark",
            Theme::BreezeLight => "Breeze Light",
            Theme::BreezeDark => "Breeze Dark",
            Theme::Nord => "Nord",
            Theme::CatppuccinMocha => "Catppuccin Mocha",
            Theme::CatppuccinLatte => "Catppuccin Latte",
            Theme::SolarizedDark => "Solarized Dark",
            Theme::SolarizedLight => "Solarized Light",
        }
    }

    /// The colours of this theme.
    pub fn palette(self) -> Palette {
        match self {
            Theme::Default => Palette::default_theme(),
            Theme::AdwaitaLight => Palette::adwaita_light(),
            Theme::AdwaitaDark => Palette::adwaita_dark(),
            Theme::BreezeLight => Palette::breeze_light(),
            Theme::BreezeDark => Palette::breeze_dark(),
            Theme::Nord => Palette::nord(),
            Theme::CatppuccinMocha => Palette::catppuccin_mocha(),
            Theme::CatppuccinLatte => Palette::catppuccin_latte(),
            Theme::SolarizedDark => Palette::solarized_dark(),
            Theme::SolarizedLight => Palette::solarized_light(),
        }
    }

    /// The theme with this name, as the settings file stores it.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|theme| theme.name().eq_ignore_ascii_case(name))
    }

    /// The theme a desktop session should start on.
    ///
    /// Only KDE changes the answer: its programs look wrong in the app's own
    /// palette next to a Breeze window, so the KDE session gets Breeze Dark.
    /// Every other desktop keeps the app's own dark theme, which is the one
    /// the icon was drawn for.
    pub fn for_desktop(session: &str) -> Self {
        if session.to_ascii_lowercase().contains("kde") {
            Self::BreezeDark
        } else {
            Self::Default
        }
    }

    /// The theme a first launch starts on, read from the desktop.
    ///
    /// `XDG_CURRENT_DESKTOP` names the desktop, which is what decides the
    /// palette. This only decides the first run: once the settings hold a
    /// theme, that choice stands.
    pub fn first_run() -> Self {
        Self::for_desktop(&std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default())
    }
}

/// Applies a palette and the type scale to a context.
///
/// Both of egui's theme styles get the same palette, so the system theme
/// cannot change how the window looks: the chosen theme decides.
pub fn apply(ctx: &egui::Context, theme: Theme) {
    let palette = theme.palette();
    let light = palette.light;
    let mut style = (*ctx.style_of(if light {
        egui::Theme::Light
    } else {
        egui::Theme::Dark
    }))
    .clone();
    let visuals = &mut style.visuals;

    *visuals = if light {
        egui::Visuals::light()
    } else {
        egui::Visuals::dark()
    };
    let border = Color32::from_rgb(palette.border.0, palette.border.1, palette.border.2);
    visuals.panel_fill = Color32::from_rgb(
        palette.background.0,
        palette.background.1,
        palette.background.2,
    );
    // Dropdown menus float over the window, so they get the raised colour.
    visuals.window_fill = Color32::from_rgb(palette.panel.0, palette.panel.1, palette.panel.2);
    visuals.extreme_bg_color =
        Color32::from_rgb(palette.recess.0, palette.recess.1, palette.recess.2);
    visuals.faint_bg_color = Color32::from_rgb(
        palette.placeholder.0,
        palette.placeholder.1,
        palette.placeholder.2,
    );
    visuals.window_stroke = Stroke::new(1.0, border);
    visuals.override_text_color = Some(palette.text);
    visuals.weak_text_color = Some(palette.weak_text);
    visuals.hyperlink_color = palette.accent;

    visuals.widgets.noninteractive.bg_fill =
        Color32::from_rgb(palette.panel.0, palette.panel.1, palette.panel.2);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border);
    visuals.widgets.inactive.bg_fill =
        Color32::from_rgb(palette.widget.0, palette.widget.1, palette.widget.2);
    visuals.widgets.inactive.weak_bg_fill =
        Color32::from_rgb(palette.panel.0, palette.panel.1, palette.panel.2);
    visuals.widgets.hovered.bg_fill =
        Color32::from_rgb(palette.hovered.0, palette.hovered.1, palette.hovered.2);
    visuals.widgets.hovered.weak_bg_fill =
        Color32::from_rgb(palette.hovered.0, palette.hovered.1, palette.hovered.2);
    visuals.widgets.active.bg_fill =
        Color32::from_rgb(palette.active.0, palette.active.1, palette.active.2);
    visuals.widgets.active.weak_bg_fill =
        Color32::from_rgb(palette.active.0, palette.active.1, palette.active.2);

    let accent = palette.accent;
    visuals.selection.bg_fill =
        Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 70);
    visuals.selection.stroke = Stroke::new(1.0, accent);

    style.spacing.item_spacing = egui::vec2(8.0, 4.0);
    style.spacing.button_padding = egui::vec2(10.0, 4.0);

    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(20.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(13.5)),
        (egui::TextStyle::Button, egui::FontId::proportional(13.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(11.5)),
        (egui::TextStyle::Monospace, egui::FontId::monospace(12.0)),
    ]
    .into();

    ctx.set_style_of(egui::Theme::Dark, style.clone());
    ctx.set_style_of(egui::Theme::Light, style);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_theme_round_trips_through_its_name() {
        for theme in Theme::ALL {
            assert_eq!(Theme::from_name(theme.name()), Some(theme));
        }
        assert_eq!(Theme::from_name("no such theme"), None);
    }

    #[test]
    fn the_default_palette_is_dark_and_blue() {
        let palette = Palette::default_theme();
        assert!(!palette.light);
        let (r, g, b) = palette.background;
        assert!(b > g && g > r, "the background leans blue: {r},{g},{b}");
    }

    #[test]
    fn the_light_palettes_are_lighter_than_the_dark_ones() {
        for theme in [Theme::AdwaitaLight, Theme::BreezeLight] {
            assert!(theme.palette().light, "{} is light", theme.name());
        }
        for theme in [Theme::Default, Theme::AdwaitaDark, Theme::BreezeDark] {
            assert!(!theme.palette().light, "{} is dark", theme.name());
        }
    }

    #[test]
    fn a_kde_desktop_starts_on_breeze_dark_and_the_rest_on_the_app_palette() {
        assert_eq!(Theme::for_desktop("KDE"), Theme::BreezeDark);
        assert_eq!(Theme::for_desktop("KDE:Plasma"), Theme::BreezeDark);
        assert_eq!(Theme::for_desktop("GNOME"), Theme::Default);
        assert_eq!(Theme::for_desktop(""), Theme::Default);
    }
}
