use rusmpp::values::InterfaceVersion as RusmppInterfaceVersion;
use strum::VariantArray;

#[derive(VariantArray, Clone, Copy, Default, PartialEq, Eq)]
pub enum InterfaceVersion {
    Smpp3_3,
    Smpp3_4,
    #[default]
    Smpp5_0,
}

impl From<InterfaceVersion> for RusmppInterfaceVersion {
    fn from(interface_version: InterfaceVersion) -> Self {
        match interface_version {
            InterfaceVersion::Smpp5_0 => RusmppInterfaceVersion::Smpp5_0,
            InterfaceVersion::Smpp3_4 => RusmppInterfaceVersion::Smpp3_4,
            InterfaceVersion::Smpp3_3 => RusmppInterfaceVersion::Smpp3_3OrEarlier(0x33),
        }
    }
}

impl ::core::convert::From<InterfaceVersion> for &'static str {
    #[inline]
    fn from(x: InterfaceVersion) -> &'static str {
        match x {
            InterfaceVersion::Smpp5_0 => "v5",
            InterfaceVersion::Smpp3_4 => "v3.4",
            InterfaceVersion::Smpp3_3 => "v3.3",
        }
    }
}
