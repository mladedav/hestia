use hestia::build;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    build().await?.launch().await
}
