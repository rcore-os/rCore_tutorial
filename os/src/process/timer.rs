//! A naive timer

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::time::Duration;

/// A naive timer
#[derive(Default)]
pub struct Timer {
    events: BTreeMap<Duration, Callback>,
}

/// The type of callback function.
type Callback = Box<dyn FnOnce() + Send + Sync + 'static>;

impl Timer {
    /// Add a timer with given `deadline`.
    ///
    /// The `callback` will be called on timer expired.
    pub fn add(&mut self, deadline: Duration, callback: impl FnOnce() + Send + Sync + 'static) {
        self.events.insert(deadline, Box::new(callback));
    }

    /// Called on each tick.
    ///
    /// The caller should give the current time `now`, and all expired timer will be trigger.
    pub fn tick(&mut self, now: Duration) {
        while let Some(entry) = self.events.first_entry() {
            if *entry.key() > now {
                return;
            }
            let (_, callback) = entry.remove_entry();
            callback();
        }
    }
}
