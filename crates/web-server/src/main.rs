mod config;
mod errors;
mod api_service;
mod root;
mod static_files;

use std::net::SocketAddr;

use axum::http::header;
use axum::middleware;
use axum::{extract::Request, response::Response};
use axum::{routing::{get, post}, Extension, Router};
use clorinde::deadpool_postgres::Manager;
use clorinde::tokio_postgres::NoTls;
use grpc_api::api::users_server::UsersServer;
use tower_livereload::LiveReloadLayer;
use tower_http::services::ServeDir;
use tonic::service::Routes;

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

    // gRPC routes on the same server/port.
    // We expose the same service under multiple prefixes to make it work cleanly with:
    // - direct gRPC clients that call `/api.Users/GetUsers`
    // - ingress setups that route by `/api` prefix (calls `/api/api.Users/GetUsers`)
    // - tools that prepend a custom base-path (e.g. `/api.users/getusers/...`)
    let reflection_service_v1 = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_api::FILE_DESCRIPTOR_SET)
        .build_v1()
        .expect("Failed to build gRPC reflection service (v1)");

    let reflection_service_v1alpha = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_api::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .expect("Failed to build gRPC reflection service (v1alpha)");

    // let grpc_service = tonic::transport::Server::builder()
    //     .accept_http1(true)
    //     .add_service(tonic_web::enable(crate::api::fortunes_server::FortunesServer::new(
    //         api_service::FortunesService { pool },
    //     )))
    //     .into_service();

    // let hybrid_make_service = hybrid::hybrid(axum_make_service, grpc_service);       

    let grpc_router = Routes::new(UsersServer::new(api_service::UsersService {
        pool: pool.clone(),
    }))
    .add_service(reflection_service_v1)
    .add_service(reflection_service_v1alpha)
    .prepare()
    .into_axum_router();

    // build our application with a route
    let http_app = Router::new()
        .route("/", get(root::loader))
        .route("/new_user", post(root::new_user_action))
        .route("/static/{*path}", get(static_files::static_path))
        .nest_service(
            "/dist",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../web-assets/dist")),
        );

    let app = http_app
        .merge(grpc_router.clone())
        .nest("/api", grpc_router.clone())
        .nest("/api.users/getusers", grpc_router)
        .layer(middleware::from_fn(log_grpc_requests))
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

async fn log_grpc_requests(req: Request, next: middleware::Next) -> Response {
    if let Some(ct) = req.headers().get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()) {
        if ct.starts_with("application/grpc") || ct.starts_with("application/grpc-web") {
            eprintln!("gRPC request: {} {} (content-type: {})", req.method(), req.uri().path(), ct);
        }
    }

    next.run(req).await
}