use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

//register a new wallet
#[derive(Deserialize, Debug)]
pub struct NewWalletRequest<T> {
    cert: String,
    info: T,
}
#[derive(Deserialize, Debug)]
pub struct Info {
    name: String,
    idcard: String,
    other: String,
}

#[derive(Serialize)]
pub struct NewWalletRespones {
    cert: String,
    uid: String,
}

#[post("/api/wallet")]
pub async fn new_reg_wallet(req: web::Json<NewWalletRequest<Info>>) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().json(ResponseBody::new_success(Some(NewWalletRespones {
        cert: String::from(""),
        uid: String::from(""),
    })))
}
