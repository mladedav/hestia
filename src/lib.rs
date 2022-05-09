#[macro_use]
extern crate diesel;
extern crate rocket;

use std::sync::Arc;

use diesel::{Connection, SqliteConnection};
use handlers::recipes;
use rocket::{
    figment::{
        providers::{Env, Format, Toml},
        Figment, Profile,
    },
    fs::FileServer,
    routes, Build, Rocket,
};
use rocket_dyn_templates::Template;
use serde::Deserialize;
use tokio::sync::Mutex;

mod authorization;
mod db;
mod handlebars;
mod handlers;

#[derive(Debug, Deserialize)]
struct Config {
    db_file: String,
    pictures_dir: String,
}

pub fn build() -> Rocket<Build> {
    let figment = Figment::new()
        .merge(Toml::file("hestia.toml").nested())
        .merge(Env::prefixed("HESTIA_").global())
        .select(Profile::from_env_or(
            "HESTIA_ENVIRONMENT",
            Profile::const_new("local"),
        ));

    let config: Config = figment.extract().expect("Unable to parse configuration");

    let db_url = &config.db_file;
    let connection = SqliteConnection::establish(db_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {}", db_url, e));

    rocket::build()
        .attach(Template::custom(|engines| {
            // handlebars::setup(&mut engines.handlebars);
            handlebars::customize(&mut engines.handlebars);
        }))
        .manage(Arc::new(Mutex::new(connection)))
        .mount("/static", FileServer::from("static"))
        .mount("/pictures", FileServer::from(config.pictures_dir))
        .mount(
            "/recipes",
            routes![
                recipes::list,
                recipes::get,
                recipes::edit,
                recipes::update,
                recipes::add,
                recipes::insert
            ],
        )
}
