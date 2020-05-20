use actix_web::{App, HttpServer};
use log::Level;
use simple_logger;

mod admin_cert;
mod config;
pub mod response;
mod wallet;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();
    HttpServer::new(|| {
        App::new()
            .data(config::Config::default())
            .service(admin_cert::read_cert)
            .service(admin_cert::new_reg_cert)
            .service(admin_cert::update_reg_cert)
            .service(admin_cert::read_reg_cert)
            .service(wallet::new_reg_wallet)
    })
    .bind("127.0.0.1:8808")
    .unwrap()
    .run()
    .await
}
