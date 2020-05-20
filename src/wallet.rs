use crate::response::ResponseBody;
//备用方法
//use crypto::{digest::Digest,ripemd160::Ripemd160};//{ ripemd160::Ripemd160, sha3::Sha3};
use actix_web::{post, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::sm3::Sm3;
use dislog_hal::Hasher;
use hex::ToHex;
use serde::{Deserialize, Serialize};

//register a new wallet
#[derive(Deserialize, Debug)]
pub struct NewWalletRequest {
    cert: String,
    info: Info,
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
pub async fn new_reg_wallet(req: web::Json<NewWalletRequest>) -> impl Responder {
    //备用方法
    //hash conversion
    /*let mut hasher = crypto::sha3::Sha3::sha3_256();
    hasher.input_str(&public_str);
    let hex = hasher.result_str();

    let mut ripemd = Ripemd160::new();
    ripemd.input_str(&hex);
    let uid_hasher = ripemd.result_str();*/

    //use Sm3算法实现hash转换
    let mut uid_hasher = Sm3::default();
    uid_hasher.update(&req.cert);
    let uid_str = uid_hasher.finalize().encode_hex();

    HttpResponse::Ok().json(ResponseBody::new_success(Some(NewWalletRespones {
        cert: req.cert.clone(),
        uid: uid_str,
    })))
}
