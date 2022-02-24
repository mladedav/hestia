use hestia::start;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start()?.await
}
