use std::sync::Arc;

use axum::{Router, extract::State, routing::get};
use tokio::sync::Mutex;

mod frontend;

#[derive(Default)]
struct AppState {}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new().route("/", get(index)).with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(state): State<Arc<Mutex<AppState>>>) -> String {
    let mut state = state.lock().await; // &mut AppState
    String::from("pee")
}
