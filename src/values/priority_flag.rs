use strum::{IntoStaticStr, VariantArray};

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Default)]
pub enum GsmSmsPriorityFlag {
    #[default]
    None,
    Priority1,
    Priority2,
    Priority3,
}

impl GsmSmsPriorityFlag {
    pub const fn from_u8(value: u8) -> Option<Self> {
        let value = match value {
            0 => GsmSmsPriorityFlag::None,
            1 => GsmSmsPriorityFlag::Priority1,
            2 => GsmSmsPriorityFlag::Priority2,
            3 => GsmSmsPriorityFlag::Priority3,
            _ => return None,
        };

        Some(value)
    }
}

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Default)]
pub enum GsmCbsPriorityFlag {
    #[default]
    Normal,
    ImmediateBroadcast,
    HighPriority,
    Reserved,
    PriorityBackground,
}

impl GsmCbsPriorityFlag {
    pub const fn from_u8(value: u8) -> Option<Self> {
        let value = match value {
            0 => GsmCbsPriorityFlag::Normal,
            1 => GsmCbsPriorityFlag::ImmediateBroadcast,
            2 => GsmCbsPriorityFlag::HighPriority,
            3 => GsmCbsPriorityFlag::Reserved,
            4 => GsmCbsPriorityFlag::PriorityBackground,
            _ => return None,
        };

        Some(value)
    }
}

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Default)]
pub enum Ansi136PriorityFlag {
    #[default]
    Bulk,
    Normal,
    Urgent,
    VeryUrgent,
}

impl Ansi136PriorityFlag {
    pub const fn from_u8(value: u8) -> Option<Self> {
        let value = match value {
            0 => Ansi136PriorityFlag::Bulk,
            1 => Ansi136PriorityFlag::Normal,
            2 => Ansi136PriorityFlag::Urgent,
            3 => Ansi136PriorityFlag::VeryUrgent,
            _ => return None,
        };

        Some(value)
    }
}

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Default)]
pub enum Is95PriorityFlag {
    #[default]
    Normal,
    Interactive,
    Urgent,
    Emergency,
}

impl Is95PriorityFlag {
    pub const fn from_u8(value: u8) -> Option<Self> {
        let value = match value {
            0 => Is95PriorityFlag::Normal,
            1 => Is95PriorityFlag::Interactive,
            2 => Is95PriorityFlag::Urgent,
            3 => Is95PriorityFlag::Emergency,
            _ => return None,
        };

        Some(value)
    }
}

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Default)]
pub enum Ansi41CbsPriorityFlag {
    #[default]
    Normal,
    Interactive,
    Urgent,
    Emergency,
}

impl Ansi41CbsPriorityFlag {
    pub const fn from_u8(value: u8) -> Option<Self> {
        let value = match value {
            0 => Ansi41CbsPriorityFlag::Normal,
            1 => Ansi41CbsPriorityFlag::Interactive,
            2 => Ansi41CbsPriorityFlag::Urgent,
            3 => Ansi41CbsPriorityFlag::Emergency,
            _ => return None,
        };

        Some(value)
    }
}
