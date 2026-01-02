use eframe::AppCreator;

use crate::{
    actions::{Action, ActionsChannel},
    apps::Tabs,
    background::BackgroundApp,
    state::AppState,
    widgets::BindIndicator,
};

pub struct App {
    state: AppState,
    tabs: Tabs,
    version: &'static str,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, state: AppState, actions: ActionsChannel) -> Self {
        egui_material_icons::initialize(&cc.egui_ctx);

        Self {
            tabs: Tabs::new(state.holder(), actions),
            state,
            version: env!("CARGO_PKG_VERSION"),
        }
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

            Ok(Box::new(App::new(cc, state, actions)))
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

            Ok(Box::new(App::new(cc, state, actions)))
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
                            ui.add_space(ui.available_width() - BindIndicator::size().x);
                            ui.add(BindIndicator::new(bound));
                        });
                    });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                self.tabs.ui(ctx, ui);
            });
        });
    }
}
