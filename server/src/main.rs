use actix_web::{web, App, HttpServer};

mod client;
mod handlers;
mod messages;
mod server;

use crate::client::Client;
use crate::handlers::echo;
use crate::messages::DelistClient;
use crate::messages::RegisterClient;
use crate::messages::Transmission;
use crate::server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/echo/", web::get().to(echo)))
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
