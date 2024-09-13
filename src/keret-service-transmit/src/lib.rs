use std::time::{Duration};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionReport {
    timestamp: DateTime<Utc>,
    duration: Duration
}

impl From<u64> for ActionReport {
    fn from(value: u64) -> Self {
        Self {
            timestamp: Utc::now(),
            duration: Duration::from_micros(value)
        }
    }
}
