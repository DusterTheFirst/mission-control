use iced::{button, container};

pub struct ControlCluster;

impl container::StyleSheet for ControlCluster {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: colors::BORDER,
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
            text_color: colors::TEXT,
            background: colors::ACTIVE.into(),
            border_radius: 3.0,
            ..Default::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: colors::HOVERED.into(),
            text_color: colors::TEXT,
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            border_width: 1.0,
            border_color: colors::TEXT,
            ..self.hovered()
        }
    }
}

pub struct Instrument;

impl container::StyleSheet for Instrument {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: colors::BORDER,
            border_width: 1.0,
            border_radius: 10.0,
            background: colors::SURFACE.into(),
            text_color: colors::TEXT.into(),
        }
    }
}

pub mod colors {
    use iced::Color;

    pub const TEXT: Color = Color::from_rgb(
        0xEE as f32 / 255.0,
        0xEE as f32 / 255.0,
        0xEE as f32 / 255.0,
    );

    pub const BORDER: Color = Color::from_rgb(
        0x25 as f32 / 255.0,
        0x25 as f32 / 255.0,
        0x25 as f32 / 255.0,
    );

    pub const SURFACE: Color = Color::from_rgb(
        0x1C as f32 / 255.0,
        0x1C as f32 / 255.0,
        0x1C as f32 / 255.0,
    );

    pub const BACKGROUND: Color = Color::from_rgb(
        0x16 as f32 / 255.0,
        0x16 as f32 / 255.0,
        0x16 as f32 / 255.0,
    );

    pub const ACCENT: Color = Color::from_rgb(
        0x6F as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xE9 as f32 / 255.0,
    );

    pub const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    pub const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );
}
