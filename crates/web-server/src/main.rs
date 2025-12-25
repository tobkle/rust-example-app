mod config;
mod errors;
mod root;
mod static_files;

use std::net::SocketAddr;

use axum::{routing::{get, post}, Extension, Router};
use clorinde::deadpool_postgres::Manager;
use clorinde::tokio_postgres::NoTls;
use tower_livereload::LiveReloadLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pg_config: clorinde::tokio_postgres::Config = config
        .database_url
        .parse()
        .expect("DATABASE_URL is invalid");

    let manager = Manager::new(pg_config, NoTls);
    
    let pool = clorinde::deadpool_postgres::Pool::builder(manager)
        .build()
        .expect("Failed to build database pool");

    // build our application with a route
    let app = Router::new()
        .route("/", get(root::loader))
        .route("/new_user", post(root::new_user_action))
        .route("/static/*path", get(static_files::static_path))
        .nest_service(
            "/dist",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../web-assets/dist")),
        )
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}