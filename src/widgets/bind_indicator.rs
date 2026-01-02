use crate::colors::{FUSION_RED, REPTILE_GREEN};

pub struct BindIndicator(bool);

impl BindIndicator {
    pub const fn new(connected: bool) -> Self {
        Self(connected)
    }

    pub const fn size() -> egui::Vec2 {
        egui::vec2(12.0, 12.0)
    }
}

impl egui::Widget for BindIndicator {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(Self::size(), egui::Sense::hover());

        let color = if self.0 { REPTILE_GREEN } else { FUSION_RED };

        ui.painter().circle_filled(rect.center(), 6.0, color);

        response
    }
}
