use crate::Error;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new(self) -> ModelController {
        Self {
            ticket_store: Arc::default(),
        }
    }
}

impl ModelController {
    pub fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket, Error> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub fn list_tickets(&self) -> Result<Vec<Ticket>, Error> {
        let store = self.ticket_store.lock().unwrap();
        let ticket_filter = store.iter().filter_map(|t| t.clone()).collect();
        Ok(ticket_filter)
    }

    pub fn delete_ticket(&self, id: u64) -> Result<Ticket, Error> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFileIdNotFound(id))
    }
}
