use egui::{Stroke, WidgetText};
use egui_dock::{DockArea, DockState, Style};

use crate::{actions::ActionsChannel, apps::SubmitSmApp};

enum Tab {
    SubmitSm(SubmitSmApp),
}

impl Tab {
    const fn title(&self) -> &str {
        match self {
            Tab::SubmitSm(_) => "Submit Sm",
        }
    }

    fn set_bound(&mut self, bound: bool) {
        match self {
            Tab::SubmitSm(app) => app.set_bound(bound),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        match self {
            Tab::SubmitSm(app) => app.ui(ui),
        }
    }
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        egui::Frame::new()
            .inner_margin(egui::Margin::same(16))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        tab.ui(ui);
                    });
            });
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [false, false]
    }
}

pub struct Tabs {
    dock_state: DockState<Tab>,
}

impl Tabs {
    pub fn new(actions: ActionsChannel) -> Self {
        let tabs = [Tab::SubmitSm(SubmitSmApp::new(actions))]
            .into_iter()
            .collect();

        let dock_state = DockState::new(tabs);

        Self { dock_state }
    }

    pub fn set_bound(&mut self, bound: bool) {
        for (_, node) in self.dock_state.iter_leaves_mut() {
            node.tabs_mut().iter_mut().for_each(|tab| {
                tab.set_bound(bound);
            });
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let mut style = Style::from_egui(ui.style());
        style.tab.tab_body.stroke = Stroke::NONE;

        DockArea::new(&mut self.dock_state)
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .draggable_tabs(false)
            .style(style)
            .show(ctx, &mut TabViewer);
    }
}
