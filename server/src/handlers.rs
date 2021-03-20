use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

// use crate::WebsocketServer;
use crate::Client;

pub async fn echo(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", &request);
    let client = &request
        .headers()
        .get("sec-websocket-key")
        .unwrap()
        .to_str()
        .unwrap();
    let header_value = actix_web::http::HeaderValue::from_str(client).unwrap();

    // ws::start(WebsocketServer { host: header_value }, &request, stream)
    ws::start(Client { host: header_value }, &request, stream)
}
