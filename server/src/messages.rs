use crate::Client;

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
pub struct RegisterClient(pub actix::Addr<Client>);

#[derive(Debug, actix::Message)]
#[rtype(result = "()")]
pub struct DelistClient(pub actix::Addr<Client>);

#[derive(Clone, Debug, actix::Message)]
#[rtype(result = "()")]
pub struct Transmission {
    pub data: String,
}
