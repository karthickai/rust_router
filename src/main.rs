extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate easy_password;

use actix_files as fs;
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    http::header, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod auth_handler;
mod cpu_handler;
mod errors;
mod ip_handler;
mod models;
mod mstp_handler;
mod mstp_status_handler;
mod register_handler;
mod schema;
mod utils;

fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("pong")
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let address: String =
        std::env::var("ADDRESS").unwrap_or_else(|_| "http://localhost".to_string());
    println!("{}", address);
    // create db connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            .data(web::JsonConfig::default().limit(4096))
            .wrap(
                Cors::new().supports_credentials()
            )

            .service(
                web::scope("/api")
                    .service(web::resource("/ping").to(ping))
                    .service(web::resource("/get/cpu").to(cpu_handler::cpu))
                    .service(web::resource("/get/ip").to(ip_handler::get_ip))
                    .service(web::resource("/get/mstp").to(mstp_handler::get_mstp))
                    .service(
                        web::resource("/get/mstp/status/deep").to(mstp_status_handler::deep_scan),
                    )
                    .service(
                        web::resource("/get/mstp/status/quick").to(mstp_status_handler::quick_scan),
                    )
                    .service(web::resource("/post/ip").route(web::post().to(ip_handler::post_ip)))
                    .service(
                        web::resource("/post/mstp").route(web::post().to(mstp_handler::post_mstp)),
                    )
                    .service(
                        web::scope("/auth")
                            .service(
                                web::resource("/register")
                                    .route(web::post().to_async(register_handler::create_user)),
                            )
                            .service(
                                web::resource("/login")
                                    .route(web::post().to_async(auth_handler::login)),
                            )
                            .service(
                                web::resource("/logout").route(web::get().to(auth_handler::logout)),
                            )
                            .service(
                                web::resource("/update_password")
                                    .route(web::post().to_async(auth_handler::update_password)),
                            ),
                    ),
            )
            


    })
    .bind("0.0.0.0:8000")
    .expect("Cannot bind to 0.0.0.0:8000")
    .workers(3)
    .run()
    
}
