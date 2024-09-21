use {
    crate::{
        application::{AppState, controllers::error_handler},
    },
    axum::{
        extract::{Path, State},
        http::StatusCode,
        debug_handler,
        Json,
    },
    std::sync::Arc,
    serde_json::{Value, json},
    log::{log, Level}
};

#[debug_handler]
pub async fn get_order(
    State(state): State<Arc<AppState>>,
    Path(order_uid): Path<String>,
) -> (StatusCode, Json<Value>) {
    log!(target: "get_order_controller", Level::Info, "Got new get-request by order_uid: {order_uid}");
    match state.order_service().get_order(&order_uid, state.repository_mut()).await {
        Ok(Some(order)) => {
            (StatusCode::OK, Json(serde_json::to_value(order).unwrap()))
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(json!({"error": "Order with given uid not found"})))
        },
        Err(err) => {
            error_handler::handler(err)
        }
    }
    
}
