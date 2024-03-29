use crate::gui::app_messages::{AppMessage, Page};
use iced::widget::{button, text, Column, Row};
use iced::Element;

use super::controls::navigation::render_nav;

pub fn render_layout(page: Page) -> Element<'static, AppMessage> {
    let nav = render_nav(page);
    let row: Row<AppMessage> = match page {
        Page::Main => {
            Row::new().push(text("Main page")).into()
        }
        Page::Analysis => {
            let text = text::Text::new("Analysis page");
            Row::new().push(text).into()
        }
        Page::ImageManagement => {
            let text = text::Text::new("Image management page");
            Row::new().push(text).into()
        }
        Page::AudioManagement => {
            let text = text::Text::new("Audio management page");
            Row::new().push(text).into()
        }
        Page::Settings => {
            let text = text::Text::new("Settings page");
            Row::new().push(text).into()
        }
    };
    Column::new().push(nav).push(row).into()
}
