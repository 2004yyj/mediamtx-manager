mod error;
mod routes;

use std::net::SocketAddr;
use std::sync::Arc;

use mediamtx_manager_core::{AppConfig, AppState};
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfig::default();
    let state = Arc::new(AppState::new(config));

    let api_router = routes::create_router(state);

    // 프론트엔드 static 파일 서빙 (SPA fallback)
    let frontend_dir = std::env::var("FRONTEND_DIR").unwrap_or_else(|_| "frontend/dist".into());
    let spa_fallback = ServeFile::new(format!("{frontend_dir}/index.html"));
    let static_files = ServeDir::new(&frontend_dir).not_found_service(spa_fallback);

    let app = api_router
        .fallback_service(static_files)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Web server listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
