// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId,TrySendError<Command>> {
        let (sender_insert, receiver_insert) = std::sync::mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: sender_insert,
            })?;
        Ok(receiver_insert.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<Command>> {
        let (sender_get, receiver_get) = std::sync::mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: sender_get,
            })?;
        Ok(receiver_get.recv().unwrap())
    }
    pub fn new(sender: SyncSender<Command>) -> Self {
        Self { sender }
    }
}
pub fn launch(capacity: usize)  -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));

    TicketStoreClient { sender }
}

pub enum Command {
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
