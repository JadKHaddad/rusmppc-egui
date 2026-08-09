use std::str::FromStr;

use eframe::egui::{self, Color32, RichText};
use rusmpp::{pdus::QuerySm, types::COctetString};
use serde::{Deserialize, Serialize};
use strum::VariantArray;

use crate::{
    actions::ActionsChannel,
    colors::HIGH_BLUE,
    result::{AppResult, AppUiError},
    values::{Npi, Ton},
    widgets::ComboBox,
};

struct RusmppFields {
    message_id: AppResult<COctetString<1, 65>>,
    source_addr: AppResult<COctetString<1, 21>>,
    query_sm: AppResult<QuerySm>,
}

impl RusmppFields {
    fn new(message_id: &str, source_addr: &str) -> Self {
        Self {
            message_id: COctetString::from_str(message_id)
                .map_err(|_| AppUiError::invalid_message_id()),
            source_addr: COctetString::from_str(source_addr)
                .map_err(|_| AppUiError::invalid_source_addr()),
            query_sm: Ok(QuerySm::default()),
        }
    }

    fn set_message_id(&mut self, message_id: &str) {
        self.message_id =
            COctetString::from_str(message_id).map_err(|_| AppUiError::invalid_message_id());
    }

    fn set_source_addr(&mut self, source_addr: &str) {
        self.source_addr =
            COctetString::from_str(source_addr).map_err(|_| AppUiError::invalid_source_addr());
    }

    fn all_fields_valid(&self) -> bool {
        matches!((&self.message_id, &self.source_addr), (Ok(_), Ok(_)))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SerdeQuerySmApp {
    message_id: String,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: String,
}

pub struct QuerySmApp {
    actions: ActionsChannel,
    message_id: String,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: String,
    fields: RusmppFields,
    bound: bool,
}

impl QuerySmApp {
    pub fn new_from_values(
        actions: ActionsChannel,
        message_id: String,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: String,
    ) -> Self {
        let fields = RusmppFields::new(&message_id, &source_addr);

        let mut app = Self {
            actions,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            fields,
            bound: false,
        };

        app.update_query_sm();

        app
    }

    pub fn new_default(actions: ActionsChannel) -> Self {
        let message_id = String::new();
        let source_addr_ton = Ton::default();
        let source_addr_npi = Npi::default();
        let source_addr = String::new();

        Self::new_from_values(
            actions,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        )
    }

    pub fn from_serde(actions: ActionsChannel, serde_app: SerdeQuerySmApp) -> Self {
        Self::new_from_values(
            actions,
            serde_app.message_id,
            serde_app.source_addr_ton,
            serde_app.source_addr_npi,
            serde_app.source_addr,
        )
    }

    pub fn to_serde(&self) -> SerdeQuerySmApp {
        SerdeQuerySmApp {
            message_id: self.message_id.clone(),
            source_addr_ton: self.source_addr_ton,
            source_addr_npi: self.source_addr_npi,
            source_addr: self.source_addr.clone(),
        }
    }

    fn update_query_sm(&mut self) {
        self.fields.query_sm = self.build_query_sm();
    }

    fn update_message_id(&mut self) {
        self.message_id.retain(|c| c.is_ascii());
        self.fields.set_message_id(&self.message_id);

        self.update_query_sm();
    }

    fn update_source_addr(&mut self) {
        self.source_addr.retain(|c| c.is_ascii());
        self.fields.set_source_addr(&self.source_addr);

        self.update_query_sm();
    }

    fn build_query_sm(&self) -> AppResult<QuerySm> {
        let query_sm = QuerySm::builder()
            .message_id(self.fields.message_id.clone()?)
            .source_addr_ton(self.source_addr_ton.into())
            .source_addr_npi(self.source_addr_npi.into())
            .source_addr(self.fields.source_addr.clone()?)
            .build();

        Ok(query_sm)
    }

    pub fn set_bound(&mut self, bound: bool) {
        self.bound = bound;
    }

    fn on_query_button_clicked(&mut self) {
        // We build the QuerySm every time the button is clicked to prevent building it every time a field is updated.
        if let Ok(query_sm) = self.build_query_sm() {
            self.actions.query_sm(query_sm);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered(|ui| {
            let display_err = |ui: &mut egui::Ui, err: &AppUiError| {
                ui.allocate_space(egui::vec2(0.0, 0.0));
                ui.colored_label(crate::colors::FUSION_RED, err.display_message());
                ui.end_row();
            };

            egui::Grid::new("query_sm_grid")
                .num_columns(2)
                .spacing([12.0, 10.0])
                .striped(false)
                .show(ui, |ui| {
                    ui.label("Source Address TON");
                    ui.add(ComboBox::new(
                        "query_sm_source_addr_ton",
                        &mut self.source_addr_ton,
                        Ton::VARIANTS,
                    ));
                    ui.end_row();

                    ui.label("Source Address NPI");
                    ui.add(ComboBox::new(
                        "query_sm_source_addr_npi",
                        &mut self.source_addr_npi,
                        Npi::VARIANTS,
                    ));
                    ui.end_row();

                    ui.label("Source Address");
                    ui.add(egui::TextEdit::singleline(&mut self.source_addr).char_limit(20))
                        .on_hover_text("Max 20 ASCII characters")
                        .changed()
                        .then(|| self.update_source_addr());

                    ui.end_row();

                    if let Err(err) = &self.fields.source_addr {
                        display_err(ui, err);
                    }

                    ui.label("Message ID");
                    ui.add(egui::TextEdit::multiline(&mut self.message_id).char_limit(64))
                        .on_hover_text("Max 64 ASCII characters")
                        .changed()
                        .then(|| {
                            self.update_message_id();
                        });
                    ui.end_row();

                    if let Err(err) = &self.fields.message_id {
                        display_err(ui, err);
                    }
                });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_enabled_ui(self.bound && self.fields.all_fields_valid(), |ui| {
                        ui.add_sized(
                            [140.0, 32.0],
                            egui::Button::new(
                                RichText::new("Query SM").color(Color32::WHITE).strong(),
                            )
                            .fill(HIGH_BLUE),
                        )
                        .clicked()
                        .then(|| {
                            self.on_query_button_clicked();
                        });
                    });
                });
            });
        })
        .response
    }
}
