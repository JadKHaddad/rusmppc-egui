use core::f64;
use std::{
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

#[derive(Clone)]
pub struct BlinkerHandle {
    blink: Arc<AtomicBool>,
    ctx: egui::Context,
}

impl BlinkerHandle {
    pub fn new(blink: Arc<AtomicBool>, ctx: egui::Context) -> Self {
        Self { blink, ctx }
    }

    pub fn blink(&self) {
        self.blink.store(true, std::sync::atomic::Ordering::Relaxed);
        self.ctx.request_repaint();
    }
}

pub struct Blinker {
    blink: Arc<AtomicBool>,
    blink_requested_at: f64,
    blink_for: Duration,
}

impl Blinker {
    pub fn new(blink_for: Duration, ctx: egui::Context) -> (Self, BlinkerHandle) {
        let blink = Arc::new(AtomicBool::new(false));
        (
            Self {
                blink: blink.clone(),
                // Prevent blinking on startup
                blink_requested_at: f64::MAX,
                blink_for,
            },
            BlinkerHandle::new(blink, ctx),
        )
    }

    pub fn blink(&mut self, ui: &egui::Ui) -> bool {
        let now = ui.input(|i| i.time);

        let blink = self.blink.swap(false, std::sync::atomic::Ordering::Relaxed);

        if blink {
            self.blink_requested_at = now;
        }

        let should_blink = now > self.blink_requested_at
            && now < self.blink_requested_at + self.blink_for.as_secs_f64();

        if should_blink {
            ui.ctx().request_repaint_after(self.blink_for);
        }

        should_blink
    }
}
