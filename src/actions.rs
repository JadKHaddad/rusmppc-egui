use std::sync::{Arc, atomic::AtomicBool};

use rusmpp::pdus::{BindAny, SubmitSm};

use crate::{types::SmppUrl, values::BindMode};

#[derive(Clone)]
pub struct ActionsChannel {
    sender: tokio::sync::mpsc::UnboundedSender<Action>,
}

impl ActionsChannel {
    pub const fn new(sender: tokio::sync::mpsc::UnboundedSender<Action>) -> Self {
        Self { sender }
    }

    fn send(&self, action: Action) {
        let _ = self.sender.send(action);
    }

    pub fn bind(
        &self,
        mode: BindMode,
        url: SmppUrl,
        interval: u64,
        bind: BindAny,
        loading: Arc<AtomicBool>,
    ) {
        let action = Action::Bind(BindAction {
            mode,
            interval,
            url,
            bind,
            loading,
        });

        self.send(action);
    }

    pub fn unbind(&self, loading: Arc<AtomicBool>) {
        self.send(Action::Unbind(UnbindAction { loading }));
    }

    pub fn submit_sms(&self, sms: Vec<SubmitSm>) {
        self.send(Action::SubmitSms(SubmitSmsAction { sms }));
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Action {
    Bind(BindAction),
    Unbind(UnbindAction),
    SubmitSms(SubmitSmsAction),
}

#[derive(Debug, Clone)]
pub struct BindAction {
    pub mode: BindMode,
    pub url: SmppUrl,
    pub interval: u64,
    pub bind: BindAny,
    pub loading: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
pub struct UnbindAction {
    pub loading: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
pub struct SubmitSmsAction {
    pub sms: Vec<SubmitSm>,
}
