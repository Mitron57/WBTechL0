use {
    crate::{
        application::AppState,
        infrastructure,
        domain::{
            models::Order,
            interfaces::{Cache, Database},
        },
    },
    axum::{extract::State, http::StatusCode, Json},
    serde_json::{json, Value},
    std::sync::Arc,
    log::{log, Level}
};

pub async fn add_order(
    State(state): State<Arc<AppState<infrastructure::Cache, infrastructure::Database>>>,
    Json(order): Json<Order>,
) -> (StatusCode, Json<Value>)
{
    log!(target: "add_order", Level::Info, "Got new order: {:?}", order);
    let cache = state.cache().read().await;
    if cache.get(&order.order_uid).is_some() {
        log!(target: "add_order", Level::Warn, "Insertion failed, reason: Order already exists");
        return (
            StatusCode::CONFLICT,
            Json(json!({"error" : "Order already exists"})),
        );
    }
    drop(cache);
    let mut database = state.database().write().await;
    if database.get(&order.order_uid).await.is_some() {
        log!(target: "add_order", Level::Warn, "Insertion failed, reason: Order already exists");
        return (
            StatusCode::CONFLICT,
            Json(json!({"error" : "Order already exists"})),
        );
    }
    if let Err(err) = database.insert(order).await {
        log!(target: "add_order", Level::Error, "Insertion failed: err: {}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()})));
    }
    (StatusCode::CREATED, Json::default())
}
