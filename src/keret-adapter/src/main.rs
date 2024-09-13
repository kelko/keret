mod listening;
mod sending;

use crate::listening::PortListener;
use crate::sending::ReportSender;
use clap::Parser;
use snafu::{ResultExt, Snafu};
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

#[derive(Debug, Snafu)]
pub(crate) enum AppError {
    #[snafu(display("An error occurred while listening on serial port"))]
    FailedListeningOnPort {
        #[snafu(backtrace)]
        source: listening::ListeningError,
    },
    #[snafu(display("An error occurred while trying to send report to target"))]
    FailedSendingToTarget {
        #[snafu(backtrace)]
        source: sending::SendingError,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let device = cli.device.to_str().unwrap();
    let url = cli.url;

    let mut listener = match listening::PortListener::new(device) {
        Ok(l) => l,
        Err(e) => {
            report(&e);
            std::process::exit(1);
        }
    };

    let sender = sending::ReportSender::new(url);

    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

        match read_and_forward(&mut listener, &sender).await {
            Ok(()) => {}
            Err(e) => {
                report(&e);
            }
        }
    }
}

async fn read_and_forward(
    listener: &mut PortListener,
    sender: &ReportSender,
) -> Result<(), AppError> {
    let report = listener
        .read_next_report()
        .context(FailedListeningOnPortSnafu)?;
    if let Some(report) = report {
        sender
            .send(report.duration())
            .await
            .context(FailedSendingToTargetSnafu)?;
    }

    Ok(())
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
