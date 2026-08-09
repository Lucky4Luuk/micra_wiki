#[macro_use]
extern crate log;

use std::sync::Arc;

use axum::{Router, extract::State, routing::get};
use tokio::sync::Mutex;

mod db;
mod frontend;

use frontend::PageTemplater;

use crate::db::Database;

struct AppState {
    db: Database,
    templater: PageTemplater,
}

impl AppState {
    pub async fn new() -> Self {
        let db = Database::load_or_init("db_root")
            .await
            .expect("Failed to load or create database!");
        info!("Database loaded succesfully!");

        let templater = PageTemplater::new();

        Self { db, templater }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::max())
        .init();

    info!("Hello wiki!");

    let state = Arc::new(Mutex::new(AppState::new().await));

    let app = Router::new().route("/", get(index)).with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(state): State<Arc<Mutex<AppState>>>) -> String {
    // let mut state = state.lock().await;
    String::from("pee")
}
