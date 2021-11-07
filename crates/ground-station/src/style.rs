use iced::{button, container};

pub struct ControlCluster;

impl container::StyleSheet for ControlCluster {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: colors::BORDER.into(),
            border_width: 1.0,
            border_radius: 10.0,
            background: colors::SURFACE.into(),
            text_color: colors::TEXT.into(),
        }
    }
}

impl button::StyleSheet for ControlCluster {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: colors::TEXT.into(),
            background: colors::ACTIVE.into(),
            border_radius: 3.0,
            ..Default::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: colors::HOVERED.into(),
            text_color: colors::TEXT.into(),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            border_width: 1.0,
            border_color: colors::TEXT.into(),
            ..self.hovered()
        }
    }
}

pub struct Instrument;

impl container::StyleSheet for Instrument {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: colors::BORDER.into(),
            border_width: 1.0,
            border_radius: 5.0,
            background: colors::SURFACE.into(),
            text_color: colors::TEXT.into(),
        }
    }
}

pub struct Tooltip;

impl container::StyleSheet for Tooltip {
    fn style(&self) -> container::Style {
        // TODO:
        container::Style {
            border_color: colors::BORDER.into(),
            border_width: 1.0,
            border_radius: 10.0,
            background: colors::SURFACE.into(),
            text_color: colors::TEXT.into(),
        }
    }
}


pub mod colors {
    pub const TEXT: Color = Color::from_rgb(0xEE, 0xEE, 0xEE);
    pub const BORDER: Color = Color::from_rgb(0x25, 0x25, 0x25);
    pub const SURFACE: Color = Color::from_rgb(0x16, 0x16, 0x16);
    pub const BACKGROUND: Color = Color::from_rgb(0x00, 0x00, 0x00);
    pub const ACCENT: Color = Color::from_rgb(0x6F, 0xFF, 0xE9);
    pub const ACTIVE: Color = Color::from_rgb(0x72, 0x89, 0xDA);
    pub const HOVERED: Color = Color::from_rgb(0x67, 0x7B, 0xC4);

    pub const GRID_LINES: Color = Color::from_rgb(0x45, 0x45, 0x45);
    pub const AXIS: Color = Color::from_rgb(0xEE, 0xEE, 0xEE);


    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    impl Color {
        pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b }
        }
    }

    impl From<Color> for iced::Color {
        fn from(Color { r, g, b }: Color) -> Self {
            iced::Color::from_rgb8(r, g, b)
        }
    }

    impl From<Color> for Option<iced::Color> {
        fn from(color: Color) -> Self {
            Some(iced::Color::from(color))
        }
    }

    impl From<Color> for Option<iced::Background> {
        fn from(color: Color) -> Self {
            Some(iced::Background::Color(iced::Color::from(color)))
        }
    }

    impl plotters_backend::BackendStyle for Color {
        fn color(&self) -> plotters_backend::BackendColor {
            plotters_backend::BackendColor {
                alpha: 1.0,
                rgb: (self.r, self.g, self.b),
            }
        }
    }
    impl plotters::style::Color for Color {}
}
