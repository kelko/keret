mod app_service;
mod infrastructure;
mod model;

use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Device Path (/dev/...)
    device: PathBuf,

    /// full URL to post to
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let url = cli.url;
    let Some(device) = cli.device.to_str() else {
        eprintln!("--device must be valid UTF-8");
        std::process::exit(1);
    };

    let listener = match infrastructure::listening::PortListener::new(device) {
        Ok(l) => l,
        Err(e) => {
            report(&e);
            std::process::exit(1);
        }
    };

    let sender = infrastructure::sending::ReportSender::new(url);
    let mut app_service = app_service::ApplicationService::new(listener, sender);

    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

        match app_service.read_and_forward().await {
            Ok(()) => {}
            Err(e) => {
                report(&e);
            }
        }
    }
}

pub fn report<E>(err: &E)
where
    E: 'static,
    E: std::error::Error,
    E: snafu::ErrorCompat,
    E: Send + Sync,
{
    eprintln!("[ERROR] {}", err);
    if let Some(source) = err.source() {
        eprintln!();
        eprintln!("Caused by:");
        for (i, e) in std::iter::successors(Some(source), |e| e.source()).enumerate() {
            eprintln!("   {}: {}", i, e);
        }
    }

    if let Some(backtrace) = snafu::ErrorCompat::backtrace(err) {
        eprintln!("Backtrace:");
        eprintln!("{}", backtrace);
    }
}
