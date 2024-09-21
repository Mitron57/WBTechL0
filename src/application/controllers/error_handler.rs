use std::error::Error;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;
use serde_json::json;
use tokio_postgres::error::SqlState;
use crate::infrastructure::MultiError;

pub fn handler(error: Box<dyn Error>) -> (StatusCode, Json<Value>) {
    let multi_error = error.downcast_ref::<MultiError>();
    if multi_error.is_some() {
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