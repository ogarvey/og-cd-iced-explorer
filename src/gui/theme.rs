use iced::{border::Radius, widget::button, Color};

pub struct CyanButtonStyleSheet;

impl CyanButtonStyleSheet {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}

impl button::StyleSheet for CyanButtonStyleSheet {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
      
        let background = iced::Background::Color(Color::from_rgba8(0, 117, 117,255.0));
        button::Appearance {
            background: Some(background),
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::from_rgba8(0, 255, 255,255.0),
                width: 1.0,
                radius: Radius::from(5.0),
            },
            ..Default::default()
        }
    }
}
