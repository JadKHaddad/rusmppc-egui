use egui::{Stroke, WidgetText};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use serde::{Deserialize, Serialize};

use crate::{
    actions::ActionsChannel,
    apps::{BindApp, LogsApp, SerdeBindApp, SerdeLogsApp, SerdeSubmitSmApp, SubmitSmApp},
    state::EventsHolder,
};

#[derive(Clone, Serialize, Deserialize)]
enum SerdeTab {
    Bind(SerdeBindApp),
    SubmitSm(SerdeSubmitSmApp),
    Logs(SerdeLogsApp),
}

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

    fn from_serde(actions: ActionsChannel, events_holder: EventsHolder, serde: SerdeTab) -> Self {
        match serde {
            SerdeTab::Bind(serde) => Tab::Bind(BindApp::from_serde(actions, serde)),
            SerdeTab::SubmitSm(serde) => Tab::SubmitSm(SubmitSmApp::from_serde(actions, serde)),
            SerdeTab::Logs(serde) => Tab::Logs(LogsApp::from_serde(events_holder, serde)),
        }
    }

    fn to_serde(&self) -> SerdeTab {
        match self {
            Tab::Bind(app) => SerdeTab::Bind(app.to_serde()),
            Tab::SubmitSm(app) => SerdeTab::SubmitSm(app.to_serde()),
            Tab::Logs(app) => SerdeTab::Logs(app.to_serde()),
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

#[derive(Serialize, Deserialize)]
pub struct SerdeTabs {
    dock_state: DockState<SerdeTab>,
}

pub struct Tabs {
    pub dock_state: DockState<Tab>,
}

impl Tabs {
    pub fn new_default(events_holder: EventsHolder, actions: ActionsChannel) -> Self {
        let mut dock_state = DockState::new(vec![Tab::Bind(BindApp::new_default(actions.clone()))]);

        let [a, _] = dock_state.main_surface_mut().split_below(
            NodeIndex::root(),
            0.6,
            vec![Tab::Logs(LogsApp::new_default(events_holder))],
        );

        let [_, _] = dock_state.main_surface_mut().split_right(
            a,
            0.3,
            vec![Tab::SubmitSm(SubmitSmApp::new_default(actions))],
        );

        Self { dock_state }
    }

    pub fn from_serde(
        events_holder: EventsHolder,
        actions: ActionsChannel,
        serde: SerdeTabs,
    ) -> Self {
        let dock_state = serde
            .dock_state
            .map_tabs(|tab| Tab::from_serde(actions.clone(), events_holder.clone(), tab.clone()));

        Self { dock_state }
    }

    pub fn to_serde(&self) -> SerdeTabs {
        let dock_state = self.dock_state.map_tabs(|tab| tab.to_serde());

        SerdeTabs { dock_state }
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
