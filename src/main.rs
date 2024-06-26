use gui::app_messages::{AppMessage, Page};
use gui::layout::render_layout;
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
use og_lib_cdi::data::cdi_file::CdiFile;
mod gui;

static ICON: &[u8] = include_bytes!("./assets/icon.png");
const ICON_WIDTH: u32 = 256;
const ICON_HEIGHT: u32 = 256;

pub fn main() -> iced::Result {
    let image = image::load_from_memory(ICON).unwrap();
    let icon = iced::window::icon::from_rgba(image.as_bytes().to_vec(), ICON_WIDTH, ICON_HEIGHT).unwrap();
    
    let settings = iced::settings::Settings {
        window: iced::window::Settings {
            icon: Some(icon), 
            ..Default::default()
        },
        ..Default::default()
    };
    App::run(settings)
}

#[derive(Clone)]
struct App {
    // Your application state goes here
    pub cdi_file: Option<CdiFile>,
    current_page: Page,
    theme: Theme,
    //active_pages: Vec<Page>,
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
                // initially, no pages are active
                //active_pages: Vec::new(),
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
                        // Load the cdi file if it hasn't been loaded yet
                        if self.cdi_file.is_none() {
                            let cdi_file = CdiFile::new("C:/Dev/Projects/Gaming/CD-i/Disc Images/Extracted/Plunderball/Intro.rtr".to_string());
                            self.cdi_file = Some(cdi_file);
                        }
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
