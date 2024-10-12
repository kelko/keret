#![cfg_attr(not(test), no_std)]
mod app_service;
mod error;
pub mod ports;

pub use app_service::ApplicationService;
pub use error::Error;
