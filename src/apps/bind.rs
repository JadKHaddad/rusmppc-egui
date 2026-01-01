use std::{
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use eframe::egui::{self, Color32, RichText};
use egui_material_icons::{
    icon_button,
    icons::{ICON_VISIBILITY, ICON_VISIBILITY_OFF},
};
use rusmpp::{pdus::BindAny, types::COctetString};
use strum::VariantArray;

use crate::{
    actions::ActionsChannel,
    colors::{FUSION_RED, HIGH_BLUE},
    result::{AppResult, AppUiError},
    types::SmppUrl,
    values::{BindMode, InterfaceVersion, Npi, Ton},
    widgets::ComboBox,
};

struct RusmppFields {
    url: AppResult<SmppUrl>,
    system_id: AppResult<COctetString<1, 16>>,
    password: AppResult<COctetString<1, 9>>,
    system_type: AppResult<COctetString<1, 13>>,
}

impl RusmppFields {
    fn new(url: &str, system_id: &str, password: &str, system_type: &str) -> Self {
        Self {
            url: SmppUrl::new(url).map_err(AppUiError::Url),
            system_id: COctetString::from_str(system_id)
                .map_err(|_| AppUiError::invalid_system_id()),
            password: COctetString::from_str(password).map_err(|_| AppUiError::invalid_password()),
            system_type: COctetString::from_str(system_type)
                .map_err(|_| AppUiError::invalid_system_type()),
        }
    }

    fn set_url(&mut self, url: &str) {
        self.url = SmppUrl::new(url).map_err(AppUiError::Url);
    }

    fn set_system_id(&mut self, system_id: &str) {
        self.system_id =
            COctetString::from_str(system_id).map_err(|_| AppUiError::invalid_system_id());
    }

    fn set_password(&mut self, password: &str) {
        self.password =
            COctetString::from_str(password).map_err(|_| AppUiError::invalid_password());
    }

    fn set_system_type(&mut self, system_type: &str) {
        self.system_type =
            COctetString::from_str(system_type).map_err(|_| AppUiError::invalid_system_type());
    }

    fn all_fields_valid(&self) -> bool {
        matches!(
            (
                &self.url,
                &self.system_id,
                &self.password,
                &self.system_type
            ),
            (Ok(_), Ok(_), Ok(_), Ok(_))
        )
    }
}

pub struct BindApp {
    actions: ActionsChannel,
    url: String,
    system_id: String,
    password: String,
    system_type: String,
    addr_ton: Ton,
    addr_npi: Npi,
    interface_version: InterfaceVersion,
    mode: BindMode,
    fields: RusmppFields,
    bound: bool,
    password_visible: bool,
    loading: Arc<AtomicBool>,
}

impl BindApp {
    pub fn new(actions: ActionsChannel) -> Self {
        let url = String::from("smpps://rusmpps.rusmpp.org:2776");
        let system_id = String::from("system_id");
        let password = String::from("password");
        let system_type = String::from("system_type");

        let fields = RusmppFields::new(&url, &system_id, &password, &system_type);

        Self {
            actions,
            url,
            system_id,
            password,
            system_type,
            addr_ton: Ton::default(),
            addr_npi: Npi::default(),
            interface_version: InterfaceVersion::default(),
            mode: BindMode::default(),
            fields,
            bound: false,
            password_visible: false,
            loading: Arc::new(AtomicBool::new(false)),
        }
    }

    fn create_bind_pdu(&self) -> AppResult<BindAny> {
        let bind_pdu = BindAny::builder()
            .system_id(self.fields.system_id.clone()?)
            .password(self.fields.password.clone()?)
            .system_type(self.fields.system_type.clone()?)
            .addr_ton(self.addr_ton.into())
            .addr_npi(self.addr_npi.into())
            .interface_version(self.interface_version.into())
            .build();

        Ok(bind_pdu)
    }

    fn get_url_and_pdu(&self) -> AppResult<(SmppUrl, BindAny)> {
        let url = self.fields.url.clone()?;
        let bind_pdu = self.create_bind_pdu()?;

        Ok((url, bind_pdu))
    }

    fn update_url(&mut self) {
        self.fields.set_url(&self.url);
    }

    fn update_system_id(&mut self) {
        self.system_id.retain(|c| c.is_ascii());
        self.fields.set_system_id(&self.system_id);
    }

    fn update_password(&mut self) {
        self.password.retain(|c| c.is_ascii());
        self.fields.set_password(&self.password);
    }

    fn update_system_type(&mut self) {
        self.system_type.retain(|c| c.is_ascii());
        self.fields.set_system_type(&self.system_type);
    }

    fn toggle_password_visibility(&mut self) {
        self.password_visible = !self.password_visible;
    }

    fn password_visibility_icon(&self) -> &'static str {
        if self.password_visible {
            ICON_VISIBILITY_OFF
        } else {
            ICON_VISIBILITY
        }
    }

    pub fn set_bound(&mut self, bound: bool) {
        self.bound = bound;
    }

    const fn bind_button_text(&self) -> &'static str {
        if self.bound { "Unbind" } else { "Bind" }
    }

    const fn bind_button_color(&self) -> egui::Color32 {
        if self.bound { FUSION_RED } else { HIGH_BLUE }
    }

    fn on_bind_button_clicked(&mut self) {
        if self.bound {
            self.actions.unbind(self.loading.clone());
        } else if let Ok((url, bind)) = self.get_url_and_pdu() {
            self.actions
                .bind(self.mode, url, bind, self.loading.clone());
        }
    }
}

impl egui::Widget for &mut BindApp {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let loading = self.loading.load(Ordering::Relaxed);

        ui.vertical_centered(|ui| {
            ui.add_enabled_ui(!loading && !self.bound, |ui| {
                egui::Grid::new("bind_grid_1")
                    .num_columns(2)
                    .spacing([12.0, 10.0])
                    .striped(false)
                    .show(ui, |ui| {
                        // this will now be used for the url error
                        let display_err = |ui: &mut egui::Ui, err: &AppUiError| {
                            ui.allocate_space(egui::vec2(0.0, 0.0));
                            ui.colored_label(FUSION_RED, err.display_message());
                            ui.end_row();
                        };

                        ui.label("URL");
                        ui.add(egui::TextEdit::singleline(&mut self.url).desired_width(320.0))
                            .changed()
                            .then(|| {
                                self.update_url();
                            });
                        ui.end_row();

                        if let Err(err) = &self.fields.url {
                            display_err(ui, err);
                        }

                        ui.label("System ID");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.system_id)
                                .char_limit(15)
                                .desired_width(320.0),
                        )
                        .on_hover_text("Max 15 ASCII characters")
                        .changed()
                        .then(|| {
                            self.update_system_id();
                        });
                        ui.end_row();

                        if let Err(err) = &self.fields.system_id {
                            display_err(ui, err);
                        }

                        ui.label("Password");
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.password)
                                    .char_limit(8)
                                    .password(!self.password_visible)
                                    .desired_width(320.0),
                            )
                            .on_hover_text("Max 8 ASCII characters")
                            .changed()
                            .then(|| {
                                self.update_password();
                            });

                            icon_button(ui, self.password_visibility_icon())
                                .clicked()
                                .then(|| {
                                    self.toggle_password_visibility();
                                });
                        });
                        ui.end_row();

                        if let Err(err) = &self.fields.password {
                            display_err(ui, err);
                        }

                        ui.label("System Type");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.system_type)
                                .char_limit(12)
                                .desired_width(320.0),
                        )
                        .on_hover_text("Max 12 ASCII characters")
                        .changed()
                        .then(|| {
                            self.update_system_type();
                        });
                        ui.end_row();

                        if let Err(err) = &self.fields.system_type {
                            display_err(ui, err);
                        }
                    });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);

                egui::Grid::new("bind_grid_2")
                    .num_columns(2)
                    .spacing([12.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Address TON");
                        ui.add(ComboBox::new(
                            "bind_addr_ton",
                            &mut self.addr_ton,
                            Ton::VARIANTS,
                        ));
                        ui.end_row();

                        ui.label("Address NPI");
                        ui.add(ComboBox::new(
                            "bind_addr_npi",
                            &mut self.addr_npi,
                            Npi::VARIANTS,
                        ));
                        ui.end_row();

                        ui.label("Interface Version");
                        ui.add(ComboBox::new(
                            "bind_interface_version",
                            &mut self.interface_version,
                            InterfaceVersion::VARIANTS,
                        ));
                        ui.end_row();

                        ui.label("Bind Mode");
                        ui.add(ComboBox::new(
                            "bind_bind_mode",
                            &mut self.mode,
                            BindMode::VARIANTS,
                        ));
                        ui.end_row();
                    });

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(12.0);
            });

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_enabled_ui(!loading && self.fields.all_fields_valid(), |ui| {
                        ui.add_sized(
                            [140.0, 32.0],
                            egui::Button::new(
                                RichText::new(self.bind_button_text())
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(self.bind_button_color()),
                        )
                        .clicked()
                        .then(|| {
                            self.on_bind_button_clicked();
                        });
                    });
                });
            });
        })
        .response
    }
}
