use std::{cell::RefCell, rc::Rc};

use actix_web::{
    body::BoxBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorInternalServerError,
    http::header,
    HttpResponse,
};
use futures::future::LocalBoxFuture;

use crate::db::validate_session;

#[derive(Clone)]
pub enum AuthRedirectStrategy {
    /// Require authentication for the page.
    RequireAuth,
    /// Don't allow authenticated users to see this page. Redirect them to the path in the string.
    DisallowAuth(String),
}

impl<S> Transform<S, ServiceRequest> for AuthRedirectStrategy
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>
        + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthRedirectMiddleware<S>;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(AuthRedirectMiddleware {
            service: Rc::new(RefCell::new(service)),
            strategy: self.clone(),
        })
    }
}

pub struct AuthRedirectMiddleware<S> {
    service: Rc<RefCell<S>>,
    strategy: AuthRedirectStrategy,
}

impl<S> Service<ServiceRequest> for AuthRedirectMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>
        + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let strategy = self.strategy.clone();

        Box::pin(async move {
            // Check if user is logged in or not
            let is_logged_in = match get_session_token_service_request(&req) {
                Some(t) => match validate_session(t.as_str()).await {
                    Ok(v) => v,
                    Err(e) => {
                        //log::error!("Error validating session: {:?}", e);
                        false
                    }
                },
                None => false,
            };

            match strategy {
                AuthRedirectStrategy::RequireAuth => match is_logged_in {
                    true => service.call(req).await.map(|res| res.map_into_boxed_body()),
                    false => Ok(req.into_response(
                        HttpResponse::Found()
                            .append_header((header::LOCATION, "/login"))
                            .finish(),
                    )),
                },
                AuthRedirectStrategy::DisallowAuth(ref path) => match is_logged_in {
                    true => Ok(req.into_response(
                        HttpResponse::Found()
                            .append_header((header::LOCATION, path.as_str()))
                            .finish(),
                    )),
                    false => service.call(req).await.map(|res| res.map_into_boxed_body()),
                },
            }
        })
    }
}
