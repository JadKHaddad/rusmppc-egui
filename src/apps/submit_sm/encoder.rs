use rusmpp::{
    extra::{
        concatenation::{Concatenation, Concatenator},
        encoding::{
            Encoder as RusmppEncoder,
            gsm7bit::{Gsm7BitPackedEncoder, Gsm7BitUnpackedEncoder},
            latin1::Latin1Encoder,
            ucs2::Ucs2Encoder,
        },
    },
    values::DataCoding,
};

use crate::result::{EncodeError, MultiPartError};

pub enum Encoder {
    Gsm7BitUnpacked(Gsm7BitUnpackedEncoder),
    Gsm7BitPacked(Gsm7BitPackedEncoder),
    Latin1(Latin1Encoder),
    Ucs2(Ucs2Encoder),
}

impl RusmppEncoder for Encoder {
    type Error = EncodeError;

    fn encode(&self, message: &str) -> Result<(Vec<u8>, DataCoding), Self::Error> {
        match self {
            Encoder::Gsm7BitUnpacked(encoder) => {
                encoder.encode(message).map_err(|_| EncodeError::Todo)
            }
            Encoder::Gsm7BitPacked(encoder) => {
                encoder.encode(message).map_err(|_| EncodeError::Todo)
            }
            Encoder::Latin1(encoder) => encoder.encode(message).map_err(|_| EncodeError::Todo),
            Encoder::Ucs2(encoder) => encoder.encode(message).map_err(|_| EncodeError::Todo),
        }
    }
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
            Encoder::Gsm7BitPacked(encoder) => encoder
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
