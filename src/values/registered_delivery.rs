use rusmpp::values::{
    IntermediateNotification as RusmppIntermediateNotification,
    McDeliveryReceipt as RusmppMcDeliveryReceipt, RegisteredDelivery as RusmppRegisteredDelivery,
    SmeOriginatedAcknowledgement as RusmppSmeOriginatedAcknowledgement,
};

use serde::{Deserialize, Serialize};
use strum::{IntoStaticStr, VariantArray};

pub struct RegisteredDelivery {
    mc_delivery_receipt: McDeliveryReceipt,
    sme_originated_acknowledgement: SmeOriginatedAcknowledgement,
    intermediate_notification: IntermediateNotification,
}

impl RegisteredDelivery {
    pub fn new(
        mc_delivery_receipt: McDeliveryReceipt,
        sme_originated_acknowledgement: SmeOriginatedAcknowledgement,
        intermediate_notification: IntermediateNotification,
    ) -> Self {
        Self {
            mc_delivery_receipt,
            sme_originated_acknowledgement,
            intermediate_notification,
        }
    }
}

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum McDeliveryReceipt {
    #[default]
    NotRequested,
    SuccessOrFailure,
    Failure,
    Success,
}

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum SmeOriginatedAcknowledgement {
    #[default]
    NotRequested,
    Delivery,
    Ack,
    DeliveryAndAck,
}

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum IntermediateNotification {
    #[default]
    NotRequested,
    Requested,
}

impl From<McDeliveryReceipt> for RusmppMcDeliveryReceipt {
    fn from(value: McDeliveryReceipt) -> Self {
        match value {
            McDeliveryReceipt::NotRequested => Self::NoMcDeliveryReceiptRequested,
            McDeliveryReceipt::SuccessOrFailure => {
                Self::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure
            }
            McDeliveryReceipt::Failure => {
                Self::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure
            }
            McDeliveryReceipt::Success => {
                Self::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess
            }
        }
    }
}

impl From<SmeOriginatedAcknowledgement> for RusmppSmeOriginatedAcknowledgement {
    fn from(value: SmeOriginatedAcknowledgement) -> Self {
        match value {
            SmeOriginatedAcknowledgement::NotRequested => {
                Self::NoReceiptSmeAcknowledgementRequested
            }
            SmeOriginatedAcknowledgement::Delivery => Self::SmeDeliveryAcknowledgementRequested,
            SmeOriginatedAcknowledgement::Ack => Self::SmeUserAcknowledgementRequested,
            SmeOriginatedAcknowledgement::DeliveryAndAck => {
                Self::BothDeliveryAndUserAcknowledgmentRequested
            }
        }
    }
}

impl From<IntermediateNotification> for RusmppIntermediateNotification {
    fn from(value: IntermediateNotification) -> Self {
        match value {
            IntermediateNotification::NotRequested => Self::NoIntermediaryNotificationRequested,
            IntermediateNotification::Requested => Self::IntermediateNotificationRequested,
        }
    }
}

impl
    From<(
        McDeliveryReceipt,
        SmeOriginatedAcknowledgement,
        IntermediateNotification,
    )> for RegisteredDelivery
{
    fn from(
        value: (
            McDeliveryReceipt,
            SmeOriginatedAcknowledgement,
            IntermediateNotification,
        ),
    ) -> RegisteredDelivery {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<RegisteredDelivery> for RusmppRegisteredDelivery {
    fn from(value: RegisteredDelivery) -> RusmppRegisteredDelivery {
        Self::new(
            value.mc_delivery_receipt.into(),
            value.sme_originated_acknowledgement.into(),
            value.intermediate_notification.into(),
            0,
        )
    }
}
