use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

struct WebsocketServer {
    host: actix_web::http::HeaderValue,
}

impl Actor for WebsocketServer {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("ping received!");
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(text)) => {
                println!("text recieved!");
                println!("client - {:?}", self.host);
                println!("test - {:?}", &text);
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(close)) => {
                println!("closing...",);
                if let Some(close_reason) = &close {
                    println!("Client - {:?}", self.host);
                    println!("Closing with the following code : {:?}", close_reason.code);
                    if let Some(description) = &close_reason.description {
                        println!("Closing with the following description : {}", description);
                    }
                }
                ctx.close(close);
            }
            _ => (),
        }
    }
}

async fn index(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", &request);
    let host = &request.headers().get("host").unwrap().to_str().unwrap();
    let header_value = actix_web::http::HeaderValue::from_str(host).unwrap();
    let response = ws::start(WebsocketServer { host: header_value }, &request, stream);
    println!("{:?}", response);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
