use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

use crate::DelistClient;
use crate::RegisterClient;
// use crate::SomeMessage;
use crate::Transmission;
// use crate::State;
use crate::Server;

#[derive(Debug)]
pub struct Client {
    pub host: actix_web::http::HeaderValue,
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor Started");
        println!("{:?}", self);
        use actix::AsyncContext;
        use actix::SystemService;

        // let addr = State::from_registry();
        let addr = Server::from_registry();

        addr.do_send(RegisterClient(ctx.address()));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor Stopped");
        println!("{:?}", self);
        use actix::AsyncContext;
        use actix::SystemService;

        // let addr = State::from_registry();
        let addr = Server::from_registry();

        addr.do_send(DelistClient(ctx.address()));
    }
}

// impl actix::Handler<SomeMessage> for Client {
impl actix::Handler<Transmission> for Client {
    type Result = ();

    fn handle(
        &mut self,
        // msg: SomeMessage,
        transmission: Transmission,
        context: &mut actix_web_actors::ws::WebsocketContext<Self>,
    ) -> Self::Result {
        println!("Whoami? {:?}", self);
        println!("Websocket Actor has received something");
        // println!("Message = {:?}", msg.something);
        println!("Message = {:?}", transmission.data);

        // context.text(msg.something);
        context.text(transmission.data);
    }
}

impl actix::Message for Client {
    type Result = ();
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
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
                // let addr = State::from_registry();
                let addr = Server::from_registry();

                // addr.do_send(SomeMessage { something: text });
                addr.do_send(Transmission { data: text });
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
