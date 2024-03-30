use iced::{border::Radius, widget::{column, container, text, vertical_space, Column, Text}, Alignment, Border, Color, Element};
use og_lib_cdi::data::CdiSectorType;

use crate::gui::app_messages::AppMessage;

pub struct CdiFileOverview {
  sector_type: CdiSectorType,
  sector_count: usize,
}

impl CdiFileOverview {
  pub fn new(sector_type: CdiSectorType, sector_count: usize) -> Self {
    Self {
      sector_type,
      sector_count,
    }
  }
}

pub fn render_overview(overview: CdiFileOverview) -> Element<'static,AppMessage> {
  let sector_type = match overview.sector_type {
    CdiSectorType::Data => "Data",
    CdiSectorType::Audio => "Audio",
    CdiSectorType::Video => "Video",
    CdiSectorType::Empty => "Empty",
    CdiSectorType::Message => "Message",
  };
  let sector_count = overview.sector_count;
  let sector_type_text = text(format!("{} Sectors", sector_type)).size(20);
  let sector_count_text = text(format!("{}", sector_count)).size(20);
  container(column![sector_type_text, vertical_space(), sector_count_text]
    .align_items(Alignment::Center)).padding(10).style(container::Appearance {
      background: Some(iced::Background::Color(Color::from_rgba8(0, 117, 117,255.0))),
      border: Border{ 
        color: Color::from_rgba8(0, 255, 255,255.0), //#007575
        width: 1.0,
        radius: Radius::default(),
      },
      text_color: Some(Color::WHITE),
      ..container::Appearance::default()
    }).into()
}
