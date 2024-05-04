
	use axum::{
		extract::Request,
		body::Body,
		response::Response,
	};
	use futures_util::future::BoxFuture;
	use tower::{Layer, Service};
	use std::task::{Context, Poll};


	// middleware layer
	#[derive(Clone)]
	pub struct LayerLog;

	impl<S> Layer<S> for LayerLog {
		type Service = MiddlewareLog<S>;

		fn layer(&self, inner: S) -> Self::Service {
			MiddlewareLog { inner }
		}
	}


	// Define the middleware service
	#[derive(Clone)]
	pub struct MiddlewareLog<S> {
		inner: S,
	}

	// Implement the Service trait for the middleware
	impl<S> Service<Request<Body>> for MiddlewareLog<S>
	where
		S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
		S::Future: Send + 'static,
	{
		type Response = S::Response;
		type Error = S::Error;
		type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

		// Forward the readiness check to the inner service
		fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
			self.inner.poll_ready(cx)
		}

		// Handle the incoming request
		fn call(&mut self, request: Request<Body>) -> Self::Future {
			// Log the request method and URI
			println!("Handling request: {} {}", request.method(), request.uri());

			// Clone the inner service so it can be moved into the async block
			let mut inner = self.inner.clone();

			// Call the inner service with the request
			let future = inner.call(request);

			// Return a future that will wait for the inner service's response
			Box::pin(async move {
				let response = future.await;
				// Optionally, log more information here
				response
			})
		}
	}