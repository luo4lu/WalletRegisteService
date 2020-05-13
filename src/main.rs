use actix_web::{App, HttpServer};

mod admin_cert;
mod admin_wallet;
mod config;
pub mod response;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(config::Config::default())
            .service(admin_cert::read_cert)
            .service(admin_cert::new_reg_cert)
            .service(admin_cert::update_reg_cert)
            .service(admin_cert::read_reg_cert)
            .service(admin_wallet::new_reg_wallet)
    })
    .bind("127.0.0.1:8808")
    .unwrap()
    .run()
    .await
}
