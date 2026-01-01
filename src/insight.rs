use rusmpp::{Command, CommandStatus, Pdu};
use rusmppc::Insight;

use crate::values::Event;

pub enum InsightCommand {
    Sent(Command),
    Received(Command),
}

pub trait InsightExt: Sized {
    fn into_command(self) -> Option<InsightCommand>;

    fn into_event(self) -> Option<Event> {
        match self.into_command() {
            Some(InsightCommand::Sent(cmd)) => Some(Event::Sent(cmd)),
            Some(InsightCommand::Received(cmd)) => Some(Event::Received(cmd)),
            None => None,
        }
    }
}

impl InsightExt for Insight {
    fn into_command(self) -> Option<InsightCommand> {
        let command =
            match self {
                Insight::SentEnquireLink(number) => InsightCommand::Sent(Command::new_const(
                    CommandStatus::EsmeRok,
                    number,
                    Pdu::EnquireLink,
                )),
                Insight::ReceivedEnquireLinkResp(number) => InsightCommand::Received(
                    Command::new_const(CommandStatus::EsmeRok, number, Pdu::EnquireLinkResp),
                ),
                Insight::ReceivedEnquireLink(number) => InsightCommand::Received(
                    Command::new_const(CommandStatus::EsmeRok, number, Pdu::EnquireLink),
                ),
                Insight::SentEnquireLinkResp(number) => InsightCommand::Sent(Command::new_const(
                    CommandStatus::EsmeRok,
                    number,
                    Pdu::EnquireLinkResp,
                )),
                _ => return None,
            };

        Some(command)
    }
}
