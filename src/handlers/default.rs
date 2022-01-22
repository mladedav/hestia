use actix_web::{Result, HttpResponse, HttpRequest};
use actix_web::http::{StatusCode, header};
use actix_files as fs;

// pub async fn p404() -> Result<fs::NamedFile> {
pub async fn p404(req: HttpRequest) -> Result<HttpResponse> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND).into_response(&req).unwrap())
}
pub async fn redirect() -> HttpResponse {
    HttpResponse::Found()
        .header(header::LOCATION, "/recipes")
        .finish()
}