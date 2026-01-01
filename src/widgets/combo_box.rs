use eframe::egui;

/// ComboBox wrapper for types convertible to &'static str
pub struct ComboBox<'a, T> {
    id_salt: &'a str,
    width: f32,
    selected: &'a mut T,
    variants: &'a [T],
}

impl<'a, T> ComboBox<'a, T> {
    pub fn new(id_salt: &'a str, selected: &'a mut T, variants: &'a [T]) -> Self {
        Self {
            id_salt,
            width: 100.0,
            selected,
            variants,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl<'a, T> egui::Widget for ComboBox<'a, T>
where
    T: Copy + PartialEq,
    &'static str: From<T>,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::ComboBox::from_id_salt(self.id_salt)
            .width(self.width)
            .selected_text(<&'static str>::from(*self.selected))
            .show_ui(ui, |ui| {
                for item in self.variants {
                    ui.selectable_value(self.selected, *item, <&'static str>::from(*item));
                }
            })
            .response
    }
}
