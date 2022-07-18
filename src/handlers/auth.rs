use rocket::http::{Cookie, SameSite};
use rocket::request::{FromRequest, Request};
use rocket::time::{Duration, OffsetDateTime};
use rocket::{get, http::CookieJar, outcome::IntoOutcome, request, response::Redirect, uri};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Tokens {
    id_token: String,
    scope: String,
    // refresh_token: String,
    access_token: String,
    token_type: String,
    expires_in: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    sub: String,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    email: String,
    email_verified: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("user")
            .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
            .or_forward(())
    }
}

#[get("/callback?<code>")]
pub async fn callback(jar: &CookieJar<'_>, code: &str) -> Redirect {
    let client_id = "CRviuZCro0wWnbyqZR6paB7Pzr1eKXLc";
    let client_secret = "RHqi_n8pSroVmgu8Mc0JBw-xvnUq2z1sZCbmSJ4n3FulR8KLxpnGPjjPrhph0wE7";
    let this_host = "http://localhost:8080";
    let client = reqwest::Client::new();
    let token: Tokens = client.post("https://dev-q1c7r97i.eu.auth0.com/oauth/token")
        .header("content-type", "application/x-www-form-urlencoded")
        .header("accept", "application/json")
        .body(
            format!(
                "grant_type=authorization_code&client_id={}&client_secret={}&code={}&redirect_uri={}/callback",
                client_id,
                client_secret,
                code,
                this_host,
            )
        )
        .send()
        .await
        .unwrap()
        .json::<Tokens>()
        .await
        .unwrap();

    let user = client
        .get("https://dev-q1c7r97i.eu.auth0.com/userinfo")
        .header("accept", "application/json")
        .header("authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    let mut cookie = Cookie::new("user", serde_json::to_string(&user).unwrap());
    cookie.set_expires(OffsetDateTime::now_utc() + Duration::HOUR);
    // This is here because the redirects may mix http and https in some cases
    cookie.set_same_site(Some(SameSite::Lax));
    jar.add_private(cookie);

    Redirect::to(uri!("/recipes", super::recipes::list))
}

#[get("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("user"));
    Redirect::to(uri!("/recipes", super::recipes::list))
}
