use actix_web::{web, guard};
use actix_web_httpauth::middleware::HttpAuthentication;
use handlebars::{Handlebars, handlebars_helper};

use crate::{handlers::{recipes, users, default}, authorization};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(actix_files::Files::new("/pictures", "./pictures").show_files_listing())
            .service(web::resource("").route(web::get().to(default::redirect)))
            .service(
                web::scope("/users")
                    .wrap(HttpAuthentication::bearer(authorization::validator))
                    .service(
                        web::resource("")
                            .route(web::get().to(users::list_users))
                    )
                    .service(
                        web::resource("/{user_id}")
                            .route(web::get().to(users::get_user))
                    )
            )
            .service(
                web::scope("/recipes")
                    .route("", web::get().to(recipes::list_recipes))
                    .route("/add", web::get().to(recipes::add_form))
                    .route("",
                        web::post()
                            .guard(guard::Header("content-type", "application/x-www-form-urlencoded"))
                            .to(recipes::add_recipe_url_encoded),
                    )
                    .route("",
                        web::post()
                            .guard(guard::fn_guard(|req| {
                                if let Some(content_type) = req.headers().get("content-type") {
                                    return content_type.to_str().unwrap_or("").starts_with("multipart/form-data;"); 
                                }
                                return false;
                            }))
                            .to(recipes::add_recipe_multipart)
                    )
                    .route("",
                        web::post()
                            .guard(guard::Header("content-type", "application/json"))
                            .to(recipes::add_recipe_json),
                    )
                    .route("/{recipe_id}", web::get().to(recipes::get_recipe))
                    .route("/{recipe_id}", web::delete().to(recipes::remove_recipe))
            )
    );
}

pub fn handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    handlebars_helper!(range: |x: u64| (0..x).collect::<Vec<u64>>());
    handlebars.register_helper("range", Box::new(range));
    handlebars_helper!(sub: |x: u64, y: u64| x - y);
    handlebars.register_helper("sub", Box::new(sub));
    handlebars_helper!(excerpt: |text: String, length: usize| {
        if length < text.len() {
            format!("{}...", &text[..length])
        } else {
            text
        }
    });
    handlebars.register_helper("excerpt", Box::new(excerpt));
    handlebars.register_templates_directory(".html.hbs", "templates").unwrap();
    handlebars
}
