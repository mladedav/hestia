use std::convert::Infallible;

use actix_web::{dev::Server};
use async_trait::async_trait;
use cucumber::{given, when, then, World, WorldInit};
use hestia::start;
use reqwest::{Response, redirect::Policy};
use scraper::{Html, Selector};

// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct RecipeWorld {
    server: Option<Server>,
    response: Option<Response>,
    page: Option<Html>,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for RecipeWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            server: None,
            response: None,
            page: None,
        })
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a running server")]
async fn start_server(world: &mut RecipeWorld) {
    world.server = Some(start().expect("Unable to start the server."));
}

#[when("a recipe is added")]
async fn add_recipe(_world: &mut RecipeWorld) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("title", "Cucumber")
        .text("ingredients", "Cucumber and salt")
        .text("tips", "Do not eat")
        .text("preparation_minutes", "5")
        .text("stars", "0")
        .text("class", "other");

    client.post("http://localhost:8080/recipes").multipart(form).send().await.expect("Unable to add recipe");
    Ok(())
}

#[when("recipes are loaded")]
async fn load_recipes(world: &mut RecipeWorld) -> reqwest::Result<()> {
    let response = reqwest::get("http://localhost:8080/recipes").await?;
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    world.page = Some(document);
    Ok(())
}

#[then(expr = "{int} recipes are shown")]
fn check_recipes(world: &mut RecipeWorld, expected: usize) {
    let selector = Selector::parse(".listfeaturedtag .card").unwrap();
    let count = world.page.as_ref().expect("Page was not loaded").select(&selector).count();
    assert_eq!(count, expected, "Expected {} recipes, instead found {}", expected, count)
}

#[when("root page is loaded")]
async fn load_root(world: &mut RecipeWorld) -> reqwest::Result<()> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?;
    let response = client.get("http://localhost:8080").send().await?;
    world.response = Some(response);
    Ok(())
}

#[then("I am redirected to recipe list")]
fn check_redirect(world: &mut RecipeWorld) {
    let response = world.response.as_ref().expect("Response is missing");
    assert_eq!(response.status().is_redirection(), true, "Expected redirect status code, instead found {}", response.status());
    let location = response.headers().get_all("Location");
    assert_eq!(location.iter().count(), 1);
    let location = location.iter().next().unwrap();
    assert_eq!(location, "/recipes");
}


#[actix_web::main]
async fn main2() {
    RecipeWorld::run("tests/cucumber").await;
}

#[tokio::main]
async fn main() {
    main2();
}
