use crate::config::Config;
use crate::response::ResponseBody;
//备用方法
//use crypto::{digest::Digest,ripemd160::Ripemd160};//{ ripemd160::Ripemd160, sha3::Sha3};
use actix_web::{post, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::{sha3::Sha3, sm3::Sm3};
use asymmetric_crypto::keypair::Keypair;
use dislog_hal::Hasher;
use hex::ToHex;
use serde::{Deserialize, Serialize};

use tokio::fs::File;
use tokio::prelude::*;
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
pub async fn new_reg_wallet(
    config: web::Data<Config>,
    req: web::Json<NewWalletRequest<Info>>,
) -> impl Responder {
    println!("{:?}", req);
    let mut file = match File::open(&config.cert_path).await{
        Ok(f) => f,
        Err(e) => {
            println!("file open failed:{:?}",e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        },
    };

    //read json file to string
    let mut contents = String::new();
    match file.read_to_string(&mut contents).await{
        Ok(s) => s,
        Err(e) => {
            println!("read file to string failed:{:?}",e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        },
    };

    //Deserialize to the specified data format
    let deserialize: Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = match serde_json::from_str(&contents){
        Ok(de) => de,
        Err(e) =>{
            println!("Keypair conversion failed:{:?}",e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        },
    };
    //format conversion to string
    let public_str: String = deserialize.get_public_key().to_bytes().encode_hex();

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
    uid_hasher.update(&public_str);
    let uid_str = uid_hasher.finalize().encode_hex();

    HttpResponse::Ok().json(ResponseBody::new_success(Some(NewWalletRespones {
        cert: public_str,
        uid: uid_str,
    })))
}
