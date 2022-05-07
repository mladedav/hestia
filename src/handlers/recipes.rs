use std::collections::BTreeMap;
use std::env;

use diesel::prelude::*;
use dotenv::dotenv;
use handlebars::Handlebars;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::response::content::{self, Html};
use rocket::{get, State, post, uri};

use crate::db::models::{NewRecipeDb, RecipeDb, RecipeForm};
use crate::db::schema::recipes;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/<id>")]
pub async fn get(id: i32, tmpl: &State<Handlebars<'static>>) -> Option<Html<String>> {
    let mut conn = establish_connection();
    let recipe = match recipes::table.find(id).first::<RecipeDb>(&mut conn) {
        Ok(recipe) => recipe,
        Err(diesel::result::Error::NotFound) => return None,
        Err(e) => panic!("Unexpected error: {}", e),
    };

    let ingredients = recipe.ingredients.as_ref().map(|i| i.lines().map(|s| s.to_string()).collect::<Vec<String>>()).filter(|v| v.len() > 0);
    let content = recipe.content.clone().filter(|c| c.len() > 0).unwrap_or(String::from("No directions provided."));

    let mut data = BTreeMap::new();
    let recipe = serde_json::value::to_value(recipe).unwrap();
    for (key, value) in recipe.as_object().unwrap() {
        data.insert(key.as_str(), value.to_owned());
    }
    data.insert("ingredients", serde_json::value::to_value(ingredients).unwrap());
    data.insert("content", serde_json::value::to_value(content).unwrap());
    
    let html = tmpl.render("recipes/detail", &data).unwrap();

    Some(Html(html))
}

#[get("/")]
pub async fn list(tmpl: &State<Handlebars<'static>>) -> Html<String> {
    let mut conn = establish_connection();
    let recipes = recipes::table.limit(10).load::<RecipeDb>(&mut conn).expect("Error loading recipes");

    let mut data = BTreeMap::new();
    data.insert("recipes", recipes);
    let html = tmpl.render("recipes/list", &data).unwrap();

    Html(html)
}

#[get("/add")]
pub async fn add(tmpl: &State<Handlebars<'static>>) -> Html<String> {
    let html = tmpl.render("recipes/add", &()).unwrap();

    Html(html)
}

#[get("/edit/<id>")]
pub async fn edit(id: i32, tmpl: &State<Handlebars<'static>>) -> Option<Html<String>> {
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

    let html = tmpl.render("recipes/edit", &data).unwrap();

    Some(Html(html))
}

#[post("/<id>", data = "<recipe>")]
pub async fn update<'a>(id: i32, mut recipe: Form<RecipeForm<'a>>) -> Redirect {
    let recipe = recipe.into_db(id).await;

    let mut conn = establish_connection();
    let recipe: RecipeDb = diesel::replace_into(recipes::table)
        .values(&recipe)
        .get_result(&mut conn)
        .expect("Error saving new recipe");

    Redirect::to(uri!("/recipes", list))
}

// pub async fn add_recipe_url_encoded(web::Form(recipe): web::Form<NewRecipe>) -> Result<HttpResponse, Error> {
//     let mut conn = establish_connection();
//     let recipe: Recipe = diesel::insert_into(recipes::table)
//         .values(&recipe)
//         .get_result(&mut conn)
//         .expect("Error saving new recipe");

//     let location = format!("/recipes/{}", recipe.id);
//     Ok(HttpResponse::SeeOther().header(header::LOCATION, location).finish())
// }

#[post("/", data = "<recipe>")]
pub async fn insert<'a>(mut recipe: Form<RecipeForm<'a>>) -> Redirect {
    let recipe = recipe.into_new_db().await;

    let mut conn = establish_connection();
    let recipe: RecipeDb = diesel::insert_into(recipes::table)
        .values(&recipe)
        .get_result(&mut conn)
        .expect("Error saving new recipe");

    Redirect::to(uri!("/recipes", get(recipe.id)))
}

// pub async fn add_recipe_json(web::Json(recipe): web::Json<NewRecipe>) -> Result<HttpResponse, Error> {
//     let mut conn = establish_connection();
//     let recipe: Recipe = diesel::insert_into(recipes::table)
//         .values(&recipe)
//         // .returning(records::all_columns)
//         .get_result(&mut conn)
//         // .execute(conn)
//         .expect("Error saving new recipe");

//     Ok(HttpResponse::Ok().json(recipe))
// }

// pub async fn remove_recipe(web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
//     let mut conn = establish_connection();
//     diesel::delete(recipes::table.find(id)).execute(&mut conn).expect("Error deleting recipe");
//     Ok(HttpResponse::Ok().finish())
// }
