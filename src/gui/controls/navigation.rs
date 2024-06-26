
use iced::widget::{button, horizontal_space, row, text, Row};
use iced::{Alignment, Element};

use crate::gui::app_messages::{AppMessage, Page};
use crate::gui::theme::CyanButtonStyleSheet;

pub fn render_nav(current_page: Page) -> Element<'static, AppMessage> {
    let buttons = [
        ("Main", Page::Main),
        ("Analysis", Page::Analysis),
        ("Image Management", Page::ImageManagement),
        ("Audio Management", Page::AudioManagement),
        ("Settings", Page::Settings),
    ];

    let content = buttons.iter().map(|(label, page)| {
        if *page == current_page {
            button(text(label))
            .style(CyanButtonStyleSheet::new())
                .padding(10)
                .into()
        } else {
            button(text(label))
            .style(CyanButtonStyleSheet::new())
                .padding(10)
                .on_press(AppMessage::ChangePage(*page))
                .into()
        }
    });

    let button_row = Row::with_children(content)
        .spacing(20)
        .align_items(Alignment::Center);
    row![button_row, horizontal_space()]
        .padding(10)
        .spacing(20)
        .into()
}

