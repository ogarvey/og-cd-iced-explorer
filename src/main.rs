use gui::app_messages::{AppMessage, Page};
use gui::layout::render_layout;
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{button, column, text, Column};
use og_lib_cdi::data::cdi_file::CdiFile;
mod gui;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    // Your application state goes here
    cdi_file: Option<CdiFile>,
    current_page: Page
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = AppMessage;
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<AppMessage>) {
        (App { cdi_file: None, current_page: Page::Main}, Command::none())
    }

    fn title(&self) -> String {
        String::from("OG CD-Iced Explorer")
    }

    fn update(&mut self, _message: AppMessage) -> Command<AppMessage> {
        match _message {
            AppMessage::ChangePage(page) => {
                self.current_page = page;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, AppMessage>  {
        render_layout(self.current_page)
    }
}
