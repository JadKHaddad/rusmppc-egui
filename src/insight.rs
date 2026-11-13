use rusmpp::Command;
use rusmppc::event::Insight;

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
        let command = match self {
            Insight::Sent(command) => InsightCommand::Sent(command),
            Insight::Received(command) => InsightCommand::Received(command),
            _ => return None,
        };

        Some(command)
    }
}
