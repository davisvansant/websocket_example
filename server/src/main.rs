use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

#[derive(Debug, Default, Clone)]
struct State {
    // clients: std::sync::Mutex<Vec<actix::Addr<WebsocketServer>>>,
    // clients: Vec<actix::Addr<WebsocketServer>>,
    clients: Vec<actix::Recipient<SomeMessage>>,
}

// impl State {
//     pub async fn collector(&mut self, actor: actix::Addr<WebsocketServer>) {
//         // self.clients.push(actor);
//         self.clients.push(actor.recipient());
//         println!("addr pushed");
//         println!("{:?}", self.clients);
//     }
// }

impl Actor for State {
    // type Context = ws::WebsocketContext<Self>;
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("State initiatized");
        println!("{:?}", ctx);
    }
}

impl actix::Handler<RegisterClient> for State {
    type Result = ();

    fn handle(
        &mut self,
        RegisterClient(client): RegisterClient,
        context: &mut actix::Context<Self>,
    ) -> Self::Result {
        println!("Before message - {:?}", self.clients.len());
        self.clients.push(client.recipient());
        println!("After message = {:?}", self.clients.len());
    }
}

impl actix::Handler<SomeMessage> for State {
    type Result = ();

    fn handle(&mut self, msg: SomeMessage, context: &mut actix::Context<Self>) -> Self::Result {
        println!("do I have clients?{:?}", self.clients.len());
        println!("State has received something!");
        println!("The message state received = {:?}", msg.something);

        for c in self.clients.iter() {
            println!("am I connected? {:?}", c.connected());
            let msg = msg.clone();
            c.do_send(msg).unwrap();
        }
    }
}

impl actix::Supervised for State {}

impl actix::SystemService for State {
    fn service_started(&mut self, ctx: &mut actix::Context<Self>) {
        println!("Service started");
    }
}

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
struct RegisterClient(actix::Addr<WebsocketServer>);

#[derive(Clone, Debug, actix::Message)]
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
        use actix::AsyncContext;
        use actix::SystemService;

        let addr = State::from_registry();

        addr.do_send(RegisterClient(ctx.address()));
    }
}

impl actix::Handler<SomeMessage> for WebsocketServer {
    type Result = ();

    fn handle(
        &mut self,
        msg: SomeMessage,
        context: &mut actix_web_actors::ws::WebsocketContext<Self>,
    ) -> Self::Result {
        println!("Whoami? {:?}", self);
        println!("Websocket Actor has received something");
        println!("Message = {:?}", msg.something);

        context.text(msg.something);
    }
}

impl actix::Message for WebsocketServer {
    type Result = ();
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

                use actix::SystemService;
                let addr = State::from_registry();

                addr.do_send(SomeMessage { something: text });
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

async fn echo(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", &request);
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

            let arbiter = actix::Arbiter::current();
            println!("{:?}", arbiter);
            let system = actix::System::current();
            println!("{:?}", system);
            println!("{}", message.status().as_str());
            Ok(message)
        }
        Err(error) => Err(error),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/echo/", web::get().to(echo)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
