use std::{ops::Deref, sync::Arc};

use eframe::egui::Context;
use parking_lot::RwLock;
use rusmppc::Client;

use crate::values::Event;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Clone)]
pub struct EventsHolder {
    inner: Arc<AppStateInner>,
}

impl EventsHolder {
    /// Get a read-only reference to the events
    ///
    /// The returned reference must dropped as soon as possible to avoid blocking writes
    pub fn events(&self) -> impl Deref<Target = Vec<Event>> + '_ {
        self.inner.events.read()
    }
}

impl AppState {
    pub fn new(ctx: Context) -> Self {
        Self {
            inner: Arc::new(AppStateInner::new(ctx)),
        }
    }

    pub fn holder(&self) -> EventsHolder {
        EventsHolder {
            inner: self.inner.clone(),
        }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct AppStateInner {
    ctx: Context,
    client: Arc<RwLock<Option<Client>>>,
    events: RwLock<Vec<Event>>,
}

impl AppStateInner {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            client: Arc::new(RwLock::new(None)),
            events: RwLock::new(Vec::new()),
        }
    }

    pub fn set_client(&self, client: Client) {
        *self.client.write() = Some(client);
    }

    pub fn clear_client(&self) {
        *self.client.write() = None;
    }

    pub fn request_repaint(&self) {
        self.ctx.request_repaint();
    }

    pub fn is_bound(&self) -> bool {
        self.client.read().is_some()
    }

    pub fn push_event(&self, event: Event) {
        self.events.write().push(event);
    }

    pub fn extend_events(&self, events: impl Iterator<Item = Event>) {
        self.events.write().extend(events);
    }

    /// Get a read-only reference to the events
    ///
    /// The returned reference must dropped as soon as possible to avoid blocking writes
    pub fn events(&self) -> impl Deref<Target = Vec<Event>> + '_ {
        self.events.read()
    }

    pub fn client(&self) -> impl Deref<Target = Option<Client>> + '_ {
        self.client.read()
    }
}
