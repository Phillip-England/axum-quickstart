use std::sync::Arc;

use axum::{response::{IntoResponse, Redirect}, Extension, Form};
use serde::Deserialize;

use crate::state::state::AppState;



#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login(
    Extension(_shared_state): Extension<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {

    fn fail(login_err: &str, email: &str) -> String {
        format!("/?login_err={}&email={}", login_err, email)
    }

    if form.email == "" || form.password == "" {
        return Redirect::to(&fail("invalid credentails", &form.email));
    }

    if form.email != dotenv!("ADMIN_EMAIL") && form.password != dotenv!("ADMIN_PASSWORD") {
        return Redirect::to(&fail("invalid credentails", &form.email));
    }
    
    return Redirect::to("/admin");
    
}