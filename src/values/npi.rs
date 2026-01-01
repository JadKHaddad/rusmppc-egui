use rusmpp::values::Npi as RusmppNpi;
use strum::{IntoStaticStr, VariantArray};

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq)]
pub enum Npi {
    #[default]
    Unknown,
    Isdn,
    Data,
    Telex,
    LandMobile,
    National,
    Private,
    Ermes,
    Internet,
    WapClientId,
}

impl From<Npi> for RusmppNpi {
    fn from(npi: Npi) -> Self {
        match npi {
            Npi::Unknown => RusmppNpi::Unknown,
            Npi::Isdn => RusmppNpi::Isdn,
            Npi::Data => RusmppNpi::Data,
            Npi::Telex => RusmppNpi::Telex,
            Npi::LandMobile => RusmppNpi::LandMobile,
            Npi::National => RusmppNpi::National,
            Npi::Private => RusmppNpi::Private,
            Npi::Ermes => RusmppNpi::Ermes,
            Npi::Internet => RusmppNpi::Internet,
            Npi::WapClientId => RusmppNpi::WapClientId,
        }
    }
}
