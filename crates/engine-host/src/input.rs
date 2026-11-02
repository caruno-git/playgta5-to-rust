//! Input.
//!
//! In the browser the engine called `env.wasm_input_poll` from its main loop and
//! the JS shell pushed DOM events into a queue. Server builds replace the DOM
//! with a channel: netcode, a replay file, or a test harness writes
//! [`InputEvent`]s and the engine drains them exactly the same way.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Mutex;

use crate::host::{HostError, HostResult};
use crate::time::Micros;

/// Event class — matches the engine's `InputKind` enum in `game_input`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputKind {
    /// Keyboard scan/virtual code in `code`, `value` is 0 up / 1 down.
    Key,
    /// Mouse button in `code`, `value` is 0 up / 1 down.
    MouseButton,
    /// Mouse delta in `code` (x) and `value` (y), or absolute when the source
    /// reports positions.
    MouseMove,
    /// Analog axis (pad stick, trigger) in `code`, `value` in `-32768..=32767`.
    Axis,
    /// UTF-32 code point typed this frame.
    Text,
}

/// One queued input event. `at` is the host clock reading the sample was taken,
/// so the engine can interpolate between polls instead of assuming the poll
/// instant is the input instant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputEvent {
    pub kind: InputKind,
    pub code: u16,
    pub value: i32,
    pub at: Micros,
}

impl InputEvent {
    pub fn new(kind: InputKind, code: u16, value: i32, at: Micros) -> Self {
        Self {
            kind,
            code,
            value,
            at,
        }
    }

    pub fn key(code: u16, down: bool, at: Micros) -> Self {
        Self::new(InputKind::Key, code, i32::from(down), at)
    }
}

/// `env.wasm_input_poll` — drain queued events into the engine's frame buffer.
pub trait InputSource: Send + Sync {
    /// Append everything queued to `out`; returns how many were appended.
    /// Never blocks and never fails because the queue is empty.
    fn poll(&self, out: &mut Vec<InputEvent>) -> HostResult<usize>;
}

/// `std::sync::mpsc`-backed source. The receiver lives behind a `Mutex`
/// because `Receiver` is `Send` but not `Sync`, and the engine polls from one
/// thread while producers live elsewhere.
pub struct ChannelInputSource {
    rx: Mutex<Receiver<InputEvent>>,
    closed: AtomicBool,
}

/// Producer half. Cloneable, `Send`, held by netcode/replay/test code.
#[derive(Clone, Debug)]
pub struct InputSender {
    tx: Sender<InputEvent>,
}

impl InputSender {
    /// Queue an event. Fails with [`HostError::Closed`] once the engine's source
    /// has been dropped.
    pub fn send(&self, event: InputEvent) -> HostResult<()> {
        self.tx.send(event).map_err(|_| HostError::Closed)
    }
}

impl ChannelInputSource {
    /// Create a source and the sender that feeds it.
    pub fn new() -> (Self, InputSender) {
        let (tx, rx) = mpsc::channel();
        (
            Self {
                rx: Mutex::new(rx),
                closed: AtomicBool::new(false),
            },
            InputSender { tx },
        )
    }

    /// A source that can never receive events (browser stub, disabled input).
    pub fn disconnected() -> Self {
        let (tx, rx) = mpsc::channel();
        drop(tx);
        Self {
            rx: Mutex::new(rx),
            closed: AtomicBool::new(true),
        }
    }
}

impl Default for ChannelInputSource {
    fn default() -> Self {
        Self::new().0
    }
}

impl InputSource for ChannelInputSource {
    fn poll(&self, out: &mut Vec<InputEvent>) -> HostResult<usize> {
        let rx = self
            .rx
            .lock()
            .map_err(|_| HostError::Thread("input queue poisoned".into()))?;
        let mut taken = 0;
        loop {
            match rx.try_recv() {
                Ok(event) => {
                    out.push(event);
                    taken += 1;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.closed.store(true, Ordering::Relaxed);
                    break;
                }
            }
        }
        Ok(taken)
    }
}

impl std::fmt::Debug for ChannelInputSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelInputSource")
            .field("closed", &self.closed.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_drains_in_order_and_reports_the_count() {
        let (source, sender) = ChannelInputSource::new();
        let mut out = Vec::new();
        assert_eq!(source.poll(&mut out).expect("empty poll"), 0);

        sender
            .send(InputEvent::key(0x41, true, Micros(10)))
            .expect("send");
        sender
            .send(InputEvent::new(InputKind::MouseMove, 0, 7, Micros(11)))
            .expect("send");
        assert_eq!(source.poll(&mut out).expect("poll"), 2);
        assert_eq!(out[0], InputEvent::key(0x41, true, Micros(10)));
        assert_eq!(out[1].kind, InputKind::MouseMove);
        assert_eq!(out[1].value, 7);

        // Poll is destructive: a second poll sees nothing new.
        assert_eq!(source.poll(&mut out).expect("poll again"), 0);
        assert_eq!(out.len(), 2);
        assert_eq!(out[1].at, Micros(11));
    }

    #[test]
    fn polling_before_any_send_is_cheap_and_empty() {
        let (source, _sender) = ChannelInputSource::new();
        let mut out = Vec::with_capacity(4);
        for _ in 0..100 {
            assert_eq!(source.poll(&mut out).expect("poll"), 0);
        }
        assert!(out.is_empty());
    }

    #[test]
    fn disconnected_source_reports_empty_not_error() {
        let source = ChannelInputSource::disconnected();
        let mut out = Vec::new();
        assert_eq!(source.poll(&mut out).expect("poll"), 0);
        assert_eq!(source.poll(&mut out).expect("poll"), 0);
    }
}