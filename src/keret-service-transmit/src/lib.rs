use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ActionReport {
    timestamp: DateTime<Utc>,
    duration: Duration,
}

impl ActionReport {
    pub fn new(timestamp: DateTime<Utc>, duration: Duration) -> Self {
        Self {
            timestamp,
            duration,
        }
    }
}

impl From<u64> for ActionReport {
    fn from(value: u64) -> Self {
        Self {
            timestamp: Utc::now(),
            duration: Duration::from_secs(value),
        }
    }
}
