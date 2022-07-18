#[macro_use]
extern crate diesel;
extern crate rocket;

use std::{fs::DirBuilder, path::Path};

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

use handlers::auth;
use handlers::recipes;

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

    let rocket = rocket::build()
        .attach(Template::custom(|engines| {
            handlebars::customize(&mut engines.handlebars);
        }))
        .mount("/static", FileServer::from("static"))
        .mount("/pictures", FileServer::from(config.pictures_dir.clone()))
        .mount(
            "/recipes",
            routes![
                recipes::list,
                recipes::list_paged,
                recipes::get,
                recipes::edit,
                recipes::update,
                recipes::add,
                recipes::add_redirect,
                recipes::insert
            ],
        )
        .mount("/", routes![auth::callback, auth::logout,])
        .mount("/", routes![handlers::redirect_recipes,]);

    let rocket_config: rocket::Config = rocket.figment().extract().unwrap();

    DirBuilder::new()
        .recursive(true)
        .create(Path::new(&config.pictures_dir))
        .unwrap();
    DirBuilder::new()
        .recursive(true)
        .create(Path::new(rocket_config.temp_dir.original()))
        .unwrap();

    rocket
}
