use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};


pub enum AppError {
    Internal,
}

impl From<tera::Error> for AppError {
    fn from(error: tera::Error) -> Self {
        tracing::error!("Tera error: {}", error);
        AppError::Internal
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Internal => {
                tracing::error!("Internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}