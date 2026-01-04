use crate::{
    colors::HIGH_BLUE,
    widgets::{BlinkerHandle, blinker::Blinker},
};

use egui::Label;
use egui_material_icons::{
    icon_text,
    icons::{ICON_NORTH, ICON_SOUTH},
};

pub struct EventIndicator {
    blinker: Blinker,
    icon: &'static str,
}

impl EventIndicator {
    fn new(
        blink_for: std::time::Duration,
        icon: &'static str,
        ctx: egui::Context,
    ) -> (Self, BlinkerHandle) {
        let (blinker, handle) = Blinker::new(blink_for, ctx);

        (Self { blinker, icon }, handle)
    }

    pub fn incoming(blink_for: std::time::Duration, ctx: egui::Context) -> (Self, BlinkerHandle) {
        Self::new(blink_for, ICON_SOUTH, ctx)
    }

    pub fn outgoing(blink_for: std::time::Duration, ctx: egui::Context) -> (Self, BlinkerHandle) {
        Self::new(blink_for, ICON_NORTH, ctx)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let blink = self.blinker.blink(ui);

        let color = if blink {
            HIGH_BLUE
        } else {
            ui.visuals().widgets.inactive.bg_fill
        };

        ui.add(Label::new(icon_text(self.icon).color(color)).selectable(false))
    }
}
