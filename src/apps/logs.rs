use egui_virtual_list::VirtualList;

use crate::state::EventsHolder;

pub struct LogsApp {
    events_holder: EventsHolder,
    list: VirtualList,
}

impl LogsApp {
    pub fn new(events_holder: EventsHolder) -> Self {
        Self {
            events_holder,
            list: VirtualList::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let events = self.events_holder.events();
        let len = events.len();

        ui.set_width(ui.available_width());

        self.list.ui_custom_layout(ui, len, |ui, start_index| {
            let index = len - 1 - start_index;

            if let Some(event) = events.get(index) {
                ui.push_id(index, |ui| {
                    egui::Frame::group(ui.style())
                        .inner_margin(egui::Margin::same(8))
                        .corner_radius(egui::CornerRadius::same(6))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.label(format!("{event:?}"));
                        });
                });
            }

            1
        });
    }
}
