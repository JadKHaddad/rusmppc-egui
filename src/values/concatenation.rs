use serde::{Deserialize, Serialize};
use strum::{IntoStaticStr, VariantArray};

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum Concatenation {
    #[default]
    Udh,
    Sar,
    MessagePayload,
}

impl Concatenation {
    pub const fn is_udh(&self) -> bool {
        matches!(self, Concatenation::Udh)
    }
}
