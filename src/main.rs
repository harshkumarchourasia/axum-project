#![allow(unused)]

use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;

mod errors;
mod model;
mod web;
use errors::Error;
use model::Ticket;
use web::routes_login::login_handler;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/bye/:name", get(handler_bye))
        .fallback(fallback)
}

#[tokio::main]
async fn main() {
    let router: Router<()> = Router::new()
        .merge(routes_hello())
        .merge(login_handler())
        .layer(axum::middleware::map_response(main_response_mapper))
        .layer(tower_cookies::CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.unwrap_or("World".to_string());
    let response = format!("Hello {name}");
    println!("{}", response);
    Html(response)
}

async fn handler_bye(Path(name): Path<String>) -> impl IntoResponse {
    let response = format!("Bye {name}");
    println!("{}", response);
    Html(response)
}

async fn main_response_mapper(res: axum::response::Response) -> axum::response::Response {
    println!("main response mapper");
    println!("");
    res
}
