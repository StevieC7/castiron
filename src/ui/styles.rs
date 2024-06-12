use iced::{
    border::Radius,
    theme,
    widget::{
        button::{Appearance as ButtonAppearance, StyleSheet},
        container::Appearance,
        rule::{Appearance as RuleAppearance, FillMode},
    },
    Background, Border, Color, Shadow, Theme, Vector,
};

pub fn style_main_area(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(Background::Color(palette.background.weak.color)),
        text_color: Some(palette.background.weak.text),
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

pub fn style_sidebar(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(Background::Color(palette.background.strong.color)),
        text_color: Some(palette.background.strong.text),
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

pub fn style_sidebar_rule(theme: &Theme) -> RuleAppearance {
    let palette = theme.extended_palette();
    RuleAppearance {
        color: palette.background.base.color,
        width: 3,
        radius: Radius::default(),
        fill_mode: FillMode::Full,
    }
}

pub fn style_player_area(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(iced::Background::Color(palette.background.base.color)),
        text_color: None,
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::default(),
        },
        shadow: Shadow::default(),
    }
}

pub fn style_list_item(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();
    Appearance {
        background: Some(iced::Background::Color(palette.background.strong.color)),
        text_color: None,
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: [5.0, 5.0, 5.0, 5.0].into(),
        },
        shadow: Shadow::default(),
    }
}

pub struct MyButtonStyle;
impl MyButtonStyle {
    pub fn new() -> Self {
        Self
    }
}
impl StyleSheet for MyButtonStyle {
    type Style = Theme;
    fn active(&self, style: &Self::Style) -> ButtonAppearance {
        ButtonAppearance {
            shadow: Shadow::default(),
            shadow_offset: Vector::default(),
            background: None,
            text_color: style.extended_palette().background.strong.text,
            border: Border::default(),
        }
    }
    fn hovered(&self, style: &Self::Style) -> ButtonAppearance {
        ButtonAppearance {
            background: Some(iced::Background::Color(
                style.extended_palette().background.weak.color,
            )),
            ..self.active(style)
        }
    }
}

impl Into<theme::Button> for MyButtonStyle {
    fn into(self) -> theme::Button {
        theme::Button::custom(self)
    }
}
