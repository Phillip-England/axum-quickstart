
mod handlers;
mod middleware;
mod state;
mod templates;

use crate::handlers::view_handlers::{
    home,
    not_found,
    admin_panel
};
use crate::handlers::form_handlers::login;
use crate::middleware::middleware::TimingMiddleware;
use crate::state::state::new_app_state;

use std::sync::Arc;
use axum::Extension;
use axum::{
	routing::get,
    routing::post,
	Router,
};
use tower_http::services::ServeDir;

#[macro_use]
extern crate dotenv_codegen;
#[tokio::main]
async fn main() {

    // loading in env file
    dotenv::dotenv().ok();

	// getting hostname
	let host = "0.0.0.0";
	let addr = format!("{}:{}", host, "8080");

    // setting up shared state
    let shared_state = Arc::new(new_app_state());

	// building router
	let app = Router::new()
		.route("/", get(home))
        .route("/", post(login))
        .route("/admin", get(admin_panel))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TimingMiddleware)
        .layer(Extension(shared_state))
        .fallback(get(not_found));

	// binding and serving
	println!("development server running on {}", addr);
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app).await.unwrap()


}
