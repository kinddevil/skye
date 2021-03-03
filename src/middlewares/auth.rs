use std::pin::Pin;

use actix_service::{Service, Transform};
use std::task::{Context, Poll};

use actix_web::{dev::ServiceRequest, dev::ServiceResponse, error::ErrorUnauthorized, Error};
use futures::future::{ok, Ready};
use futures::Future;

const TOKEN_NAME: &str = "TOKEN";
const TOKEN: &str = "df235527-3cfc-4925-89ba-fc0d5f70dcef";
#[derive(Default)]
pub struct Auth;

impl Auth {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        match req.headers().get(TOKEN_NAME) {
            Some(value) if value == TOKEN => {
                let fut = self.service.call(req);
                Box::pin(async { Ok(fut.await?) })
            }
            // _ => Box::pin(Err(error::ErrorUnauthorized("Unauthorized")))
            _ => Box::pin(ok(req.error_response(ErrorUnauthorized("Unauthorized")))),
        }
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use actix_http::http::Version;
    use actix_web::http::StatusCode;
    use actix_web::test;
    #[actix_rt::test]
    async fn test_auth_success() {
        let mut auth = Auth.new_transform(test::ok_service()).await.unwrap();
        let req = test::TestRequest::with_uri("test").version(Version::HTTP_2).header("TOKEN", TOKEN).to_srv_request();
        let resp = test::call_service(&mut auth, req).await;
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn test_auth_failed() {
        let mut auth = Auth.new_transform(test::ok_service()).await.unwrap();
        let req = test::TestRequest::with_uri("test").version(Version::HTTP_2).header("TOKEN", "Sometoken").to_srv_request();
        let resp = test::call_service(&mut auth, req).await;
        assert_eq!(StatusCode::UNAUTHORIZED, resp.status());
    }
}
