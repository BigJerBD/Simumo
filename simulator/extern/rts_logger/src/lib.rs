#[macro_use]
extern crate lazy_static;

use crate::logger_configuration::LoggerConfiguration;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread::JoinHandle,
};
pub mod data_writer;
pub mod logger_configuration;

/// this static collection is used to send the sender channels easily
/// so the users dont have to do complex manipulation
/// when creating a new AsyncLogSender
///
lazy_static! {
    static ref LOG_SENDERS: LogSenderProvider = LogSenderProvider::new();
}

///
type LogMessage = Box<erased_serde::Serialize + Send>;
///
type LogSenderMapping = HashMap<String, Sender<LogMessage>>;

///
///
///
pub struct LogSenderProvider(Mutex<LogSenderMapping>);
impl LogSenderProvider {
    fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    fn add_sender(&self, name: String, sender: Sender<LogMessage>) {
        self.0.lock().unwrap().insert(name, sender);
    }

    fn get_sender(&self, name: String) -> Result<Sender<LogMessage>, ()> {
        match self.0.lock().unwrap().get(&name) {
            Some(val) => Ok(val.clone()),
            None => Err(()),
        }
    }
}

///
///
///
pub struct LogWriterManager {
    loggers: HashMap<String, AsyncLogWriter>,
}
impl LogWriterManager {
    pub fn new() -> Self {
        Self {
            loggers: HashMap::new(),
        }
    }

    pub fn from_loggers<L>(log_configs: L) -> Result<Self, String>
    where
        L: Iterator<Item = LoggerConfiguration>,
    {
        let mut manager = Self::new();
        for config in log_configs {
            manager.add_logger(config)?
        }
        Ok(manager)
    }

    pub fn add_logger(&mut self, log_config: LoggerConfiguration) -> Result<(), String> {
        let name = log_config.name.clone();
        if let Some(_) = self.loggers.get(&name) {
            Err(format!(
                "The Logger name was already found in the manager. name={}",
                name
            ))
        } else {
            self.loggers
                .insert(name, Self::spawn_log_writer(log_config));
            Ok(())
        }
    }

    pub fn remove_logger(&mut self, name: String) -> Result<(), String> {
        match self.loggers.remove(&name) {
            Some(_) => Ok(()),
            None => Err(format!("The Logger to delete was not found. name={}", name)),
        }
    }

    ///spawn a log writer
    /// TODO :: handle the other config parameters more properly
    /// TODO :: give the reciever to the log writer
    fn spawn_log_writer(log_config: LoggerConfiguration) -> AsyncLogWriter {
        let (sender, receiver) = channel::<LogMessage>();

        LOG_SENDERS.add_sender(log_config.name, sender);
        AsyncLogWriter::new()
    }
}

/// entry point to send logs records to an AsyncLogWriter.
///
/// # Fields
///
/// * log_input : Sender Channel that sends to the receiver in the AsyncLogWriter
///
/// TODO :: make the error handling configurable?
pub struct AsyncLogSender {
    log_input: Sender<LogMessage>,
}
impl AsyncLogSender {
    /// Create and fetch the proper sender to send future records
    ///
    pub fn new(logger_name: String) -> Self {
        match LOG_SENDERS.get_sender(logger_name) {
            Ok(sender) => Self { log_input: sender },
            Err(_) => panic!("The current AsyncLogWriter shut down unexpectedly"),
        }
    }

    ///Send a serializable record to the receiver specific AsyncLogWriter
    ///
    pub fn log(&self, record: Box<LogMessage>) {
        match self.log_input.send(record) {
            Ok(()) => (),
            Err(_) => panic!("The current AsyncLogWriter shut down unexpectedly"),
        };
    }
}

/// Log writer that
///
/// # Fields
///
///
struct AsyncLogWriter {
    //file_handle: RefCell<LoggerType>,
    thread_handle: JoinHandle<()>,
}
impl AsyncLogWriter {
    fn new() -> Self {
        unimplemented!()
        //let sender = thread::spawn(
        //    move || {
        //    //
        //});
    }
}
impl Drop for AsyncLogWriter {
    fn drop(&mut self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    //TODO :: add unit tests
}
