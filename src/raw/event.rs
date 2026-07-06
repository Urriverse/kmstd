//! Kernel event subscription and publishing interface.
//!
//! This module provides raw bindings for the kernel's event system, allowing
//! tasks to subscribe to, unsubscribe from, and publish system-wide events.

pub type EventId = u64;
pub type EventCallback = fn(EventId, usize);

Import! {
    pub fn EventSubscribe(event_id: EventId, callback: EventCallback) -> Result<(), ()> where kernel 0.1;
    pub fn EventUnsubscribe(event_id: EventId, callback: EventCallback) -> Result<(), ()> where kernel 0.1;
    pub fn EventPublish(event_id: EventId, data: usize, affinity: Option<usize>) -> Result<(), ()> where kernel 0.1;
}
