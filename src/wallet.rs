use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::sm3::Sm3;
use dislog_hal::Hasher;
use hex::ToHex;
use serde::{Deserialize, Serialize};
//数据库相关
use deadpool_postgres::Pool;

//register a new wallet
#[derive(Deserialize, Debug)]
pub struct NewWalletRequest {
    cert: String,
    info: Info,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    name: String,
    idcard: String,
    info: String,
}

#[derive(Serialize)]
pub struct NewWalletRespones {
    cert: String,
    uid: String,
}

#[post("/api/wallet")]
pub async fn new_reg_wallet(
    data: web::Data<Pool>,
    req: web::Json<NewWalletRequest>,
) -> impl Responder {
    //use Sm3算法实现hash转换
    let mut uid_hasher = Sm3::default();
    uid_hasher.update(&req.cert);
    let uid_str = uid_hasher.finalize().encode_hex();
    //插入数据库
    let conn = data.get().await.unwrap();
    let statement = conn
        .prepare("INSERT INTO wallets (id,cert,info,create_time,update_time) VALUES ($1, $2, $3,now(),now())")
        .await
        .unwrap();
    let jstr = serde_json::to_value(&req.info).unwrap();
    conn.execute(&statement, &[&uid_str, &req.cert, &jstr])
        .await
        .unwrap();

    HttpResponse::Ok().json(ResponseBody::new_success(Some(NewWalletRespones {
        cert: req.cert.clone(),
        uid: uid_str,
    })))
}
