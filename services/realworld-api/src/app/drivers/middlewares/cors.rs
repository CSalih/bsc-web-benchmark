use actix_cors::Cors;

pub fn cors() -> Cors {
    Cors::default()
        .allow_any_method()
        .allow_any_origin()
        .allow_any_header()
        .allowed_header(actix_web::http::header::AUTHORIZATION)
        .allowed_header(actix_web::http::header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600)
}
