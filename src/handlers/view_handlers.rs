use std::collections::HashMap;
use std::sync::Arc;
use time::Duration;

use askama::Template;

use axum::extract::Query;
use axum::http::{Uri, header::{HeaderMap, SET_COOKIE}};
use axum::response::{Html, IntoResponse, Redirect};
use axum::Extension;
use cookie::{Cookie, CookieJar};


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
    uri: Uri,
) -> impl IntoResponse {
    // Get the request path without query parameters
    let path = uri.path();

    let template = AdminPanel {
        title: "Home",
        show_bars: true,
        path,
    };

    Html(template.render().unwrap())
}

pub async fn logout(
    Extension(_shared_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let cookie = Cookie::build("session", "sasdas")
        .path("/")
        .max_age(Duration::seconds(0)) // immediately expire the cookie
        .finish();

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());
    (headers, Redirect::to("/"))
}

pub async fn not_found() -> impl IntoResponse {
    "Not Found!"
}
