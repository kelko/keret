mod metrics;

use crate::api::metrics::metrics_handler;
use crate::repository::ToDoRepository;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use keret_service_transmit::ActionReport;
use tracing::instrument;

pub(crate) fn setup_api<T: ToDoRepository + 'static>(repo: T) -> Router {
    // build our application with a route
    let todo_routes = Router::new()
        .route("/", get(list_entries::<T>).post(add_entry::<T>))
        .with_state(repo);

    Router::new()
        .route("/metrics", get(metrics_handler))
        .nest("/api/v1.0/report", todo_routes)
}

#[instrument(skip(repo))]
async fn list_entries<T: ToDoRepository>(
    State(repo): State<T>,
) -> Result<impl IntoResponse, StatusCode> {
    match repo.list() {
        Ok(list) => Ok(Json(list)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[instrument(skip(repo))]
async fn add_entry<T: ToDoRepository>(
    State(repo): State<T>,
    Json(entry): Json<ActionReport>,
) -> Result<impl IntoResponse, StatusCode> {
    match repo.add(entry) {
        Ok(index) => Ok(Json(index)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
