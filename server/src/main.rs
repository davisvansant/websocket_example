use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

#[derive(Debug, Clone)]
struct State {
    // clients: std::sync::Mutex<Vec<actix::Addr<WebsocketServer>>>,
    clients: Vec<actix::Addr<WebsocketServer>>,
}

impl Actor for State {
    // type Context = ws::WebsocketContext<Self>;
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("State initiatized");
    }
}

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
struct SomeMessage {
    something: String,
}

#[derive(Debug)]
struct WebsocketServer {
    host: actix_web::http::HeaderValue,
}

impl Actor for WebsocketServer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor Started");
        println!("{:?}", self);
        // println!("{:?}", ctx.set_mailbox_capacity(100));
    }
}

impl actix::Handler<SomeMessage> for WebsocketServer {
    type Result = ();

    fn handle(
        &mut self,
        msg: SomeMessage,
        _: &mut actix_web_actors::ws::WebsocketContext<Self>,
    ) -> Self::Result {
        println!("something");
        println!("{:?}", msg.something);
    }
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
                // ctx.text(text);
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

async fn echo(
    request: HttpRequest,
    stream: web::Payload,
    data: web::Data<std::sync::Mutex<Vec<String>>>,
    // state: web::Data<State>,
    state: web::Data<std::sync::Mutex<State>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", &request);
    // let host = &request.headers().get("host").unwrap().to_str().unwrap();
    // let header_value = actix_web::http::HeaderValue::from_str(host).unwrap();
    let client = &request
        .headers()
        .get("sec-websocket-key")
        .unwrap()
        .to_str()
        .unwrap();
    let header_value = actix_web::http::HeaderValue::from_str(client).unwrap();
    // let response = ws::start(WebsocketServer { host: header_value }, &request, stream);
    // println!("{:?}", response);
    // response
    let sender = ws::start_with_addr(WebsocketServer { host: header_value }, &request, stream);

    match sender {
        Ok(result) => {
            let actor = result.0;
            let message = result.1;
            // let mut managed = data.lock().unwrap();

            // state.clients.push(actor);
            // let mut clients = state.clients.lock().unwrap();
            // clients.push(actor);
            let mut clients = state.lock().unwrap();
            clients.clients.push(actor);
            // clients.push(actor);

            // managed.push(String::from("hi"));

            let arbiter = actix::Arbiter::current();
            println!("{:?}", arbiter);
            // println!("{:?}", actix::Arbiter::contains_item());
            let system = actix::System::current();
            println!("{:?}", system);

            // println!("{:?}", actor);

            let something_to_send = SomeMessage {
                something: String::from("welcome to websockets"),
            };

            // actor.do_send(something_to_send);

            // let recipient = actor.recipient();
            // match recipient.do_send(something_to_send) {
            //     Ok(_) => println!("Something has been sent"),
            //     Err(dunno) => println!("{:?}", dunno),
            // }

            // println!("{:?}", &managed);
            // println!("connected ? - {:?}", actor.connected());
            println!("{}", message.status().as_str());
            Ok(message)
        }
        Err(error) => Err(error),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let vec: Vec<String> = Vec::new();
    let app_state = std::sync::Mutex::new(vec);
    let state = web::Data::new(app_state);

    let state_vec: Vec<actix::Addr<WebsocketServer>> = Vec::with_capacity(10);
    let state_struct = State { clients: state_vec }.start();
    let state_mutex = std::sync::Mutex::new(state_struct);
    let system_state = web::Data::new(state_mutex);

    // let system_state = web::Data::new(State { clients: std::sync::Mutex::new(Vec::with_capacity(10)) }).start();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .data(system_state.clone())
            .route("/echo/", web::get().to(echo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
