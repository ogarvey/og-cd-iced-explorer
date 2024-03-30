use crate::gui::app_messages::{AppMessage, Page};
use iced::widget::{column, text};
use iced::Element;
use crate::App;
use super::controls::navigation::render_nav;
use super::views::analysis_view::{ AnalysisView, render_analysis_view};

pub fn render_layout(app: App) -> Element<'static, AppMessage> {
   
    let nav = render_nav(app.current_page);
    match app.current_page {
      Page::Analysis => {
          let cdi_file = app.cdi_file.clone().unwrap();
          let analysis_view = AnalysisView::new(cdi_file);
          return column![nav,render_analysis_view(&analysis_view)].padding(10).into();
      },
      Page::Main => {
        return column![nav, text("Main Page")].padding(10).into();
      },
      Page::ImageManagement => {
        return column![nav, text("Image Management Page")].padding(10).into();
      },
      Page::AudioManagement => {
        return column![nav, text("Audio Management Page")].padding(10).into();
      }
      Page::Settings => {
        return column![nav, text("Settings Page")].padding(10).into();
      }
    };
        
}
