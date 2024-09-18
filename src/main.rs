use log::info;
use {
    application::AppState,
    infrastructure::{Cache, Database},
    std::sync::Arc,
    controllers::{add_order, get_order},
    axum::routing::{post, get},
    clap::Parser,
};

mod application;
mod controllers;
mod domain;
mod infrastructure;

#[derive(Parser, Debug)]
struct Args {
    //Database connection URI
    #[arg(short, long, required = true)]
    database: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let cache = Cache::new();
    let database = Database::new(args.database).await?;
    let app_state = Arc::new(AppState::new(cache, database));
    let router = axum::Router::new()
        .route("/order/:order_uid", get(get_order))
        .route("/add_order", post(add_order))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    info!("Listening on 127.0.0.1:7878");
    Ok(axum::serve(listener, router).await?)
}
