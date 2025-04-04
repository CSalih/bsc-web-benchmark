#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix_files;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod app;
mod constants;
mod error;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start conduit server...");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        use app::drivers::middlewares::state::AppState;
        AppState::new(pool)
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(app::drivers::middlewares::cors::cors())
            .wrap(app::drivers::middlewares::auth::Authentication)
            .service(actix_files::Files::new("/static", "./static"))
            .configure(app::drivers::routes::api)
    })
    .bind(constants::BIND)?
    .run()
    .await
}
