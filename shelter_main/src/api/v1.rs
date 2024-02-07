use crate::state::ApplicationState;

use super::handlers;
use axum::routing::{get,post};
use axum::Router;
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route( "/hello", get(handlers::hello::hello).with_state(state.clone()))
        // change get(handlers::hello::hello).with_state(state), to clone, login need to use state
        .route("/login", post(handlers::login::login).with_state(state))

}
