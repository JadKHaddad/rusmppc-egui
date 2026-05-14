use std::ops::Deref;

use eframe::egui::widgets::text_edit::TextBuffer;

#[derive(Default, Clone)]
pub struct DigitsTextBuffer<T>(T);

impl<T> DigitsTextBuffer<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for DigitsTextBuffer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: TextBuffer> TextBuffer for DigitsTextBuffer<T> {
    fn is_mutable(&self) -> bool {
        self.0.is_mutable()
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        text.chars()
            .all(|ch| ch.is_ascii_digit())
            .then_some(text)
            .map(|text| self.0.insert_text(text, char_index))
            .unwrap_or(0)
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.0.delete_char_range(char_range);
    }

    fn type_id(&self) -> std::any::TypeId {
        self.0.type_id()
    }
}
