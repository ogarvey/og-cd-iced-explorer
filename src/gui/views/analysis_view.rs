use iced::{
    widget::{container, row, Column, Row, Text},
    Element,
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

    row!(video_container, audio_container, data_container)
        .height(100)
        .padding(10)
        .spacing(10)
        .into()
}
