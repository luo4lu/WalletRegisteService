use crate::response::ResponseBody;
use actix_web::{get, post, web, HttpResponse, Responder};
use asymmetric_crypto::hasher::sm3::Sm3;
use dislog_hal::Hasher;
use hex::ToHex;
use log::{info, warn};
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
    let statement = match conn
        .prepare("INSERT INTO wallets (id,cert,info,create_time,update_time) VALUES ($1, $2, $3,now(),now())")
        .await
        {
            Ok(s) => {
                info!("database command success!");
                s
            }
            Err(error) =>{
                warn!("database command failed: {:?}",error);
                return HttpResponse::Ok().json(ResponseBody::<String>::database_runing_error(Some(error.to_string())));
            }
        };
    let jstr = serde_json::to_value(&req.info).unwrap();
    match conn
        .execute(&statement, &[&uid_str, &req.cert, &jstr])
        .await
    {
        Ok(s) => {
            info!("database parameter success!");
            s
        }
        Err(error) => {
            warn!("database parameter failed: {:?}", error);
            return HttpResponse::Ok().json(ResponseBody::<String>::database_runing_error(Some(
                error.to_string(),
            )));
        }
    };

    HttpResponse::Ok().json(ResponseBody::new_success(Some(NewWalletRespones {
        cert: req.cert.clone(),
        uid: uid_str,
    })))
}

//get wallet info
#[derive(Deserialize, Debug)]
pub struct GetWalletRequest {
    uid: String,
}

#[derive(Serialize)]
pub struct GetWalletRespones {
    cert: String,
    info: String,
}

//获取钱包信息
#[get("/api/wallet")]
pub async fn get_wallet_info(
    data: web::Data<Pool>,
    req: web::Json<GetWalletRequest>,
) -> impl Responder {
    //连接数据库获取句柄
    let conn = data.get().await.unwrap();
    let inset_statement = match conn
        .query("SELECT * from wallets where id = $1", &[&req.uid])
        .await
    {
        Ok(row) => {
            info!("select success!{:?}", row);
            row
        }
        Err(error) => {
            warn!("select failde!!{:?}", error);
            return HttpResponse::Ok().json(ResponseBody::<()>::database_build_error());
        }
    };
    if inset_statement.is_empty() {
        warn!("SELECT check uid failed,please check uid value");
        return HttpResponse::Ok().json(ResponseBody::<()>::database_build_error());
    }
    let v_cert = inset_statement[0].get(1);
    let v_info: serde_json::Value = inset_statement[0].get(2);
    let s_info = v_info.to_string();
    HttpResponse::Ok().json(ResponseBody::new_success(Some(GetWalletRespones {
        cert: v_cert,
        info: s_info,
    })))
}
