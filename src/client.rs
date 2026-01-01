use futures::TryFutureExt;
use rusmpp::{Command, CommandStatus, Pdu};
use rusmppc::{Client, error::Error};

pub trait ClientExt {
    fn send_mapped(
        &self,
        pdu: impl Into<Pdu>,
    ) -> impl Future<Output = Result<(Command, impl Future<Output = Result<Command, Error>>), Error>>;
}

impl ClientExt for Client {
    /// Sends a PDU and maps the sequence number of the sent [`Command`] into a [`Command`].
    fn send_mapped(
        &self,
        pdu: impl Into<Pdu>,
    ) -> impl Future<Output = Result<(Command, impl Future<Output = Result<Command, Error>>), Error>>
    {
        let pdu = pdu.into();

        self.raw()
            .send(pdu.clone())
            .map_ok(|(sequence_number, response)| {
                (
                    Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(pdu),
                    response,
                )
            })
    }
}
