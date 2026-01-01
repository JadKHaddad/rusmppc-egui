use rusmpp::{
    extra::{
        concatenation::{Concatenation, Concatenator},
        encoding::{gsm7bit::Gsm7BitUnpacked, latin1::Latin1, ucs2::Ucs2},
    },
    values::DataCoding,
};

use crate::result::MultiPartError;

pub enum Encoder {
    Gsm7BitUnpacked(Gsm7BitUnpacked),
    Latin1(Latin1),
    Ucs2(Ucs2),
}

impl Concatenator for Encoder {
    type Error = MultiPartError;

    fn concatenate(
        &self,
        message: &str,
        max_message_size: usize,
        part_header_size: usize,
    ) -> Result<(Concatenation, DataCoding), Self::Error> {
        match self {
            Encoder::Gsm7BitUnpacked(encoder) => encoder
                .concatenate(message, max_message_size, part_header_size)
                .map_err(|_| MultiPartError::Todo),
            Encoder::Latin1(encoder) => encoder
                .concatenate(message, max_message_size, part_header_size)
                .map_err(|_| MultiPartError::Todo),
            Encoder::Ucs2(encoder) => encoder
                .concatenate(message, max_message_size, part_header_size)
                .map_err(|_| MultiPartError::Todo),
        }
    }
}
