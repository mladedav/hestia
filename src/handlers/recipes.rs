use std::collections::BTreeMap;
use std::env;

use actix_web::http::header;
use actix_web::{web, Error, HttpResponse, HttpRequest};

use diesel::prelude::*;
use dotenv::dotenv;
use handlebars::Handlebars;

use crate::db::models::{NewRecipe, Recipe};
use crate::db::schema::recipes;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_recipe(req: HttpRequest, tmpl: web::Data<Handlebars<'static>>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut conn = establish_connection();
    let recipe = match recipes::table.find(id).first::<Recipe>(&mut conn) {
        Ok(recipe) => recipe,
        Err(diesel::result::Error::NotFound) => return super::default::p404(req).await,
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

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

pub async fn list_recipes(tmpl: web::Data<Handlebars<'static>>) -> Result<HttpResponse, Error> {
    let mut conn = establish_connection();
    let recipes = recipes::table.limit(10).load::<Recipe>(&mut conn).expect("Error loading recipes");

    let mut data = BTreeMap::new();
    data.insert("recipes", recipes);
    let html = tmpl.render("recipes/list", &data).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

pub async fn add_form(tmpl: web::Data<Handlebars<'static>>) -> Result<HttpResponse, Error> {
    let html = tmpl.render("recipes/add", &()).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

pub async fn edit_recipe(req: HttpRequest, tmpl: web::Data<Handlebars<'static>>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut conn = establish_connection();
    let recipe = match recipes::table.find(id).first::<Recipe>(&mut conn) {
        Ok(recipe) => recipe,
        Err(diesel::result::Error::NotFound) => return super::default::p404(req).await,
        Err(e) => panic!("Unexpected error: {}", e),
    };

    let mut data = BTreeMap::new();
    let recipe = serde_json::value::to_value(recipe).unwrap();
    for (key, value) in recipe.as_object().unwrap() {
        data.insert(key.as_str(), value.to_owned());
    }

    let html = tmpl.render("recipes/edit", &data).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

pub async fn update_recipe_multipart(mut parts: awmp::Parts, web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    let qs = parts.texts.to_query_string();
    let mut recipe = web::Query::<Recipe>::from_query(&qs).unwrap().into_inner();
    // In case someone tries to update something they should not
    recipe.id = id;

    recipe.picture = parts
        .files
        .take("picture")
        .pop()
        .and_then(|f| f.persist_in("./pictures/").ok())
        .map(|p| p.to_str().unwrap().to_string());

    let mut conn = establish_connection();
    let recipe: Recipe = diesel::replace_into(recipes::table)
        .values(&recipe)
        .get_result(&mut conn)
        .expect("Error saving new recipe");

    let location = format!("/recipes/{}", recipe.id);
    Ok(HttpResponse::SeeOther().header(header::LOCATION, location).finish())
}

pub async fn add_recipe_url_encoded(web::Form(recipe): web::Form<NewRecipe>) -> Result<HttpResponse, Error> {
    let mut conn = establish_connection();
    let recipe: Recipe = diesel::insert_into(recipes::table)
        .values(&recipe)
        .get_result(&mut conn)
        .expect("Error saving new recipe");

    let location = format!("/recipes/{}", recipe.id);
    Ok(HttpResponse::SeeOther().header(header::LOCATION, location).finish())
}

pub async fn add_recipe_multipart(mut parts: awmp::Parts) -> Result<HttpResponse, Error> {
    let qs = parts.texts.to_query_string();
    let mut recipe = web::Query::<NewRecipe>::from_query(&qs).unwrap().into_inner();

    recipe.picture = parts
        .files
        .take("picture")
        .pop()
        .and_then(|f| f.persist_in("./pictures/").ok())
        .map(|p| p.to_str().unwrap().to_string());

    let mut conn = establish_connection();
    let recipe: Recipe = diesel::insert_into(recipes::table)
        .values(&recipe)
        .get_result(&mut conn)
        .expect("Error saving new recipe");

    let location = format!("/recipes/{}", recipe.id);
    Ok(HttpResponse::SeeOther().header(header::LOCATION, location).finish())
}

pub async fn add_recipe_json(web::Json(recipe): web::Json<NewRecipe>) -> Result<HttpResponse, Error> {
    let mut conn = establish_connection();
    let recipe: Recipe = diesel::insert_into(recipes::table)
        .values(&recipe)
        // .returning(records::all_columns)
        .get_result(&mut conn)
        // .execute(conn)
        .expect("Error saving new recipe");

    Ok(HttpResponse::Ok().json(recipe))
}

pub async fn remove_recipe(web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut conn = establish_connection();
    diesel::delete(recipes::table.find(id)).execute(&mut conn).expect("Error deleting recipe");
    Ok(HttpResponse::Ok().finish())
}
