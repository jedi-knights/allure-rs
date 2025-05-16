// crates/allure-server/src/lib.rs
use axum::{
    routing::get_service,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub async fn serve_report(report_dir: PathBuf, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().nest("/", get_service(ServeDir::new(report_dir)).handle_error(
        |error: std::io::Error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", error),
            )
        },
    ));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Serving report on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
