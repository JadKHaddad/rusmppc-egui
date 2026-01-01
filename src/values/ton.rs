use rusmpp::values::Ton as RusmppTon;
use strum::{IntoStaticStr, VariantArray};

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq)]
pub enum Ton {
    #[default]
    Unknown,
    International,
    National,
    NetworkSpecific,
    SubscriberNumber,
    Alphanumeric,
    Abbreviated,
}

impl From<Ton> for RusmppTon {
    fn from(ton: Ton) -> Self {
        match ton {
            Ton::Unknown => RusmppTon::Unknown,
            Ton::International => RusmppTon::International,
            Ton::National => RusmppTon::National,
            Ton::NetworkSpecific => RusmppTon::NetworkSpecific,
            Ton::SubscriberNumber => RusmppTon::SubscriberNumber,
            Ton::Alphanumeric => RusmppTon::Alphanumeric,
            Ton::Abbreviated => RusmppTon::Abbreviated,
        }
    }
}
