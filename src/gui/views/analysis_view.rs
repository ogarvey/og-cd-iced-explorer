use iced::{
    widget::{row, Scrollable},
    Element, Length,
};
use og_lib_cdi::data::{cdi_file::CdiFile, CdiSectorType};

use crate::gui::{app_messages::AppMessage, controls::analysis::cdifile_overview::CdiFileOverview};

pub struct AnalysisView {
    cdi_file: CdiFile,
}

impl AnalysisView {
    pub fn new(cdi_file: CdiFile) -> Self {
        Self { cdi_file }
    }
}

pub fn render_analysis_view(analysis_view: &AnalysisView) -> Element<'static, AppMessage> {
    let cdi_file = &analysis_view.cdi_file;
    let data_sectors = cdi_file.get_data_sectors();
    let audio_sectors = cdi_file.get_audio_sectors();
    let video_sectors = cdi_file.get_video_sectors();

    let all_sectors = cdi_file.sectors();

    let data_sector_count = data_sectors.len();
    let audio_sector_count = audio_sectors.len();
    let video_sector_count = video_sectors.len();

    let data_overview = CdiFileOverview::new(CdiSectorType::Data, data_sector_count);
    let audio_overview = CdiFileOverview::new(CdiSectorType::Audio, audio_sector_count);
    let video_overview = CdiFileOverview::new(CdiSectorType::Video, video_sector_count);

    let data_container =
        crate::gui::controls::analysis::cdifile_overview::render_overview(data_overview);
    let audio_container =
        crate::gui::controls::analysis::cdifile_overview::render_overview(audio_overview);
    let video_container =
        crate::gui::controls::analysis::cdifile_overview::render_overview(video_overview);

    let overview_row = row!(video_container, audio_container, data_container)
        .height(100);

    let sector_rows = all_sectors.iter().map(|sector| {
        let sector_type = match sector.get_sector_type() {
            CdiSectorType::Data => "Data",
            CdiSectorType::Audio => "Audio",
            CdiSectorType::Video => "Video",
            CdiSectorType::Empty => "Empty",
            CdiSectorType::Message => "Message",
        };
        let sector_text = format!("{} Sector", sector_type);
        let sector_text = iced::widget::Text::new(sector_text).size(20);
        let sector_number = iced::widget::Text::new(format!(" {}", sector.sector_index())).size(20);
        row![sector_text, sector_number].into()
    });

    let sector_rows = iced::widget::Column::with_children(sector_rows).spacing(10);
    
    let column = iced::widget::Column::with_children(vec![overview_row.into(), sector_rows.into()])
        .padding(10);
    // display in scrollable
    Scrollable::new(column).width(Length::Fill).into()
}
