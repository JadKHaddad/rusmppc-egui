use std::str::FromStr;

use eframe::egui::{self, Color32, RichText, Separator};
use encoder::Encoder;
use rusmpp::{
    extra::{
        concatenation::SubmitSmMultipartExt,
        encoding::{gsm7bit::Gsm7BitUnpacked, latin1::Latin1, ucs2::Ucs2},
    },
    pdus::SubmitSm,
    types::COctetString,
    values::{EsmClass as RusmppEsmClass, ServiceType},
};
use serde::{Deserialize, Serialize};
use strum::VariantArray;

use crate::{
    actions::ActionsChannel,
    colors::{FUSION_RED, HIGH_BLUE},
    result::{AppResult, AppUiError, MultiPartError},
    values::{
        Ansi41Specific, DataCoding, EsmClass, GsmFeatures, MessageType, MessagingMode, Npi, Ton,
    },
    widgets::ComboBox,
};

mod encoder;

#[derive(Debug)]
struct RusmppFields {
    service_type: AppResult<COctetString<1, 6>>,
    source_addr: AppResult<COctetString<1, 21>>,
    destination_addr: AppResult<COctetString<1, 21>>,
    submit_sms: AppResult<Vec<SubmitSm>>,
}

impl RusmppFields {
    fn new(service_type: &str, source_addr: &str, destination_addr: &str) -> Self {
        Self {
            service_type: COctetString::from_str(service_type)
                .map_err(|_| AppUiError::invalid_service_type()),
            source_addr: COctetString::from_str(source_addr)
                .map_err(|_| AppUiError::invalid_source_addr()),
            destination_addr: COctetString::from_str(destination_addr)
                .map_err(|_| AppUiError::invalid_destination_addr()),
            submit_sms: Ok(Vec::new()),
        }
    }

    fn set_service_type(&mut self, service_type: &str) {
        self.service_type =
            COctetString::from_str(service_type).map_err(|_| AppUiError::invalid_service_type());
    }

    fn set_source_addr(&mut self, source_addr: &str) {
        self.source_addr =
            COctetString::from_str(source_addr).map_err(|_| AppUiError::invalid_source_addr());
    }

    fn set_destination_addr(&mut self, destination_addr: &str) {
        self.destination_addr = COctetString::from_str(destination_addr)
            .map_err(|_| AppUiError::invalid_destination_addr());
    }

    fn sms_valid_and_not_empty(&self) -> bool {
        match &self.submit_sms {
            Ok(sms) => {
                !sms.is_empty()
                    && sms
                        .iter()
                        .any(|submit_sm| !submit_sm.short_message().is_empty())
            }
            Err(_) => false,
        }
    }

    fn all_fields_valid(&self) -> bool {
        matches!(
            (
                &self.service_type,
                &self.source_addr,
                &self.destination_addr,
                self.sms_valid_and_not_empty()
            ),
            (Ok(_), Ok(_), Ok(_), true)
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SerdeSubmitSmApp {
    short_message: String,
    service_type: String,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: String,
    dest_addr_ton: Ton,
    dest_addr_npi: Npi,
    destination_addr: String,
    data_coding: DataCoding,
    esm_class: EsmClass,
    last_gsm_features: GsmFeatures,
}

pub struct SubmitSmApp {
    actions: ActionsChannel,
    short_message: String,
    service_type: String,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: String,
    dest_addr_ton: Ton,
    dest_addr_npi: Npi,
    destination_addr: String,
    data_coding: DataCoding,
    esm_class: EsmClass,
    last_gsm_features: GsmFeatures,
    reference: u8,
    fields: RusmppFields,
    bound: bool,
}

impl SubmitSmApp {
    #[allow(clippy::too_many_arguments)]
    pub fn new_from_values(
        actions: ActionsChannel,
        short_message: String,
        service_type: String,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: String,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: String,
        data_coding: DataCoding,
        esm_class: EsmClass,
        last_gsm_features: GsmFeatures,
    ) -> Self {
        let fields = RusmppFields::new(&service_type, &source_addr, &destination_addr);

        let mut app = Self {
            actions,
            short_message,
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            data_coding,
            esm_class,
            last_gsm_features,
            reference: 0,
            fields,
            bound: false,
        };

        app.update_short_message();

        app
    }

    pub fn new_default(actions: ActionsChannel) -> Self {
        let service_type = String::new();
        let source_addr = String::new();
        let source_addr_ton = Ton::default();
        let source_addr_npi = Npi::default();
        let destination_addr = String::new();
        let dest_addr_ton = Ton::default();
        let dest_addr_npi = Npi::default();
        let short_message = String::from("Hello Rusmppc!");
        let data_coding = DataCoding::default();
        let esm_class = EsmClass::default();
        let last_gsm_features = GsmFeatures::default();

        Self::new_from_values(
            actions,
            short_message,
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            data_coding,
            esm_class,
            last_gsm_features,
        )
    }

    pub fn from_serde(actions: ActionsChannel, serde_app: SerdeSubmitSmApp) -> Self {
        Self::new_from_values(
            actions,
            serde_app.short_message,
            serde_app.service_type,
            serde_app.source_addr_ton,
            serde_app.source_addr_npi,
            serde_app.source_addr,
            serde_app.dest_addr_ton,
            serde_app.dest_addr_npi,
            serde_app.destination_addr,
            serde_app.data_coding,
            serde_app.esm_class,
            serde_app.last_gsm_features,
        )
    }

    pub fn to_serde(&self) -> SerdeSubmitSmApp {
        SerdeSubmitSmApp {
            short_message: self.short_message.clone(),
            service_type: self.service_type.clone(),
            source_addr_ton: self.source_addr_ton,
            source_addr_npi: self.source_addr_npi,
            source_addr: self.source_addr.clone(),
            dest_addr_ton: self.dest_addr_ton,
            dest_addr_npi: self.dest_addr_npi,
            destination_addr: self.destination_addr.clone(),
            data_coding: self.data_coding,
            esm_class: self.esm_class,
            last_gsm_features: self.last_gsm_features,
        }
    }

    /// Creates the appropriate encoder on the fly based on the selected data coding.
    ///
    /// This is done like this, because we may want provide configuration options for each encoder in the future.
    /// For example, allowing the user to select different alphabets for GSM 7-bit encoding.
    pub fn encoder(&self) -> Encoder {
        match self.data_coding {
            DataCoding::Gsm7BitUnpacked => Encoder::Gsm7BitUnpacked(Gsm7BitUnpacked::default()),
            DataCoding::Latin1 => Encoder::Latin1(Latin1::default()),
            DataCoding::Ucs2 => Encoder::Ucs2(Ucs2::default()),
        }
    }

    fn update_short_message(&mut self) {
        self.fields.submit_sms = self.build_submit_sms();

        if self.udhi_indicator_must_be_set() {
            self.set_udhi_indicator();
        } else {
            self.reset_udhi_indicator();
        }
    }

    fn update_service_type(&mut self) {
        self.service_type.retain(|c| c.is_ascii());
        self.fields.set_service_type(&self.service_type);

        self.update_short_message();
    }

    fn update_source_addr(&mut self) {
        self.source_addr.retain(|c| c.is_ascii());
        self.fields.set_source_addr(&self.source_addr);

        self.update_short_message();
    }

    fn update_destination_addr(&mut self) {
        self.destination_addr.retain(|c| c.is_ascii());
        self.fields.set_destination_addr(&self.destination_addr);

        self.update_short_message();
    }

    fn build_submit_sm(&self) -> AppResult<SubmitSm> {
        let submit_sm = SubmitSm::builder()
            .service_type(ServiceType::new(self.fields.service_type.clone()?))
            .source_addr_ton(self.source_addr_ton.into())
            .source_addr_npi(self.source_addr_npi.into())
            .source_addr(self.fields.source_addr.clone()?)
            .dest_addr_ton(self.dest_addr_ton.into())
            .dest_addr_npi(self.dest_addr_npi.into())
            .destination_addr(self.fields.destination_addr.clone()?)
            .esm_class(self.esm_class.into())
            .build();

        Ok(submit_sm)
    }

    fn build_submit_sms(&self) -> AppResult<Vec<SubmitSm>> {
        let submit_sm = self.build_submit_sm()?;

        submit_sm
            .multipart(&self.short_message)
            .reference_u8(self.reference)
            .encoder(self.encoder())
            .build()
            .map_err(|_| AppUiError::MultiPart(MultiPartError::Todo))
    }

    fn increment_reference(&mut self) {
        self.reference = self.reference.wrapping_add(1);
    }

    fn counters_message(&self) -> Option<String> {
        let (sms_count, byte_count) = self.fields.submit_sms.as_ref().ok().map(|sms| {
            let byte_count: usize = sms.iter().map(|sms| sms.short_message().len()).sum();

            (sms.len(), byte_count)
        })?;
        let char_count = self.short_message.chars().count();

        Some(format!(
            "{sms_count} SMS {char_count} Characters {byte_count}/140 Bytes"
        ))
    }

    pub fn set_bound(&mut self, bound: bool) {
        self.bound = bound;
    }

    fn on_submit_button_clicked(&mut self) {
        // We build the SubmitSms every time the button is clicked to prevent building them every time a field is updated.
        if let Ok(sms) = self.build_submit_sms() {
            self.actions.submit_sms(sms);
            self.increment_reference();
        }
    }

    fn esm_class_value(&self) -> u8 {
        RusmppEsmClass::from(self.esm_class).into()
    }

    fn esm_class_value_str(&self) -> String {
        format!("0x{:02X}", self.esm_class_value())
    }

    fn set_udhi_indicator(&mut self) {
        self.esm_class.gsm_features = GsmFeatures::UdhiIndicator;
    }

    fn reset_udhi_indicator(&mut self) {
        self.esm_class.gsm_features = self.last_gsm_features;
    }

    fn save_last_gsm_features(&mut self) {
        self.last_gsm_features = self.esm_class.gsm_features;
    }

    fn udhi_indicator_must_be_set(&self) -> bool {
        self.fields
            .submit_sms
            .as_ref()
            .is_ok_and(|sms| sms.len() > 1)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered(|ui| {
            let display_err = |ui: &mut egui::Ui, err: &AppUiError| {
                ui.allocate_space(egui::vec2(0.0, 0.0));
                ui.colored_label(FUSION_RED, err.display_message());
                ui.end_row();
            };

            egui::Grid::new("submit_sm_service_type_grid")
                .num_columns(2)
                .spacing([12.0, 10.0])
                .striped(false)
                .show(ui, |ui| {
                    ui.label("Service Type");
                    ui.add(egui::TextEdit::singleline(&mut self.service_type).char_limit(5))
                        .on_hover_text("Max 5 ASCII characters")
                        .changed()
                        .then(|| {
                            self.update_service_type();
                        });
                    ui.end_row();

                    if let Err(err) = &self.fields.service_type {
                        display_err(ui, err);
                    }
                });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            egui::Grid::new("submit_sm_addr_grid")
                .num_columns(6)
                .spacing([16.0, 10.0])
                .striped(false)
                .show(ui, |ui| {
                    // TON
                    ui.label("Source Address TON");
                    ui.add(ComboBox::new(
                        "submit_sm_source_addr_ton",
                        &mut self.source_addr_ton,
                        Ton::VARIANTS,
                    ));
                    ui.label("Source Address NPI");
                    ui.add(ComboBox::new(
                        "submit_sm_source_addr_npi",
                        &mut self.source_addr_npi,
                        Npi::VARIANTS,
                    ));
                    ui.label("Source Address");
                    ui.add(egui::TextEdit::singleline(&mut self.source_addr).char_limit(20))
                        .on_hover_text("Max 20 ASCII characters")
                        .changed()
                        .then(|| self.update_source_addr());

                    ui.end_row();

                    ui.label("Destination Address TON");
                    ui.add(ComboBox::new(
                        "submit_sm_dest_addr_ton",
                        &mut self.dest_addr_ton,
                        Ton::VARIANTS,
                    ));
                    ui.label("Destination Address NPI");
                    ui.add(ComboBox::new(
                        "submit_sm_dest_addr_npi",
                        &mut self.dest_addr_npi,
                        Npi::VARIANTS,
                    ));
                    ui.label("Destination Address");
                    ui.add(egui::TextEdit::singleline(&mut self.destination_addr).char_limit(20))
                        .on_hover_text("Max 20 ASCII characters")
                        .changed()
                        .then(|| self.update_destination_addr());

                    ui.end_row();
                });

            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.heading(format!("Esm Class: ({})", self.esm_class_value_str()));
                ui.add(Separator::default().horizontal().spacing(8.0));
            });

            ui.add_space(12.0);

            egui::Grid::new("esm_class_grid")
                .num_columns(2)
                .spacing([12.0, 10.0])
                .striped(false)
                .show(ui, |ui| {
                    ui.label("Messaging Mode");
                    ui.add(ComboBox::new(
                        "submit_sm_esm_class_messaging_mode",
                        &mut self.esm_class.messaging_mode,
                        MessagingMode::VARIANTS,
                    ));

                    ui.label("Message Type");
                    ui.add(ComboBox::new(
                        "submit_sm_esm_class_message_type",
                        &mut self.esm_class.message_type,
                        MessageType::VARIANTS,
                    ));

                    ui.end_row();

                    ui.label("ANSI-41 Specific");
                    ui.add(ComboBox::new(
                        "submit_sm_esm_class_ansi41_specific",
                        &mut self.esm_class.ansi41_specific,
                        Ansi41Specific::VARIANTS,
                    ));

                    ui.label("GSM Features");
                    ui.add_enabled_ui(!self.udhi_indicator_must_be_set(), |ui| {
                        ui.add(ComboBox::new(
                            "submit_sm_esm_class_gsm_features",
                            &mut self.esm_class.gsm_features,
                            GsmFeatures::VARIANTS,
                        ))
                        .changed()
                        .then(|| self.save_last_gsm_features());
                    });

                    ui.end_row();
                });

            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.heading("Short Message");
                ui.add(Separator::default().horizontal().spacing(8.0));
            });

            ui.add_space(12.0);

            ui.vertical(|ui| {
                egui::Grid::new("submit_sm_data_coding_grid")
                    .num_columns(2)
                    .spacing([12.0, 10.0])
                    .striped(false)
                    .show(ui, |ui| {
                        ui.label("Data Coding");
                        // TODO: make our ComboBox widget take a function for the .changed() callback
                        egui::ComboBox::from_id_salt("submit_sm_data_coding")
                            .width(100.0)
                            .selected_text(<&'static str>::from(self.data_coding))
                            .show_ui(ui, |ui| {
                                for item in DataCoding::VARIANTS {
                                    ui.selectable_value(
                                        &mut self.data_coding,
                                        *item,
                                        <&'static str>::from(*item),
                                    )
                                    .changed()
                                    .then(|| self.update_short_message());
                                }
                            });

                        ui.end_row();
                    });

                ui.add_space(10.0);

                ui.add(
                    egui::TextEdit::multiline(&mut self.short_message)
                        .desired_width(ui.available_width()),
                )
                .changed()
                .then(|| {
                    self.update_short_message();
                });

                if let Some(counters_message) = self.counters_message() {
                    ui.add_space(10.0);
                    ui.label(counters_message);
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
                                RichText::new("Submit SMS").color(Color32::WHITE).strong(),
                            )
                            .fill(HIGH_BLUE),
                        )
                        .clicked()
                        .then(|| {
                            self.on_submit_button_clicked();
                        });
                    });
                });
            });
        })
        .response
    }
}
