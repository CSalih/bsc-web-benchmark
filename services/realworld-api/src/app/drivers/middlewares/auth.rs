use crate::app::drivers::middlewares::state::AppState;
use crate::app::features::user::entities::User;
use crate::constants;
use crate::error::AppError;
use crate::utils::token;
use actix_web::HttpMessage;
use actix_web::{
  body::EitherBody,
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  http::Method,
  web::Data,
  Error, HttpRequest, HttpResponse,
};
use futures::future::{ok, Ready};
use futures::Future;
use serde_json::json;
use std::pin::Pin;
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let user_id = get_user_id_from_header(&req);

        if user_id.is_ok() && !set_auth_user(&mut req) {
            return Box::pin(async move {
                let (req, _res) = req.into_parts();
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                let srv = ServiceResponse::new(req, res);
                Ok(srv)
            });
        };

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?.map_into_left_body();
            Ok(res)
        })
    }
}

const TOKEN_IDENTIFIER: &str = "Token";

fn set_auth_user(req: &mut ServiceRequest) -> bool {
    match fetch_user(req) {
        Ok(user) => {
            req.extensions_mut().insert(user);
            true
        }
        Err(err_msg) => {
            info!("Cannot fetch user {}", err_msg);
            false
        }
    }
}

fn fetch_user(req: &ServiceRequest) -> Result<User, &str> {
    let user_id = get_user_id_from_header(req)?;
    req.app_data::<Data<AppState>>()
        .ok_or("Cannot get app state.")
        .and_then(|state| state.di_container.user_usecase.find_auth_user(user_id))
}

fn get_user_id_from_header(req: &ServiceRequest) -> Result<Uuid, &str> {
    req.headers()
        .get(constants::AUTHORIZATION)
        .ok_or("Cannot find authrization key-value in req header")
        .and_then(|auth_header| auth_header.to_str().map_err(|_err| "Cannot stringify"))
        .and_then(|auth_str| {
            if auth_str.starts_with(TOKEN_IDENTIFIER) {
                Ok(auth_str)
            } else {
                Err("Invalid token convention")
            }
        })
        .map(|auth_str| auth_str[6..auth_str.len()].trim())
        .and_then(|token| token::decode(token).map_err(|_err| "Cannot decode token."))
        .map(|token| token.claims.user_id)
}

pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
    req.extensions()
        .get::<User>()
        .map(|user| user.to_owned())
        .ok_or_else(|| {
            AppError::Unauthorized(json!({"error": "Unauthrized user. Need auth token on header."}))
        })
}
