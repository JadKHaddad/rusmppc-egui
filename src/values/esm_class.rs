use rusmpp::values::{
    Ansi41Specific as RusmppAnsi41Specific, EsmClass as RusmppEsmClass,
    GsmFeatures as RusmppGsmFeatures, MessageType as RusmppMessageType,
    MessagingMode as RusmppMessagingMode,
};
use serde::{Deserialize, Serialize};
use strum::{IntoStaticStr, VariantArray};

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct EsmClass {
    pub messaging_mode: MessagingMode,
    pub message_type: MessageType,
    pub ansi41_specific: Ansi41Specific,
    pub gsm_features: GsmFeatures,
}

impl EsmClass {
    pub fn new(
        messaging_mode: MessagingMode,
        message_type: MessageType,
        ansi41_specific: Ansi41Specific,
        gsm_features: GsmFeatures,
    ) -> Self {
        Self {
            messaging_mode,
            message_type,
            ansi41_specific,
            gsm_features,
        }
    }
}

impl From<EsmClass> for RusmppEsmClass {
    fn from(value: EsmClass) -> Self {
        RusmppEsmClass {
            messaging_mode: RusmppMessagingMode::from(value.messaging_mode),
            message_type: RusmppMessageType::from(value.message_type),
            ansi41_specific: RusmppAnsi41Specific::from(value.ansi41_specific),
            gsm_features: RusmppGsmFeatures::from(value.gsm_features),
        }
    }
}

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum MessagingMode {
    #[default]
    Default,
    Datagram,
    Forward,
    StoreAndForward,
}

impl From<MessagingMode> for RusmppMessagingMode {
    fn from(value: MessagingMode) -> Self {
        match value {
            MessagingMode::Default => RusmppMessagingMode::Default,
            MessagingMode::Datagram => RusmppMessagingMode::Datagram,
            MessagingMode::Forward => RusmppMessagingMode::Forward,
            MessagingMode::StoreAndForward => RusmppMessagingMode::StoreAndForward,
        }
    }
}

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum MessageType {
    #[default]
    Default,
    MCDeliveryReceipt,
    IntermediateDeliveryNotification,
}

impl From<MessageType> for RusmppMessageType {
    fn from(value: MessageType) -> Self {
        match value {
            MessageType::Default => RusmppMessageType::Default,
            MessageType::MCDeliveryReceipt => {
                RusmppMessageType::ShortMessageContainsMCDeliveryReceipt
            }
            MessageType::IntermediateDeliveryNotification => {
                RusmppMessageType::ShortMessageContainsIntermediateDeliveryNotification
            }
        }
    }
}

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum Ansi41Specific {
    #[default]
    NotSelected,
    DeliveryAcknowledgement,
    UserAcknowledgment,
    ConversationAbort,
}

impl From<Ansi41Specific> for RusmppAnsi41Specific {
    fn from(value: Ansi41Specific) -> Self {
        match value {
            Ansi41Specific::NotSelected => RusmppAnsi41Specific::Other(0),
            Ansi41Specific::DeliveryAcknowledgement => {
                RusmppAnsi41Specific::ShortMessageContainsDeliveryAcknowledgement
            }
            Ansi41Specific::UserAcknowledgment => {
                RusmppAnsi41Specific::ShortMessageContainsUserAcknowledgment
            }
            Ansi41Specific::ConversationAbort => {
                RusmppAnsi41Specific::ShortMessageContainsConversationAbort
            }
        }
    }
}

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum GsmFeatures {
    #[default]
    NotSelected,
    UdhiIndicator,
    SetReplyPath,
    SetUdhiAndReplyPath,
}

impl From<GsmFeatures> for RusmppGsmFeatures {
    fn from(value: GsmFeatures) -> Self {
        match value {
            GsmFeatures::NotSelected => RusmppGsmFeatures::NotSelected,
            GsmFeatures::UdhiIndicator => RusmppGsmFeatures::UdhiIndicator,
            GsmFeatures::SetReplyPath => RusmppGsmFeatures::SetReplyPath,
            GsmFeatures::SetUdhiAndReplyPath => RusmppGsmFeatures::SetUdhiAndReplyPath,
        }
    }
}
