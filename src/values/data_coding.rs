use rusmpp::values::DataCoding as RusmppDataCoding;
use serde::{Deserialize, Serialize};
use strum::{IntoStaticStr, VariantArray};

#[derive(
    IntoStaticStr, VariantArray, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum DataCoding {
    #[default]
    Gsm7BitUnpacked,
    Latin1,
    Ucs2,
}

impl From<DataCoding> for RusmppDataCoding {
    fn from(data_coding: DataCoding) -> Self {
        match data_coding {
            DataCoding::Gsm7BitUnpacked => RusmppDataCoding::McSpecific,
            DataCoding::Latin1 => RusmppDataCoding::Latin1,
            DataCoding::Ucs2 => RusmppDataCoding::Ucs2,
        }
    }
}
