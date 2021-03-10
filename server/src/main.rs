use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

async fn index() -> Result<HttpResponse, Error> {
    let mut builder = actix_web::dev::HttpResponseBuilder::new(actix_web::http::StatusCode::OK);
    let body = actix_web::dev::Body::from("Hi from websockets!");
    let response = builder.body(body);
    println!("{:?}", response);
    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
