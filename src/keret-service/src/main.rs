use crate::api::setup_api;
use crate::repository::{StorageBasedRepository, YamlRepositoryStorage};

mod api;
mod repository;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let storage = YamlRepositoryStorage::new("repo.yaml");
    let repo = StorageBasedRepository::new(storage);
    let app = setup_api(repo);

    // run our app with hyper, listening globally on port 3000
    let Ok(listener) = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await else {
        eprintln!("Could not bind to port {}", PORT);
        std::process::exit(-1);
    };

    if axum::serve(listener, app).await.is_err() {
        eprintln!("Could you not start the API");
        std::process::exit(-2);
    };
}
