use actix_web::{ App, HttpServer, middleware};
use crate::vendor::{Configuration, database};
use actix_web::dev::Server;
use crate::state::State;

pub fn strike() -> Server {
    let config = Configuration::new();

    let host = config.get("THUNDER_HOST").expect("THUNDER_HOST is not set");
    let database_url = config.get("THUNDER_DATABASE").expect("THUNDER_DATABASE is not set");
    let database = database::build(database_url.as_str());
    HttpServer::new(move || App::new()
        .data(State{
            database: database.clone(),
            configuration: config.clone()
        })
        .configure(crate::router::get)
        .wrap(middleware::Logger::default()))
        .bind(host.to_owned()).expect("Cannot start server")
        .start()
}