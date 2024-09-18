use tokio_postgres::error::{DbError, SqlState};
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
            Json(json!({"error" : "Order or its unique part already exists"})),
        );
    }
    drop(cache);
    let mut database = state.database().write().await;
    if database.get(&order.order_uid).await.is_some() {
        log!(target: "add_order", Level::Warn, "Insertion failed, reason: Order already exists");
        return (
            StatusCode::CONFLICT,
            Json(json!({"error" : "Order or its unique part already exists"})),
        );
    }
    if let Err(err) = database.insert(order).await {
        let db_error = err.downcast_ref::<tokio_postgres::Error>();
        if let None = db_error {
            log!(target: "add_order", Level::Error, "Insertion failed: err: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()})));
        }
        let db_error = db_error.unwrap();
        log!(target: "add_order", Level::Error, "Insertion failed: err: {}", db_error);
        return match db_error.code() {
            Some(&SqlState::FOREIGN_KEY_VIOLATION) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error" : "Can't save this order"})))
            },
            Some(&SqlState::UNIQUE_VIOLATION) => {
                (StatusCode::CONFLICT, Json(json!({"error" : "Order or its unique part already exists"})))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json::default()),
        }
    }
    (StatusCode::CREATED, Json::default())
}
