#![no_std]

// we need the postcard wrapper only on "no_std" environments as core::error::Error is not yet
// supported in stabled fashion in snafu.
// on environments with "std" (indicated by the "std" feature of _this_ crate)
// let snafu handle everything thanks to std::error::Error
#[cfg(not(feature = "std"))]
mod postcard_error;

use postcard::{from_bytes, to_vec};
use serde::{self, Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Serialization Failed"))]
    CantSerialize {
        #[cfg(feature = "std")]
        source: postcard::Error,

        #[cfg(not(feature = "std"))]
        #[snafu(source(from(postcard::Error, postcard_error::PostcardError::new)))]
        source: postcard_error::PostcardError,
    },
    #[snafu(display("Deserialization Failed"))]
    CantDeserialize {
        #[cfg(feature = "std")]
        source: postcard::Error,

        #[cfg(not(feature = "std"))]
        #[snafu(source(from(postcard::Error, postcard_error::PostcardError::new)))]
        source: postcard_error::PostcardError,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ActionReport {
    duration: u64,
}

impl ActionReport {
    pub fn new(duration: u64) -> Self {
        Self { duration }
    }

    pub fn from_message(data: &[u8]) -> Result<Self, Error> {
        from_bytes(data).context(CantDeserializeSnafu)
    }

    pub fn as_message(&self) -> Result<heapless::Vec<u8, 8>, Error> {
        to_vec(&self).context(CantSerializeSnafu)
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }
}
