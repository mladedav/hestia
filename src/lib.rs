use rocket::{fs::FileServer, Rocket, Ignite, Error, routes};

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod handlebars;
mod authorization;

mod db;
mod handlers;

use handlers::recipes;

pub async fn build() -> Result<Rocket<Ignite>, Error> {
    rocket::build()
        .manage(handlebars::handlebars())
        .mount("/static", FileServer::from("static"))
        .mount("/pictures", FileServer::from("pictures"))
        .mount("/recipes", routes![recipes::get, recipes::list, recipes::edit, recipes::update, recipes::add, recipes::insert])
        .ignite()
        .await
}
