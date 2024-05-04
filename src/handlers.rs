use axum::response::IntoResponse;


pub async fn home() -> impl IntoResponse {
	"Home!"
}

pub async fn not_found() -> impl IntoResponse {
    "Not Found!"
}
