use hestia::build;

#[rocket::launch]
async fn rocket() -> _ {
    build()
}
