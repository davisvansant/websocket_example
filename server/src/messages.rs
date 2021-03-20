// use crate::WebsocketServer;
use crate::Client;

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
// pub struct RegisterClient(pub actix::Addr<WebsocketServer>);
pub struct RegisterClient(pub actix::Addr<Client>);

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
// pub struct DelistClient(pub actix::Addr<WebsocketServer>);
pub struct DelistClient(pub actix::Addr<Client>);

#[derive(Clone, Debug, actix::Message)]
#[rtype(result = "()")]
// pub struct SomeMessage {
//     pub something: String,
// }
pub struct Transmission {
    pub data: String,
}
