use gui::app_messages::{AppMessage, Page};
use gui::controls::analysis;
use gui::layout::render_layout;
use iced::executor;
use iced::widget::{button, column, text, Column};
use iced::{Application, Command, Element, Settings, Theme};
use og_lib_cdi::data::cdi_file::CdiFile;
mod gui;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone)]
struct App {
    // Your application state goes here
    pub cdi_file: Option<CdiFile>,
    current_page: Page,
    theme: Theme,
}

impl App {
    pub fn get_cdi_file(&self) -> Option<CdiFile> {
        self.cdi_file.clone()
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = AppMessage;
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<AppMessage>) {
        (
            App {
                cdi_file: None,
                current_page: Page::Main,
                theme: Theme::Dark,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("OG CD-Iced Explorer")
    }

    fn update(&mut self, _message: AppMessage) -> Command<AppMessage> {
        match _message {
            AppMessage::ChangePage(page) => {
                match page {
                    Page::Analysis => {
                        // Load the cdi file
                        let cdi_file = CdiFile::new("C:/Dev/Projects/Gaming/CD-i/Disc Images/Extracted/Plunderball/Intro.rtr".to_string());
                        self.cdi_file = Some(cdi_file);
                        //render_layout(page);
                    }
                    _ => {}
                }
                self.current_page = page;
            }
        }
        Command::none()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn view(&self) -> Element<'_, AppMessage> {
        render_layout(self.clone())
    }
}
