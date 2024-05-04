use axum::response::IntoResponse;


pub async fn home() -> impl IntoResponse {
	"Home!"
}
