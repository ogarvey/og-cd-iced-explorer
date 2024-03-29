#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Main,
    Analysis,
    ImageManagement,
    AudioManagement,
    Settings,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ChangePage(Page),
    // Add more messages here
}
