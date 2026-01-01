use strum::VariantArray;

#[derive(VariantArray, Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum BindMode {
    #[default]
    Tx,
    Rx,
    Trx,
}

impl ::core::convert::From<BindMode> for &'static str {
    #[inline]
    fn from(x: BindMode) -> &'static str {
        match x {
            BindMode::Tx => "Transmitter",
            BindMode::Rx => "Receiver",
            BindMode::Trx => "Transceiver",
        }
    }
}
