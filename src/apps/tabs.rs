use egui::{Stroke, WidgetText};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::{
    actions::ActionsChannel,
    apps::{BindApp, LogsApp, SubmitSmApp},
    state::EventsHolder,
};

pub enum Tab {
    Bind(BindApp),
    SubmitSm(SubmitSmApp),
    Logs(LogsApp),
}

impl Tab {
    const fn title(&self) -> &str {
        match self {
            Tab::Bind(_) => "Bind",
            Tab::SubmitSm(_) => "Submit Sm",
            Tab::Logs(_) => "Logs",
        }
    }

    fn set_bound(&mut self, bound: bool) {
        match self {
            Tab::SubmitSm(app) => app.set_bound(bound),
            Tab::Bind(app) => app.set_bound(bound),
            Tab::Logs(_) => {}
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Tab::SubmitSm(app) => {
                app.ui(ui);
            }
            Tab::Bind(app) => {
                app.ui(ui);
            }
            Tab::Logs(app) => {
                app.ui(ui);
            }
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
                tab.ui(ui);
            });
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [true, true]
    }
}

pub struct Tabs {
    pub dock_state: DockState<Tab>,
}

impl Tabs {
    pub fn new(events_holder: EventsHolder, actions: ActionsChannel) -> Self {
        let mut dock_state = DockState::new(vec![Tab::Bind(BindApp::new(actions.clone()))]);

        let [a, _] = dock_state.main_surface_mut().split_below(
            NodeIndex::root(),
            0.6,
            vec![Tab::Logs(LogsApp::new(events_holder))],
        );

        let [_, _] = dock_state.main_surface_mut().split_right(
            a,
            0.3,
            vec![Tab::SubmitSm(SubmitSmApp::new(actions))],
        );

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
            .style(style)
            .show(ctx, &mut TabViewer);
    }
}
