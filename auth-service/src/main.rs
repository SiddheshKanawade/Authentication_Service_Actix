mod config;
mod handlers;

use color_eyre::Result; //able to use ? operator
use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger}; // we need logger middlewar to give log output
use tracing::info;
use handlers::app_config; // imported the app_config function from handler

#[actix_rt::main]
async fn main() -> Result<()> {

    let config = Config::from_env()
        .expect("Server configuration");

    // defining HTTP srver
    info!("Starting server at http; //{}:{}/", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

     
    Ok(())

}
