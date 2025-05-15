mod handlers;
mod routes;
mod helper;

// use actix_web::{App, HttpServer};
use actix_web::{web, App, HttpResponse, HttpServer, http::header, error};
use dotenv::dotenv;
use helper::io::get_env_or_default;
use routes::routes;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_host = get_env_or_default("SERVER_HOST", "127.0.0.1".to_string());
    let server_port = get_env_or_default("SERVER_PORT", 8000);
    
    println!("service running in\n port : {server_port} \n host : {server_host} ");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
            
        // custom `Json` extractor configuration
        let json_cfg = web::JsonConfig::default()
            .limit(104857600)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        App::new().wrap(cors).configure(routes).app_data(json_cfg)
    })
    .bind((server_host, server_port))?
    .run()
    .await
}