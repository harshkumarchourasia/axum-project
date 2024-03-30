use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
#[derive(Debug)]
pub enum Error {
    LoginFail,
    TicketDeleteFileIdNotFound(u64),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::LoginFail => "Login Failed, Please try again".to_string(),
            Error::TicketDeleteFileIdNotFound(id) => format!("Ticket id not present {}", id),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
