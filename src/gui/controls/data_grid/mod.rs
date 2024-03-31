use iced::{widget::{Column, Row, Text}, Padding};
use og_lib_cdi::data::cdi_sector::CdiSector;

use crate::gui::app_messages::AppMessage;

pub enum ColumnType {
    Text(String),
    Numeric(i32),
    Checkbox(bool),
}

pub trait DataGridDisplay {
    fn columns() -> Vec<String>;
    fn values(&self) -> Vec<ColumnType>;
}

pub struct DataGrid<T: DataGridDisplay> {
    pub items: Vec<T>,
    // Other state and properties as needed, like selection state, etc.
}

impl<T: DataGridDisplay> DataGrid<T> {
    pub fn view(&self) -> iced::Element<'static, AppMessage> {
      
        let column_names = T::columns();
        let mut grid = Column::new();

        // Create a header row
        let header = Row::new().push(Text::new(column_names.join("  |  "))); // Simplified; you might want a more complex layout
        grid = grid.push(header);
        for item in &self.items {
            let mut row = Row::new();
            for value in item.values() {
                let cell = match value {
                    ColumnType::Text(text) => Text::new(text),
                    ColumnType::Numeric(number) => Text::new(number.to_string()).into(),
                    ColumnType::Checkbox(is_checked) => {
                      // use string till we have a checkbox control
                      Text::new(if is_checked { "Y" } else { "N" }).into()
                    },
                };
                row = row.push(cell);
            }
            grid = grid.push(row.padding(5));
        }
        grid.into()
    }
}

impl DataGridDisplay for CdiSector {
    fn columns() -> Vec<String> {
        vec![
            "Index".to_string(),
            "Sector Type".to_string(),
            "File #".to_string(),
            "Channel #".to_string(),
            "Is EOR".to_string(),
            "Is EOF".to_string(),
            "Is ASCF".to_string(),
            "Video Type".to_string(),
            "Resolution".to_string(),
            "Audio Type".to_string(),
            "BPS".to_string(),
            "Frequency".to_string(),
            "Is Form 2".to_string(),
            "Trigger".to_string(),
            "Realtime".to_string(),
        ]
    }

    fn values(&self) -> Vec<ColumnType> {
        vec![
            ColumnType::Numeric(self.sector_index() as i32),
            ColumnType::Text(format!("{}", self.get_sector_type_string())),
            ColumnType::Numeric(self.file_number() as i32),
            ColumnType::Numeric(self.channel_number() as i32),
            ColumnType::Checkbox(self.submode_info.is_eor()),
            ColumnType::Checkbox(self.submode_info.is_eof()),
            ColumnType::Checkbox(self.coding_info.is_ascf()),
            ColumnType::Text(format!("{}", self.coding_info.video_string())),
            ColumnType::Text(format!("{}", self.coding_info.resolution())),
            ColumnType::Text(format!("{}", match self.coding_info.is_mono() {
                true => "Mono",
                false => "Stereo",
                
            })),
            ColumnType::Text(format!("{}", self.coding_info.bits_per_sample_string())),
            ColumnType::Text(format!("{}", self.coding_info.sample_rate_string())),
            ColumnType::Checkbox(self.submode_info.is_form2()),
            ColumnType::Checkbox(self.submode_info.is_trigger()),
            ColumnType::Checkbox(self.submode_info.is_rtf()),
            
        ]
    }
}
