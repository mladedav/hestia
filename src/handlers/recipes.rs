use std::collections::BTreeMap;

use diesel::prelude::*;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{get, post, uri};
use rocket_dyn_templates::{context, Template};

use crate::db::models::{RecipeDb, RecipeForm};
use crate::db::schema::recipes;
use crate::handlers::auth::User;
use crate::CONFIG;

pub fn establish_connection() -> SqliteConnection {
    let db_url = &CONFIG.get().unwrap().db_file;
    SqliteConnection::establish(db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

#[get("/<id>")]
pub async fn get(id: i32, user: Option<User>) -> Option<Template> {
    let mut conn = establish_connection();
    let recipe = match recipes::table.find(id).first::<RecipeDb>(&mut conn) {
        Ok(recipe) => recipe,
        Err(diesel::result::Error::NotFound) => return None,
        Err(e) => panic!("Unexpected error: {}", e),
    };

    Some(Template::render(
        "recipes/detail",
        context! {
            recipe: &recipe,
            user: &user,
        },
    ))
}

#[get("/")]
pub async fn list(user: Option<User>) -> Template {
    list_paged(None, None, user).await
}

#[get("/?<page>&<count>")]
pub async fn list_paged(page: Option<i64>, count: Option<i64>, user: Option<User>) -> Template {
    let page_count = match count {
        Some(val) if val > 0 && val <= 96 => val,
        _ => 48,
    };
    let page = page.unwrap_or(0);
    let mut conn = establish_connection();
    let count: i64 = recipes::table
        .count()
        .get_result(&mut conn)
        .expect("Error getting count of recipes");

    let recipes = recipes::table
        .offset(page * page_count)
        .limit(page_count)
        .load::<RecipeDb>(&mut conn)
        .expect("Error loading recipes");

    Template::render(
        "recipes/list",
        context! {
            recipes: &recipes,
            count: count,
            user: &user,
        },
    )
}

#[get("/add")]
pub async fn add(user: User) -> Template {
    Template::render(
        "recipes/add",
        context! {
            user: &user
        },
    )
}

#[get("/add", rank = 6)]
pub async fn add_redirect() -> Redirect {
    Redirect::to(uri!("/recipes", list))
}

#[get("/edit/<id>")]
pub async fn edit(id: i32, user: User) -> Option<Template> {
    // TODO check that the user can change this
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

    Some(Template::render(
        "recipes/edit",
        context! {
            recipe: &data,
            user: &user,
        },
    ))
}

#[post("/<id>", data = "<recipe>")]
pub async fn update(id: i32, mut recipe: Form<RecipeForm<'_>>, _user: User) -> Redirect {
    // TODO check that the user is the same
    let recipe = recipe.as_db(id).await;

    let mut conn = establish_connection();
    diesel::replace_into(recipes::table)
        .values(&recipe)
        .get_result::<RecipeDb>(&mut conn)
        .expect("Error saving new recipe");

    Redirect::to(uri!("/recipes", list))
}

#[post("/", data = "<recipe>")]
pub async fn insert(mut recipe: Form<RecipeForm<'_>>, _user: User) -> Redirect {
    // TODO add user to the recipe
    let recipe = recipe.as_new_db().await;

    let mut conn = establish_connection();
    let recipe: RecipeDb = diesel::insert_into(recipes::table)
        .values(&recipe)
        .get_result(&mut conn)
        .expect("Error saving new recipe");

    Redirect::to(uri!("/recipes", get(recipe.id)))
}
