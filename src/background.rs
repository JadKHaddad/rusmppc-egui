use std::{sync::atomic::Ordering, time::Duration};

use futures::{Stream, StreamExt, TryFutureExt};
use rusmpp::{
    Command, CommandStatus, Pdu,
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, DeliverSmResp},
    types::COctetString,
};
use rusmppc::event::InsightEvent;

use crate::types::WsUrl;

use crate::{
    actions::{Action, BindAction, SubmitSmsAction, UnbindAction},
    client::{Client, ClientExt},
    insight::InsightExt,
    result::AppActionError,
    runtime,
    state::AppState,
    types::SmppUrl,
    values::{BindMode, Event},
};

#[derive(Clone)]
pub struct BackgroundApp {
    state: AppState,
}

impl BackgroundApp {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    fn set_client(&self, client: Client) {
        self.state.set_client(client);
    }

    fn clear_client(&self) {
        self.state.clear_client();
    }

    fn request_repaint(&self) {
        self.state.request_repaint();
    }

    fn push_event(&self, event: Event) {
        self.state.push_event(event);
    }

    fn incoming_event_blink(&self) {
        self.state.incoming_event_blink();
    }

    fn outgoing_event_blink(&self) {
        self.state.outgoing_event_blink();
    }

    async fn handle_events(&self, mut events: impl Stream<Item = InsightEvent> + Unpin) {
        while let Some(event) = events.next().await {
            match event {
                InsightEvent::Incoming(command) => {
                    let deliver_sm_resp = match command.pdu() {
                        Some(Pdu::DeliverSm(deliver_sm)) => {
                            let deliver_sm_resp = DeliverSmResp::builder()
                                .message_id(
                                    deliver_sm
                                        .receipted_message_id()
                                        .cloned()
                                        .unwrap_or(COctetString::empty()),
                                )
                                .build();

                            Some((command.sequence_number(), deliver_sm_resp))
                        }
                        _ => None,
                    };

                    self.incoming_event_blink();
                    self.push_event(Event::Received(command));

                    if let Some((sequence_number, deliver_sm_resp)) = deliver_sm_resp {
                        self.deliver_sm_resp(sequence_number, deliver_sm_resp).await;
                    }
                }
                InsightEvent::Insight(insight) => {
                    if let Some(event) = insight.into_event() {
                        match event {
                            Event::Received(_) => self.incoming_event_blink(),
                            Event::Sent(_) => self.outgoing_event_blink(),
                            _ => {}
                        }

                        self.push_event(event)
                    }
                }
                InsightEvent::Error(err) => {
                    self.push_event(Event::Error(AppActionError::Background(err)))
                }
            }

            self.request_repaint();
        }

        self.clear_client();
        self.push_event(Event::Disconnected);
        self.request_repaint();
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn connect(
        &self,
        url: SmppUrl,
        _ws_proxy_url: WsUrl,
        interval: u64,
    ) -> Result<(Client, impl Stream<Item = InsightEvent> + 'static), AppActionError> {
        Client::builder()
            .enquire_link_interval(Duration::from_secs(interval))
            .disable_interface_version_check()
            .events()
            .insights()
            .connect(url.to_string())
            .await
            .map_err(|err| AppActionError::Connection(err.into()))
    }

    #[cfg(target_arch = "wasm32")]
    async fn connect(
        &self,
        url: SmppUrl,
        ws_proxy_url: WsUrl,
        interval: u64,
    ) -> Result<(Client, impl Stream<Item = InsightEvent> + 'static), AppActionError> {
        use gloo_net::websocket::{Message, futures::WebSocket};
        use tokio_util::compat::FuturesAsyncReadCompatExt;

        #[derive(Debug, serde::Deserialize)]
        struct UpstreamResult {
            error: Option<String>,
        }

        let url = url.into_wasm();

        let mut ws = WebSocket::open(&format!(
            "{}?ssl={}&domain={}&port={}",
            ws_proxy_url.url, url.ssl, url.domain, url.port
        ))
        .map_err(|err| AppActionError::Connection(err.into()))?;

        ws.next()
            .await
            .ok_or_else(|| {
                AppActionError::Connection(anyhow::anyhow!(
                    "WebSocket connection closed unexpectedly"
                ))
            })?
            .map_err(|err| AppActionError::Connection(err.into()))
            .map(|message| match message {
                Message::Bytes(bytes) => bytes,
                Message::Text(text) => text.into_bytes(),
            })
            .map(|bytes| {
                serde_json::from_slice::<UpstreamResult>(&bytes).map_err(|err| {
                    AppActionError::Connection(anyhow::anyhow!(
                        "Failed to parse upstream response: {}",
                        err
                    ))
                })
            })?
            .map(|result| match result.error {
                None => Ok(()),
                Some(err) => Err(AppActionError::Connection(anyhow::anyhow!(
                    "Upstream connection error: {}",
                    err
                ))),
            })??;

        let client = Client::builder_wasm()
            .enquire_link_interval(Duration::from_secs(interval))
            .disable_interface_version_check()
            .events()
            .insights()
            .connected(ws.compat());

        Ok(client)
    }

    async fn bind(&self, action: BindAction) {
        if self.state.client().is_some() {
            return;
        }

        action.loading.store(true, Ordering::Relaxed);

        match self
            .connect(action.url, action.ws_proxy_url, action.interval)
            .await
        {
            Err(err) => {
                self.push_event(Event::Error(err));
            }
            Ok((client, events)) => {
                self.push_event(Event::Connected);
                self.request_repaint();

                let pdu = match action.mode {
                    BindMode::Trx => Pdu::from(BindTransceiver::from(action.bind)),
                    BindMode::Tx => Pdu::from(BindTransmitter::from(action.bind)),
                    BindMode::Rx => Pdu::from(BindReceiver::from(action.bind)),
                };

                match client
                    .send_mapped(pdu)
                    .and_then(|(command, response)| {
                        self.outgoing_event_blink();
                        self.push_event(Event::Sent(command));
                        self.request_repaint();

                        response
                    })
                    .await
                {
                    Err(err) => {
                        self.push_event(Event::Error(AppActionError::Bind(err)));
                    }
                    Ok(response) => {
                        self.incoming_event_blink();
                        self.push_event(Event::Received(response));
                        self.push_event(Event::Bound);
                        self.set_client(client);

                        let this = self.clone();

                        runtime::spawn(async move { this.handle_events(events).await });
                    }
                }
            }
        }

        action.loading.store(false, Ordering::Relaxed);

        self.request_repaint();
    }

    async fn unbind(&self, action: UnbindAction) {
        let Some(client) = self.state.client().as_ref().map(|client| client.clone()) else {
            return;
        };

        action.loading.store(true, Ordering::Relaxed);

        _ = client
            .send_mapped(Pdu::Unbind)
            .and_then(|(command, response)| {
                self.outgoing_event_blink();
                self.push_event(Event::Sent(command));
                self.request_repaint();

                response
            })
            .await
            .map(|response| {
                self.incoming_event_blink();
                self.push_event(Event::Received(response));
            })
            .map_err(|err| self.push_event(Event::Error(AppActionError::Unbind(err))));

        _ = client
            .close()
            .await
            .map(|()| {
                self.push_event(Event::Closed);
            })
            .map_err(|err| {
                self.push_event(Event::Error(AppActionError::Close(err)));
            });

        action.loading.store(false, Ordering::Relaxed);

        self.clear_client();
        self.request_repaint();
    }

    async fn deliver_sm_resp(&self, sequence_number: u32, deliver_sm_resp: DeliverSmResp) {
        let Some(client) = self.state.client().as_ref().map(|client| client.clone()) else {
            return;
        };

        _ = client
            .deliver_sm_resp(sequence_number, deliver_sm_resp.clone())
            .await
            .map(|_| {
                Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(sequence_number)
                    .pdu(deliver_sm_resp)
            })
            .map(|command| {
                self.outgoing_event_blink();
                self.push_event(Event::Sent(command));
                self.request_repaint();
            })
            .map_err(|err| {
                self.push_event(Event::Error(AppActionError::SubmitSm(err)));
            });

        self.request_repaint();
    }

    async fn submit_sms(&self, action: SubmitSmsAction) {
        let Some(client) = self.state.client().as_ref().map(|client| client.clone()) else {
            return;
        };

        for sm in action.sms {
            _ = client
                .send_mapped(sm)
                .and_then(|(command, response)| {
                    self.outgoing_event_blink();
                    self.push_event(Event::Sent(command));
                    self.request_repaint();

                    response
                })
                .await
                .map(|response| {
                    self.incoming_event_blink();
                    self.push_event(Event::Received(response));
                })
                .map_err(|err| {
                    self.push_event(Event::Error(AppActionError::SubmitSm(err)));
                });

            self.request_repaint();
        }
    }

    pub async fn run(self, mut actions: tokio::sync::mpsc::UnboundedReceiver<Action>) {
        while let Some(action) = actions.recv().await {
            match action {
                Action::Bind(action) => self.bind(action).await,
                Action::Unbind(action) => self.unbind(action).await,
                Action::SubmitSms(action) => self.submit_sms(action).await,
            }
        }
    }
}
