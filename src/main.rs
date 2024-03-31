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
use crate::model::ModelController;
use errors::Error;
use model::Ticket;
use web::routes_login::login_handler;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing;

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
async fn main() -> Result<(), Error> {

    tracing_subscriber::fmt()
        .with_target(false)
        .init();


    let mc = ModelController::new().await;
    let router: Router<()> = Router::new()
        .merge(routes_hello())
        .merge(login_handler())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http()))
        .layer(axum::middleware::map_response(main_response_mapper))
        .layer(tower_cookies::CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.unwrap_or("World".to_string());
    let response = format!("Hello {name}");
    tracing::info!("Say hello to user");
    Html(response)
}

async fn handler_bye(Path(name): Path<String>) -> impl IntoResponse {
    let response = format!("Bye {name}");
    tracing::info!("Say bye to user");
    Html(response)
}

async fn main_response_mapper(res: axum::response::Response) -> axum::response::Response {
    tracing::info!("Main Response Handler");
    res
}
