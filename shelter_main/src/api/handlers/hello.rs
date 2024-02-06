use crate::state::ApplicationState;

use axum::http::StatusCode;
use axum::extract::State;
use std::sync::Arc;

pub async fn hello(State(state): State<Arc<ApplicationState>> ) -> Result<String, StatusCode> {
    Ok(format!(
            "\nhello world!using config {}\n\n",
            state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("-".to_string())
    ))
}
