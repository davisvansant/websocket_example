use actix::Actor;

use crate::DelistClient;
use crate::RegisterClient;
use crate::Transmission;

#[derive(Debug, Default, Clone)]
pub struct Server {
    clients: Vec<actix::Recipient<Transmission>>,
}

impl Actor for Server {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("State initiatized");
        println!("{:?}", ctx);
    }
}

impl actix::Handler<RegisterClient> for Server {
    type Result = ();

    fn handle(
        &mut self,
        RegisterClient(client): RegisterClient,
        _context: &mut actix::Context<Self>,
    ) -> Self::Result {
        println!("Before message - {:?}", self.clients.len());

        self.clients.push(client.recipient());

        println!("After message = {:?}", self.clients.len());
    }
}

impl actix::Handler<DelistClient> for Server {
    type Result = ();

    fn handle(
        &mut self,
        DelistClient(client): DelistClient,
        _context: &mut actix::Context<Self>,
    ) -> Self::Result {
        println!("Before message - {:?}", self.clients.len());

        self.clients.retain(|c| *c != client.clone().recipient());

        println!("After message = {:?}", self.clients.len());
    }
}

impl actix::Handler<Transmission> for Server {
    type Result = ();

    fn handle(
        &mut self,
        transmission: Transmission,
        _context: &mut actix::Context<Self>,
    ) -> Self::Result {
        println!("do I have clients?{:?}", self.clients.len());
        println!("State has received something!");
        println!("The message state received = {:?}", transmission.data);

        for c in self.clients.iter() {
            println!("am I connected? {:?}", c.connected());

            let transmission = transmission.clone();
            c.do_send(transmission).unwrap();
        }
    }
}

impl actix::Supervised for Server {}

impl actix::SystemService for Server {
    fn service_started(&mut self, _ctx: &mut actix::Context<Self>) {
        println!("Service started");
    }
}
