#![no_std]

use postcard::{from_bytes, to_vec};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ActionReport {
    duration: u64,
}

impl ActionReport {
    pub fn new(duration: u64) -> Self {
        Self {
            duration
        }
    }

    pub fn from_message(data: &[u8]) -> Self {
        from_bytes(&data).unwrap()
    }

    pub fn as_message(&self) -> heapless::Vec<u8,8> {
        to_vec::<ActionReport,8>(&self).unwrap()
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }
}
