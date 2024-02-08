use super::handlers;
use crate::api::handlers::jwt::auth;
use crate::state::ApplicationState;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::sync::Arc;
use crate::api::handlers::dogs;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route( 
            "/hello", 
            get(handlers::hello::hello).with_state(state.clone()),
        )
        // change get(handlers::hello::hello).with_state(state), to clone, login need to use state
        //.route("/login", post(handlers::login::login).with_state(state))
        .route(
            "/login", 
            post(handlers::login::login).with_state(state.clone()),
        )
        .route(
            "/dogs",
            post(handlers::dogs::create)
            .with_state(state.clone())
            .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/dogs",
            get(handlers::dogs::list).with_state(state.clone())
        )
        .route(
            "/dogs/:id",
            get(handlers::dogs::get).with_state(state)
        )


}
