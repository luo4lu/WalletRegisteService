use crate::response::ResponseBody;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

//read cert
#[get("/api/cert")]
pub async fn read_cert() -> impl Responder {
    let data = String::from("");
    HttpResponse::Ok().json(ResponseBody::new_success(Some(data)))
}

//init create a new register system cert
#[post("/api/admin/cert")]
pub async fn new_reg_cert() -> impl Responder {
    HttpResponse::Ok().json(ResponseBody::<()>::new_success(None))
}

//update register system cert
#[derive(Deserialize, Debug)]
pub struct UpdateCertRequest {
    seed: String,
}

#[put("/api/admin/cert")]
pub async fn update_reg_cert(req: web::Json<UpdateCertRequest>) -> impl Responder {
    format!("{:?}", req);
    HttpResponse::Ok().json(ResponseBody::<()>::new_success(None))
}

//read register system cert
#[derive(Serialize)]
pub struct ReadCertResponse {
    secret: String,
    code: String,
    seed: String,
}

#[get("/admin/cert")]
pub async fn read_reg_cert() -> impl Responder {
    HttpResponse::Ok().json(ResponseBody::new_success(Some(ReadCertResponse {
        secret: String::from(""),
        code: String::from(""),
        seed: String::from(""),
    })))
}
