
//TODO

use std::convert::Infallible;
use std::time::{Duration, Instant};

use salvo_core::async_trait;

use super::{RateStore, SimpleQuota, RateGuard};

#[derive(Clone, Debug)]
pub struct SlidingWindow {
    /// The time at which the window resets.
    reset: Instant,
    /// The number of requests made in the window.
    count: usize,
}

impl Default for SlidingWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SlidingWindow {
    pub fn new() -> Self {
        Self {
            reset: Instant::now(),
            count: 0,
        }
    }
}

#[async_trait]
impl RateGuard for SlidingWindow {
    type Quota = SimpleQuota;
    async fn verify(&mut self, quota: &Self::Quota) -> bool {
        if Instant::now() > self.reset {
            self.reset = Instant::now() + quota.period;
            self.count = 0;
        }
        if self.count < quota.burst {
            self.count += 1;
            true
        } else {
            false
        }
    }
}