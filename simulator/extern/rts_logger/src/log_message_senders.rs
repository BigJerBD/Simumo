use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Mutex},
};

/// Object send through the channels for logging
///
pub type LogMessage = Box<erased_serde::Serialize + Send>;

/// used for convenience
type SenderMapping<T> = HashMap<String, Sender<T>>;

/// Sender Mapping that can be used in multiple location in the program.
///
/// this can be done because it is wrapped in a mutex, thus guaranteeing synchronisation
///
pub struct SharedSenderMapping<T>(Mutex<SenderMapping<T>>);
impl<T> SharedSenderMapping<T> {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    pub fn add_sender(&self, name: String, sender: Sender<T>) {
        self.0.lock().unwrap().insert(name, sender);
    }

    pub fn get_sender(&self, name: String) -> Result<Sender<T>, ()> {
        match self.0.lock().unwrap().get(&name) {
            Some(val) => Ok(val.clone()),
            None => Err(()),
        }
    }
}

/// this static collection is used to send the sender channels easily
/// so the users dont have to do complex manipulation
/// when creating a new AsyncLogSender
///
lazy_static! {
    pub static ref LOG_MESSAGE_SENDERS: SharedSenderMapping<LogMessage> =
        SharedSenderMapping::new();
}
