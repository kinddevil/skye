pub mod apis;
pub mod middlewares;

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use log::{error, warn};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    dotenv().ok();

    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(e) => {
            error!("Read env error {:?}", e);
            String::from("8080")
        }
    };

    warn!("Start server at :{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::auth::Auth::new())
            .wrap(Logger::default())
            .service(web::resource("/wx_login/").route(web::post().to(apis::login::wx_login)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
