use eframe::AppCreator;
use serde::{Deserialize, Serialize};

use crate::{
    actions::{Action, ActionsChannel},
    apps::{SerdeTabs, Tabs},
    background::BackgroundApp,
    state::AppState,
    widgets::BindIndicator,
};

#[derive(Serialize, Deserialize)]
struct SerdeApp {
    tabs: SerdeTabs,
}

pub struct App {
    state: AppState,
    tabs: Tabs,
    version: &'static str,
}

impl App {
    fn new_from_values(cc: &eframe::CreationContext<'_>, state: AppState, tabs: Tabs) -> Self {
        egui_material_icons::initialize(&cc.egui_ctx);

        Self {
            tabs,
            state,
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn new_default(
        cc: &eframe::CreationContext<'_>,
        state: AppState,
        actions: ActionsChannel,
    ) -> Self {
        let tabs = Tabs::new_default(state.holder(), actions);

        Self::new_from_values(cc, state, tabs)
    }

    fn from_serde(
        cc: &eframe::CreationContext<'_>,
        state: AppState,
        actions: ActionsChannel,
        serde_app: SerdeApp,
    ) -> Self {
        let tabs = Tabs::from_serde(state.holder(), actions, serde_app.tabs);

        Self::new_from_values(cc, state, tabs)
    }

    fn to_serde(&self) -> SerdeApp {
        SerdeApp {
            tabs: self.tabs.to_serde(),
        }
    }

    fn load_or_default(
        cc: &eframe::CreationContext<'_>,
        state: AppState,
        actions: ActionsChannel,
    ) -> Self {
        cc.storage
            .and_then(|storage| eframe::get_value::<SerdeApp>(storage, eframe::APP_KEY))
            .map(|serde_app| Self::from_serde(cc, state.clone(), actions.clone(), serde_app))
            .unwrap_or(Self::new_default(cc, state, actions))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn creator() -> AppCreator<'static> {
        Box::new(|cc| {
            let state = AppState::new(cc.egui_ctx.clone());

            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Action>();
            let actions = ActionsChannel::new(tx);

            let background_app = BackgroundApp::new(state.clone());

            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to build tokio runtime");

                runtime.block_on(background_app.run(rx));
            });

            Ok(Box::new(App::load_or_default(cc, state, actions)))
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn creator() -> AppCreator<'static> {
        Box::new(|cc| {
            let state = AppState::new(cc.egui_ctx.clone());

            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Action>();
            let actions = ActionsChannel::new(tx);

            let background_app = BackgroundApp::new(state.clone());

            wasm_bindgen_futures::spawn_local(background_app.run(rx));

            Ok(Box::new(App::load_or_default(cc, state, actions)))
        })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_| {
            let bound = self.state.is_bound();

            self.tabs.set_bound(bound);

            egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin::same(2))
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(self.version);
                            ui.add_space(ui.available_width() - BindIndicator::size().x * 3.0);
                            egui::widgets::global_theme_preference_switch(ui);
                            ui.add(BindIndicator::new(bound));
                        });
                    });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                self.tabs.ui(ctx, ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.to_serde());
    }
}
