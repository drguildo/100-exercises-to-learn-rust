use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    let mut ticket_store = store::TicketStore::new();

    loop {
        match receiver.recv() {
            Ok(command) => {
                match command {
                    Command::Insert(ticket_draft) => {
                        ticket_store.add_ticket(ticket_draft);
                    }
                }
            }
            Err(_) => {
                panic!("Something bad happened!")
            }
        }
    }
}
