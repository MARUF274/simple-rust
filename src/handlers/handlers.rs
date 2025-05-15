use actix_web::{ web, HttpResponse, Result, Responder, Error };
use serde_json::Value;

use crate::helper::{
    clickhouse::{ ch_query, Query },
    parser::{ map_get, to_f64, to_str, ValueParser },
};

// Handler for getting all users
pub async fn get_users(mut info: web::Json<Value>) -> Result<impl Responder, Error> {
    let param = info.take();
    println!("device : {param}");

    let query =
        format!("SELECT * FROM default.message ORDER BY created_at ASC LIMIT 10");
    let rows = ch_query("CH", Query::Select(query)).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .get_or_null("data")
        .as_array_or_default();

    Ok(HttpResponse::Ok().json(rows))
}

pub async fn insert_message(mut info: web::Json<Value>) -> Result<impl Responder, Error> {
    let param = info.take();
    let message = to_str(map_get("message", param.clone()));
    println!("MEssage: {}", message);

    if !param.as_object().map_or(false, |obj| obj.contains_key("message")) {
        return Err(actix_web::error::ErrorBadRequest("Missing 'message' field in request body"));
    }
    // let message_str = param.clone();

    println!("device : {param}");

    let query =
        format!("INSERT INTO default.message (message, created_at) values('{}', null) ", message);

    let result = ch_query("CH", Query::Insert(query)).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Ok().body("Message inserted successfully"))
        }
        Err(e) => {
            eprintln!("Error inserting message: {}", e);
            Err(actix_web::error::ErrorInternalServerError(format!("Failed to insert message: {}", e)))
        }
    }
}

// Health check handler
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
