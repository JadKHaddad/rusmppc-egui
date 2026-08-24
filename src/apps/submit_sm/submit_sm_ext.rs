use rusmpp::{pdus::SubmitSm, tlvs::TlvValue, values::MessagePayload};

pub trait SubmitSmExt {
    fn message_payload(&self) -> Option<&MessagePayload>;

    /// Returns `true` if `short_message` is not empty or if `message_payload` exists and is not empty.
    fn has_non_empty_message(&self) -> bool;
}

impl SubmitSmExt for SubmitSm {
    fn message_payload(&self) -> Option<&MessagePayload> {
        self.tlvs().iter().find_map(|tlv| {
            if let Some(TlvValue::MessagePayload(value)) = tlv.value() {
                Some(value)
            } else {
                None
            }
        })
    }

    fn has_non_empty_message(&self) -> bool {
        self.message_payload()
            .map_or(!self.short_message().is_empty(), |payload| {
                !payload.value.is_empty()
            })
    }
}
