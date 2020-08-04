use crate::config::ConfigPath;
use crate::response::ResponseBody;
use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder};
use asymmetric_crypto::hasher::sha3::Sha3;
use asymmetric_crypto::keypair::Keypair;
use core::convert::AsRef;
use hex::{FromHex, ToHex};
use log::{info, warn};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use tokio::fs::File;
use tokio::prelude::*;
//read cert
#[get("/api/cert")]
pub async fn read_cert(config: web::Data<ConfigPath>) -> impl Responder {
    //read file
    let mut file = match File::open(&config.cert_path).await {
        Ok(f) => {
            info!("{:?}", f);
            f
        }
        Err(e) => {
            warn!("file open failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        }
    };
    //read json file to String
    let mut contents = String::new();
    match file.read_to_string(&mut contents).await {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("read file to string failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };

    //Deserialize to the specified data format
    let deserialized: Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = match serde_json::from_str(&contents) {
        Ok(de) => {
            info!("{:?}", de);
            de
        }
        Err(e) => {
            warn!("Keypair conversion failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //get public key of response
    HttpResponse::Ok().json(ResponseBody::new_success(Some(
        deserialized.get_public_key(),
    )))
}

//init create a new register system cert
#[post("/api/admin/cert")]
pub async fn new_reg_cert(config: web::Data<ConfigPath>) -> impl Responder {
    //decline a rand number object
    let mut rng = thread_rng();
    //generate Serialize structure data
    let info_form_rang = match Keypair::<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    >::generate(&mut rng)
    {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("keypair conversion failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    let serialized = match serde_json::to_string(&info_form_rang) {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("serialized to string failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    let mut file = match File::create(&config.cert_path).await {
        Ok(f) => {
            info!("{:?}", f);
            f
        }
        Err(e) => {
            warn!("file create failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        }
    };
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
    config: web::Data<ConfigPath>,
    req: web::Json<UpdateCertRequest>,
) -> impl Responder {
    let sd: [u8; 32] = match FromHex::from_hex(&req.seed) {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("32 byte from hex failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    let info_form_rang = match Keypair::<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    >::generate_from_seed(sd)
    {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("keypair generate failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    let serialized = match serde_json::to_string(&info_form_rang) {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("serialized to string failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    let mut file = match File::create(&config.cert_path).await {
        Ok(f) => {
            info!("{:?}", f);
            f
        }
        Err(e) => {
            warn!("file create failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        }
    };
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
pub async fn read_reg_cert(config: web::Data<ConfigPath>) -> impl Responder {
    //read file
    let mut file = match File::open(&config.cert_path).await {
        Ok(f) => {
            info!("{:?}", f);
            f
        }
        Err(e) => {
            warn!("file open failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        }
    };
    //read json file to String
    let mut contents = String::new();
    match file.read_to_string(&mut contents).await {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("read file to string failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };

    //Deserialize to the specified data format
    let keypair_value: Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = match serde_json::from_str(&contents) {
        Ok(de) => {
            info!("{:?}", de);
            de
        }
        Err(e) => {
            warn!("Keypair generate failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //format conversion to string
    let secret_str = keypair_value.get_secret_key().to_bytes().encode_hex();
    let code_str = keypair_value.get_code().encode_hex();
    let seed_str = keypair_value.get_seed().encode_hex();

    //get public key of response
    HttpResponse::Ok().json(ResponseBody::new_success(Some(ReadCertResponse {
        secret: secret_str,
        code: code_str,
        seed: seed_str,
    })))
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    url: String,
    extra: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct DcdsRegistRequest {
    cert: String,
    extra: serde_json::Value,
    #[serde(rename = "type")]
    t: String,
}

//注册证书信息到中心管理系统
#[post("/api/admin/cms")]
pub async fn register_cms(
    config: web::Data<ConfigPath>,
    req: web::Json<RegisterRequest>,
    req_head: HttpRequest,
) -> impl Responder {
    //获取请求头中的uuid
    let http_head = req_head.headers();
    let head_value = http_head.get("X-CLOUD-USER_ID").unwrap();
    let head_str = head_value.to_str().unwrap();
    let head_name: &str = &*String::from("X-CLOUD-USER_ID");
    //read file
    let mut file = match File::open(&config.cert_path).await {
        Ok(f) => {
            info!("{:?}", f);
            f
        }
        Err(e) => {
            warn!("file open failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_file_error());
        }
    };
    //read json file to string
    let mut contents = String::new();
    match file.read_to_string(&mut contents).await {
        Ok(s) => {
            info!("{:?}", s);
            s
        }
        Err(e) => {
            warn!("read file to string failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    //Deserialize to the specified data format
    let keypair_value: Keypair<
        [u8; 32],
        Sha3,
        dislog_hal_sm2::PointInner,
        dislog_hal_sm2::ScalarInner,
    > = match serde_json::from_str(&contents) {
        Ok(de) => {
            info!("{:?}", de);
            de
        }
        Err(e) => {
            warn!("Keypair generate failed:{:?}", e);
            return HttpResponse::Ok().json(ResponseBody::<()>::new_str_conver_error());
        }
    };
    let public_str = keypair_value.get_public_key().to_bytes().encode_hex();
    let params = DcdsRegistRequest {
        cert: public_str,
        extra: req.extra.clone(),
        t: String::from("WRS"),
    };
    let client = reqwest::Client::new();
    let _res = client
        .post(&req.url)
        .header(head_name, head_str)
        .json(&params)
        .send()
        .await;
    HttpResponse::Ok().json(ResponseBody::<()>::new_success(None))
}
