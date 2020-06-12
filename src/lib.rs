#![warn(
    clippy::style,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::cargo
)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod todo;
pub mod user;

use anyhow::Result;
use env_logger::Env;

use crate::core::http::api_v2_operation;
use crate::core::http::web;

#[api_v2_operation]
async fn index() -> Result<String, ()> {
    Ok("Hello World".to_string())
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    let v1 = web::scope("/v1")
        .configure(todo::init)
        .configure(core::auth::init);

    cfg.route("/", web::get().to(index)).service(v1);
}

#[actix_rt::main]
pub async fn init() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting server");
    let settings = core::settings::Settings::new().unwrap();
    core::http::server(settings, routes).await?;
    Ok(())
}
