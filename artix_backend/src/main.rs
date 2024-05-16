use actix_web::{App, get, http, web, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
mod core;
use crate::core::generate_response;
// mod handlers;
// mod routes;
// mod state;

#[derive(Deserialize)]
struct InputData {
    message: String,
}

#[derive(Serialize)]
struct ResponseData {
    response: String,
}

async fn process_json(input_data: web::Json<InputData>) -> impl Responder {
    println!("Received message: {}", input_data.message);
    let response_text = generate_response(input_data.message.clone()).unwrap();
    let response_data = ResponseData {
        response: response_text,
    };

    HttpResponse::Ok().json(response_data)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive()
            // .allowed_origin("http://localhost:3000/")
            // .allowed_origin_fn(|origin, _req_head| {
            // origin.as_bytes().ends_with(b".localhost:3000")
            // })
            // .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            // .max_age(3600)
            ;

        App::new()
            .wrap(cors)
            // .service(index)
            .route("/", web::post().to(process_json))
            // .data(state::Message::new())
            // .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
