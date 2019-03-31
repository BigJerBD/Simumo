use std::fs::File;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use serde::Serialize;

pub mod logger_type;

use crate::logger_type::LoggerType;

/// The logger manager that allow to
///
struct AsyncLogManager;
impl AsyncLogManager {
    fn with_loggers() -> Self {
        unimplemented!()
    }

    fn add_logger(&mut self) {
        unimplemented!()
    }
}

impl Drop for AsyncLogManager {
    fn drop(&mut self) {
        unimplemented!()
    }
}

/// Async logger that uses two queues.
///
/// # fields
///
///
/// the io and record_queue is swapped when ...
///
struct AsyncLogSender {}
impl AsyncLogSender {
    ///Send a serializable record to a queuje
    ///
    fn log<S: Serialize>(&mut self, record: S) {}
}

/// Log writer that
///
struct AsyncLogWriter<T: LoggerType> {
    file_handle: T,
    thread_handle: JoinHandle<()>,
}
impl<T: LoggerType> AsyncLogWriter<T> {
    fn new() -> Self {
        unimplemented!()
        //let sender = thread::spawn(
        //    move || {
        //    //
        //});
    }
}
impl<T: LoggerType> Drop for AsyncLogWriter<T> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
