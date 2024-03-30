use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Error;
use axum::extract::Path;
use axum::routing::{delete, post};
use axum::{extract::State, Json, Router};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_tickets))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>, Error> {
    let ticket: Ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>, Error> {
    let tickets: Vec<Ticket> = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, Error> {
    let ticket: Ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
