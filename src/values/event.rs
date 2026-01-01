use rusmpp::Command;

use crate::result::AppActionError;

#[derive(Debug)]
pub enum Event {
    Error(AppActionError),
    Connected,
    Disconnected,
    Closed,
    Bound,
    Sent(Command),
    Received(Command),
}
