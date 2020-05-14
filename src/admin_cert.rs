use crate::config::Config;
use crate::response::ResponseBody;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::sha3::Sha3;
use asymmetric_crypto::keypair::Keypair;
use core::convert::AsRef;
use hex::FromHex;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

use tokio::fs::File;
use tokio::prelude::*;
//read cert
#[get("/api/cert")]
pub async fn read_cert(config: web::Data<Config>) -> impl Responder {
    //read file
    let mut file = File::open(&config.cert_path).await.unwrap();
    //read json file to String
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    //Deserialize to the specified data format
    let deserialized: Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = serde_json::from_str(&contents).unwrap();
    //get public key of response
    HttpResponse::Ok().json(ResponseBody::new_success(Some(
        deserialized.get_public_key(),
    )))
}

//init create a new register system cert
#[post("/api/admin/cert")]
pub async fn new_reg_cert(config: web::Data<Config>) -> impl Responder {
    //decline a rand number object
    let mut rng = thread_rng();
    //generate Serialize structure data
    let info_form_rang = Keypair::<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    >::generate(&mut rng)
    .unwrap();
    let serialized = serde_json::to_string(&info_form_rang).unwrap();
    let mut file = File::create(&config.cert_path).await.unwrap();
    match file.write_all(serialized.as_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_success(None)),
        Err(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_json_parse_error()),
    }
}

//update register system cert
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCertRequest {
    seed: String,
}

#[put("/api/admin/cert")]
pub async fn update_reg_cert(
    config: web::Data<Config>,
    req: web::Json<UpdateCertRequest>,
) -> impl Responder {
    let sd: [u8; 32] = FromHex::from_hex(&req.seed).unwrap();
    let info_form_rang = Keypair::<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    >::generate_from_seed(sd)
    .unwrap();
    let serialized = serde_json::to_string(&info_form_rang).unwrap();
    let mut file = File::create(&config.cert_path).await.unwrap();
    match file.write_all(serialized.as_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_success(None)),
        Err(_) => HttpResponse::Ok().json(ResponseBody::<()>::new_json_parse_error()),
    }
}

//read register system cert
#[derive(Serialize)]
pub struct ReadCertResponse {
    secret: String,
    code: String,
    seed: String,
}

#[get("/admin/cert")]
pub async fn read_reg_cert(config: web::Data<Config>) -> impl Responder {
    //read file
    let mut file = File::open(&config.cert_path).await.unwrap();
    //read json file to String
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    //Deserialize to the specified data format
    let deserialized: Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = serde_json::from_str(&contents).unwrap();
    //format conversion to string
    let mut secret_str = String::new();
    let mut code_str = String::new();
    let mut seed_str = String::new();
    for a in deserialized.get_secret_key().to_bytes().iter() {
        write!(secret_str, "{:02x}", a).unwrap();
    }
    for a in deserialized.get_code().iter() {
        write!(code_str, "{:02x}", a).unwrap();
    }
    for a in deserialized.get_seed().iter() {
        write!(seed_str, "{:02x}", a).unwrap();
    }
    //get public key of response
    HttpResponse::Ok().json(ResponseBody::new_success(Some(ReadCertResponse {
        secret: secret_str,
        code: code_str,
        seed: seed_str,
    })))
}
