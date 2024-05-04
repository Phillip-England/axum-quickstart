
mod handlers;
mod middleware;

use std::{env, process::exit};
use axum::{
	routing::get,
	Router,
};


#[tokio::main]
async fn main() {

	// selecting port
	let args: Vec<String> = env::args().collect();
	let mut port: &str = "8080";
	if args.len() > 2 {
		let arg = &args[2];
		let arg_port: Result<i32, _> = arg.parse();
		match arg_port {
			Ok(_val) => {
				port = arg;
			},
			Err(_err) => {
				println!("Port is not a number...");
				exit(1);
			}
		};
	}

	// getting hostname
	let host = "0.0.0.0";
	let addr = format!("{}:{}", host, port);

	// building router
	let app = Router::new()
		.route("/", get(handlers::home))
		.layer(middleware::LayerLog);

	// binding and serving
	println!("development server running on {}", addr);
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app).await.unwrap()


}
