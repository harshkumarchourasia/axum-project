use crate::web::AUTH_TOKEN;
use crate::Error;
use axum::routing::{post, Router};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing;
pub fn login_handler() -> Router<()> {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>, Error> {
    if payload.username == "admin" && payload.password == "admin" {
        let body = Json(json!(
            {
                "result": {
                "success": true
            }
            }
        ));
        cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));
        tracing::info!("Login Successfully");
        Ok(body)
    } else {
        tracing::error!("Login Failed");
        Err(Error::LoginFail)
    }
}

#[derive(Deserialize, Debug)]
struct LoginPayload {
    username: String,
    password: String,
}
