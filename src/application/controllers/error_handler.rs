use {
    std::error::Error,
    axum::{http::StatusCode, Json},
    serde_json::{Value, json},
    tokio_postgres::error::SqlState,
    crate::infrastructure::MultiError
};

pub fn handler(error: Box<dyn Error>) -> (StatusCode, Json<Value>) {
    let multi_error = error.downcast_ref::<MultiError>();
    let pool_error = error.downcast_ref::<deadpool_postgres::PoolError>();
    if multi_error.is_some() || pool_error.is_some() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
    }
    let db_error = error.downcast_ref::<tokio_postgres::Error>();
    if db_error.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": error.to_string()})));
    }
    let db_error = db_error.unwrap();
    match db_error.code() {
        Some(&SqlState::FOREIGN_KEY_VIOLATION) => {
            (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"error" : "Can't save this order"})))
        },
        Some(&SqlState::UNIQUE_VIOLATION) => {
            (StatusCode::CONFLICT, Json(json!({"error" : "Order or its unique part already exists"})))
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))),
    }
}