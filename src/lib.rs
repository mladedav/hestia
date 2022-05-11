#[macro_use]
extern crate diesel;
extern crate rocket;

use handlers::recipes;
use once_cell::sync::OnceCell;
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

mod authorization;
mod db;
mod handlebars;
mod handlers;

#[derive(Debug, Deserialize)]
struct Config {
    db_file: String,
    pictures_dir: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn build() -> Rocket<Build> {
    let figment = Figment::new()
        .merge(Toml::file("hestia.toml").nested())
        .merge(Env::prefixed("HESTIA_").global())
        .select(Profile::from_env_or(
            "HESTIA_ENVIRONMENT",
            Profile::const_new("local"),
        ));

    let config: Config = figment.extract().expect("Unable to parse configuration");
    CONFIG.set(config).unwrap();
    let config = CONFIG.get().unwrap();

    rocket::build()
        .attach(Template::custom(|engines| {
            handlebars::customize(&mut engines.handlebars);
        }))
        .mount("/static", FileServer::from("static"))
        .mount("/pictures", FileServer::from(config.pictures_dir.clone()))
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
