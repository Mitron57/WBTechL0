use {
    axum::routing::{get, post},
    clap::Parser,
    log::info,
    std::sync::Arc,
    wb_tech_l0::{
        application::controllers::{add_order, get_order},
        application::AppState,
        infrastructure::{Cache, Database, OrderService, Repository},
    },
};

#[derive(Parser, Debug)]
struct Args {
    //Database connection URI
    #[arg(short, long, required = true)]
    database: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:7878";
    env_logger::init();
    let args = Args::parse();
    let cache = Cache::new();
    let database = Database::new(args.database).await?;
    let repository = Box::new(Repository::new(cache, database));
    let order_service = Box::new(OrderService);
    let app_state = Arc::new(AppState::new(repository, order_service));
    let router = axum::Router::new()
        .route("/order/:order_uid", get(get_order))
        .route("/add_order", post(add_order))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on {addr}");
    Ok(axum::serve(listener, router).await?)
}
