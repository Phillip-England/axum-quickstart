use std::collections::HashMap;
use std::sync::Arc;

use askama::Template;
use axum::extract::Query;
use axum::{response::{Html, IntoResponse}, Extension};

use crate::templates::templates::{AdminPanel, Login};
use crate::state::state::AppState;


pub async fn home(
    Extension(_shared_state): Extension<Arc<AppState>>, 
    Query(params): Query<HashMap<String, String>>
) -> impl IntoResponse {
    let mut login_err = "";
    if let Some(value) = params.get("login_err") {
        login_err = value;
    }
    let template = Login {
        title: "Home",
        login_err: login_err,
        show_bars: false,
    };
    Html(template.render().unwrap())
}

pub async fn admin_panel(
    Extension(_shared_state): Extension<Arc<AppState>>, 
) -> impl IntoResponse {
    let template = AdminPanel {
        title: "Home",
        show_bars: true,
    };
    Html(template.render().unwrap())
}

pub async fn not_found() -> impl IntoResponse {
    "Not Found!"
}
