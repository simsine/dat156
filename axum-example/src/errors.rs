use axum::http::StatusCode;

use axum::response::{IntoResponse, Response};

use hypertext::prelude::*;

pub struct AppError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
	E: Into<anyhow::Error>,
{
	fn from(err: E) -> Self {
		Self(err.into())
	}
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		(StatusCode::INTERNAL_SERVER_ERROR, maud!{
			h1 {(StatusCode::NOT_FOUND.as_u16())}
			p {(self.0.to_string())}
		}.render()).into_response()
	}
}

pub async fn error_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, maud!(
        h1 {"404"}
        p {"Page not found"}
    ).render().into_response())
}
