/// The API endpoints and models
pub mod api;
pub mod build_config;

use actix_files::Files;
use actix_web::web::ServiceConfig;

/// Configure the server (add the API and static endpoints)
pub fn config(serv: &mut ServiceConfig) {
    //serv.configure(api::config).service(Files::new("/", ))
}
