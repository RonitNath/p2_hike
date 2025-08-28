mod state;
mod error;

use state::AppState;
use error::AppError;

use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    assert!(dotenvy::dotenv().is_ok());

    let _ = fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .try_init();

    let state = AppState::new();

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    tracing::info!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    state.render("base.html", None)
}
