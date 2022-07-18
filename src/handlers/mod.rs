use rocket::{get, response::Redirect, uri};

pub(super) mod auth;
pub(super) mod recipes;

#[get("/")]
pub fn redirect_recipes() -> Redirect {
    Redirect::to(uri!("/recipes", super::recipes::list))
}
