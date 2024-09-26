use {
    crate::{
        application::{AppState, controllers::error_handler},
        domain::{
            models::Order,
        },
    },
    axum::{extract::State, http::StatusCode, Json},
    serde_json::{Value, json},
    std::sync::Arc,
    log::{log, Level}
};

pub async fn add_order(
    State(state): State<Arc<AppState>>,
    Json(order): Json<Order>,
) -> (StatusCode, Json<Value>)
{
    log!(target: "add_order_controller", Level::Info, "Got new order: {order:?}");
    let result = state.order_service().add_order(state.repository(), order).await;
    if let Err(err) = result {
        return error_handler::handler(err);
    }
    (StatusCode::CREATED, Json(json!({})))
}
