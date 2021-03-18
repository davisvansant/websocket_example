use crate::WebsocketServer;

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
pub struct RegisterClient(pub actix::Addr<WebsocketServer>);

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
pub struct DelistClient(pub actix::Addr<WebsocketServer>);

#[derive(Clone, Debug, actix::Message)]
#[rtype(result = "()")]
pub struct SomeMessage {
    pub something: String,
}
