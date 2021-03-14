use actix_http::error::PayloadError;
use actix_http::http::StatusCode;
use actix_web::client::{Client, SendRequestError};

use actix_web::{web, HttpRequest, HttpResponse};
use derive_more::{Display, From};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::error;
use std::time::Duration;

const APPID: &str = "wx4c70a4fd3673d59d";
const APPSECRET: &str = "cf2cebdf2a0eac87e6bb8fc606e209db";
const GRANT_TYPE: &str = "authorization_code";

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthCode {
    code: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct WxSession {
    openid: Option<String>,
    session_key: Option<String>,
    unionid: Option<String>,
    errcode: Option<i32>,
    errmsg: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct MockJson {
    id: i32,
    user_id: i32,
    title: String,
    completed: bool,
}

#[derive(Debug, Display)]
#[display(fmt = "client error: {} - {} \n {:?}", name, status, msg)]
pub struct ClientError {
    name: &'static str,
    status: StatusCode,
    msg: &'static str,
}

impl error::Error for ClientError {}

#[derive(Debug, Display, From)]
pub enum ClientErrors {
    PayloadError(PayloadError),
    SendRequestError(SendRequestError),
    SerdeError(serde_json::error::Error),
    ClientError(ClientError),
}

impl std::error::Error for ClientErrors {}

pub async fn wx_login(info: web::Query<AuthCode>, _req: HttpRequest) -> HttpResponse {
    match get_session(&info.code).await {
        Ok(data) => {
            info!("Get data: {:?}", data);
            // TODO save cache
            HttpResponse::Ok().body("Hello")
        }
        Err(e) => match e {
            ClientErrors::ClientError(e) => {
                error!("{:?}", e);
                HttpResponse::build(e.status).body(e.msg)
            }
            ClientErrors::PayloadError { .. } | ClientErrors::SendRequestError { .. } | ClientErrors::SerdeError { .. } => {
                error!("internal {:?}", e);
                HttpResponse::InternalServerError().body("internal server error")
            }
        },
    }
}

async fn get_session(code: &str) -> Result<WxSession, ClientErrors> {
    let client = Client::new();

    let url = format!(
        "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type={}",
        APPID, APPSECRET, code, GRANT_TYPE
    );
    // let url = format!("https://jsonplaceholder.typicode.com/todos/{}", code);
    info!("Login: {}", url);

    let mut res = client
        // .get("https://www.amazon.com/error")
        .get(url)
        .timeout(Duration::from_secs(2))
        .send()
        .await?;

    let bytes = res.body().await?;
    match res.status() {
        StatusCode::OK => {
            info!("get bytes {:?}", std::str::from_utf8(&bytes));
            let data: WxSession = serde_json::from_slice(&bytes)?;
            // let data: MockJson = serde_json::from_slice(&bytes)?;
            info!("Get data {:?}", data);
            Ok(data)
        }
        status => Err(ClientErrors::from(ClientError { name: "http error", status, msg: "" })),
    }
}

#[cfg(test)]
mod login_tests {
    use super::*;
    use actix_web::{test, web::Bytes, App};

    #[actix_rt::test]
    #[ignore]
    async fn test_wx_login() {
        let mut app = test::init_service(App::new().service(web::resource("/").route(web::post().to(wx_login)))).await;
        let req = test::TestRequest::post().uri("/?code=12").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello"));
    }
}
