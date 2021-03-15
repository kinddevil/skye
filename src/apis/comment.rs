use actix_http::http::StatusCode;

use actix_web::{web, HttpRequest, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Comment {
    comment: String,
    author: String,
    timestamp: String,
}

pub async fn comment(info: web::Json<Comment>, _req: HttpRequest) -> HttpResponse {
    info!("{:?}", info);
    HttpResponse::build(StatusCode::OK).body("ok")
}

pub async fn comments(info: web::Path<String>, _req: HttpRequest) -> HttpResponse {
    info!("{:?}", info);
    HttpResponse::build(StatusCode::OK).body("ok")
}

#[cfg(test)]
mod login_tests {
    use super::*;
    use actix_web::{test, web::Bytes, App};

    #[actix_rt::test]
    #[ignore]
    async fn test_wx_login() {}
}
