use {
    crate::{
        application::AppState,
        infrastructure,
        domain::interfaces::{Cache, Database},
    },
    axum::{
        extract::{Path, State},
        http::StatusCode, 
        Json,
    },
    log::{log, Level},
    std::sync::Arc,
    serde_json::{json, Value}
};

pub async fn get_order(
    State(state): State<Arc<AppState<infrastructure::Cache, infrastructure::Database>>>,
    Path(order_uid): Path<String>,
) -> (StatusCode, Json<Value>) {
    log!(target: "get_order", Level::Info, "Get new get-request by order_uid: {order_uid}");
    let cache = state.cache().read().await;
    if let Some(order) = cache.get(&order_uid) {
        log!(target: "get_order", Level::Info, "Order found in cache: {order:?}");
        return (StatusCode::OK, Json(serde_json::to_value(order).unwrap()));
    }
    drop(cache);
    let database = state.database().read().await;
    if let Some(order) = database.get(&order_uid).await {
        state.cache().write().await.add(order_uid, order.clone());
        log!(target: "get_order", Level::Info, "Order found in database and copied to cache: {:?}", order);
        return (StatusCode::OK, Json(serde_json::to_value(order).unwrap()));
    }
    log!(target: "get_order", Level::Info, "No order found neither in database nor in cache, order_uid: {order_uid}");
    (StatusCode::NOT_FOUND, Json(json!({"error": "order with given uid not found"})))
}
