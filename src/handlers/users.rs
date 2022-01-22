use actix_web::{web, Error, HttpResponse};

pub async fn get_user(id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(id.into_inner()))
}

pub async fn list_users() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
