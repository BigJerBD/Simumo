use crate::log_message_senders::{LogMessage, LOG_MESSAGE_SENDERS};
use std::sync::mpsc::Sender;

/// entry point to send logs records to an AsyncLogWriter.
///
/// # Fields
///
/// * log_input : Sender Channel that sends to the receiver in the AsyncLogWriter
///
/// TODO :: make the error handling configurable?
pub struct LogSender {
    log_input: Sender<LogMessage>,
}
impl LogSender {
    /// Create and fetch the proper sender to send future records
    ///
    pub fn new(logger_name: String) -> Self {
        match LOG_MESSAGE_SENDERS.get_sender(logger_name) {
            Ok(sender) => Self { log_input: sender },
            Err(_) => panic!("The current AsyncLogWriter shut down unexpectedly"),
        }
    }

    ///Send a serializable record to the receiver specific AsyncLogWriter
    ///
    pub fn log(&self, record: Box<LogMessage>) {
        match self.log_input.send(record) {
            Ok(()) => (),
            Err(_) => panic!("The current LogWriter shut down unexpectedly"),
        };
    }
}
