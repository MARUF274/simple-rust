use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use crate::handlers::*;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Service Running")
}

//* template name service path : /json-api-<name_page>-<service_name>-<piechart/trend/tabular/bar>

pub fn routes(service: &mut ServiceConfig) {
    service.route("/", web::get().to(index))
    .route("/get_message",web::post().to(get_users))
    .route("/insert_message",web::post().to(insert_message));
}
