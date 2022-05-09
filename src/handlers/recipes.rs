use std::collections::BTreeMap;
use std::env;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{get, post, uri, State};
use rocket_dyn_templates::{Template, context};

use crate::db::models::{RecipeDb, RecipeForm};
use crate::db::schema::recipes;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/<id>")]
pub async fn get(id: i32) -> Option<Template> {
    let mut conn = establish_connection();
    let recipe = match recipes::table.find(id).first::<RecipeDb>(&mut conn) {
        Ok(recipe) => recipe,
        Err(diesel::result::Error::NotFound) => return None,
        Err(e) => panic!("Unexpected error: {}", e),
    };

    Some(Template::render("recipes/detail", &recipe))
}

#[get("/")]
pub async fn list() -> Template {
    let mut conn = establish_connection();
    let recipes = recipes::table
        .limit(10)
        .load::<RecipeDb>(&mut conn)
        .expect("Error loading recipes");

    Template::render("recipes/list", context! {
        recipes: &recipes
    })
}

#[get("/add")]
pub async fn add() -> Template {
    Template::render("recipes/add", &())
}

#[get("/edit/<id>")]
pub async fn edit(id: i32) -> Option<Template> {
    let mut conn = establish_connection();
    let recipe = match recipes::table.find(id).first::<RecipeDb>(&mut conn) {
        Ok(recipe) => recipe,
        Err(diesel::result::Error::NotFound) => return None,
        Err(e) => panic!("Unexpected error: {}", e),
    };

    let mut data = BTreeMap::new();
    let recipe = serde_json::value::to_value(recipe).unwrap();
    for (key, value) in recipe.as_object().unwrap() {
        data.insert(key.as_str(), value.to_owned());
    }

    Some(Template::render("recipes/edit", &data))
}

#[post("/<id>", data = "<recipe>")]
pub async fn update(id: i32, mut recipe: Form<RecipeForm<'_>>) -> Redirect {
    let recipe = recipe.as_db(id).await;

    let mut conn = establish_connection();
    diesel::replace_into(recipes::table)
        .values(&recipe)
        .get_result::<RecipeDb>(&mut conn)
        .expect("Error saving new recipe");

    Redirect::to(uri!("/recipes", list))
}

#[post("/", data = "<recipe>")]
pub async fn insert(mut recipe: Form<RecipeForm<'_>>) -> Redirect {
    let recipe = recipe.as_new_db().await;

    let mut conn = establish_connection();
    let recipe: RecipeDb = diesel::insert_into(recipes::table)
        .values(&recipe)
        .get_result(&mut conn)
        .expect("Error saving new recipe");

    Redirect::to(uri!("/recipes", get(recipe.id)))
}
