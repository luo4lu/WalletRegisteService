use actix_web::{App, HttpServer};
use log::Level;
use std::env;

mod admin_cert;
mod config;
pub mod response;
mod wallet;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //Initialize the log and set the print level
    simple_logger::init_with_level(Level::Warn).unwrap();
    HttpServer::new(|| {
        App::new()
            .data(config::get_db())
            .data(config::ConfigPath::default())
            .service(admin_cert::read_cert)
            .service(admin_cert::new_reg_cert)
            .service(admin_cert::update_reg_cert)
            .service(admin_cert::read_reg_cert)
            .service(wallet::new_reg_wallet)
            .service(wallet::get_wallet_info)
            .service(admin_cert::register_cms)
    })
    .bind(&args[1])?
    .unwrap()
    .run()
    .await
}
