// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{self, mpsc};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
    capacity: usize,
}

impl TicketStoreClient {
    fn new(sender: SyncSender<Command>, capacity: usize) -> Self {
        TicketStoreClient { sender, capacity }
    }

    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        let (sender, receiver) = mpsc::sync_channel(self.capacity);

        let send_resp = self.sender.try_send(Command::Insert {
            draft,
            response_channel: sender,
        });

        match send_resp {
            Ok(_) => Ok(receiver.recv().unwrap()),
            Err(_) => Err("Channel full try again".into()),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        let (sender, receiver) = mpsc::sync_channel(self.capacity);

        let send_resp = self.sender.try_send(Command::Get {
            id,
            response_channel: sender,
        });

        match send_resp {
            Ok(_) => Ok(receiver.recv().unwrap()),
            Err(_) => Err("Channel full try again".into()),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender, capacity)
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
                response_channel.try_send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.try_send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
