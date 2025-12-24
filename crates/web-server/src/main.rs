mod config;
mod errors;
mod root;

use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use clorinde::deadpool_postgres::Manager;
use clorinde::tokio_postgres::NoTls;
use tower_livereload::LiveReloadLayer;

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