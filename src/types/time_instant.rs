//! Метка времени
//!
//! Обертка над `std::time::Instant`
//! Добавлена поддрежка Default

use std::time::Instant;

use crate::types;

#[derive(Clone)]
pub struct TimeInstant {
    inst: Instant,
}

impl TimeInstant {
    pub fn now() -> Self {
        Self {
            inst: Instant::now(),
        }
    }

    pub fn duration_since(&self, earlier: Self) -> types::TimeDuration {
        self.inst.duration_since(earlier.inst)
    }

    pub fn elapsed(&self) -> types::TimeDuration {
        self.inst.elapsed()
    }
}

impl Default for TimeInstant {
    fn default() -> Self {
        Self {
            inst: Instant::now(),
        }
    }
}
