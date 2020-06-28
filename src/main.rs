use actix_web::{App, HttpServer};
use clap::ArgMatches;
use log::Level;

mod admin_cert;
mod config;
mod config_command;
pub mod response;
mod wallet;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut _path: String = String::new();
    let matches: ArgMatches = config_command::get_command();
    if let Some(d) = matches.value_of("wrs") {
        _path = d.to_string();
    } else {
        _path = String::from("127.0.0.1:9004");
    }

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
    .bind(_path)?
    .run()
    .await
}
