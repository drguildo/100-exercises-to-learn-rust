use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    capacity: usize,
    sender: SyncSender<Command>,
}

#[derive(Debug)]
pub struct TicketStoreError;

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TicketStoreError> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(self.capacity);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: sender,
            })
            .map_err(|_| TicketStoreError)?;
        let id = receiver.recv().unwrap();
        Ok(id)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TicketStoreError> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(self.capacity);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: sender,
            })
            .map_err(|_| TicketStoreError)?;
        let ticket = receiver.recv().unwrap();
        Ok(ticket)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { capacity, sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
