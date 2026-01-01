use eframe::AppCreator;

use crate::{
    actions::{Action, ActionsChannel},
    apps::{BindApp, SubmitSmApp},
    background::BackgroundApp,
    state::AppState,
};

pub struct App {
    state: AppState,
    bind_app: BindApp,
    submit_sm_app: SubmitSmApp,
    version: &'static str,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, state: AppState, actions: ActionsChannel) -> Self {
        egui_material_icons::initialize(&cc.egui_ctx);

        Self {
            state,
            bind_app: BindApp::new(actions.clone()),
            submit_sm_app: SubmitSmApp::new(actions),
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

            self.bind_app.set_bound(bound);
            self.submit_sm_app.set_bound(bound);

            egui::Window::new("Bind").resizable(false).show(ctx, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.add(&mut self.bind_app);
                    });
            });

            egui::Window::new("Submit SM")
                .resizable(false)
                .max_width(700.0)
                .show(ctx, |ui| {
                    egui::Frame::new()
                        .inner_margin(egui::Margin::same(16))
                        .show(ui, |ui| {
                            ui.add(&mut self.submit_sm_app);
                        });
                });

            egui::Window::new("Logs")
                .default_height(500.0)
                .resizable(true)
                .show(ctx, |ui| {
                    let events = self.state.events();
                    let len = events.len();

                    let row_height = egui::TextStyle::Body.resolve(ui.style()).size;

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false]) // <- critical
                        .show_rows(ui, row_height, len, |ui, row_range| {
                            // Claim width so the scroll area expands even when empty
                            ui.set_min_height(ui.available_height());

                            for row in row_range {
                                let index = len - 1 - row;
                                if let Some(event) = events.get(index) {
                                    egui::Frame::group(ui.style())
                                        .inner_margin(egui::Margin::same(8))
                                        .corner_radius(egui::CornerRadius::same(6))
                                        .show(ui, |ui| {
                                            ui.label(format!("{event:?}"));
                                        });
                                }
                            }
                        });
                });

            egui::TopBottomPanel::bottom("version_panel").show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(self.version);
                });
            });
        });
    }
}
