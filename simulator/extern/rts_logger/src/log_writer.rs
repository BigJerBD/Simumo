use crate::{data_writer::DataWrite};
use std::{thread, sync::mpsc::channel::Receiver};

/// Log writer that listens for new data to be written to a specific file
///
///
pub struct LogWriter {
    file_handle: RefCell<DataWrite>,
    queue: Sender<LogMessage>,
    worker_thread: thread::JoinHandle<()>,
}

impl LogWriter {
    pub fn new(log_config: LoggerConfiguration,
               queue: Sender<LogMessage>,
               receiver: Receiver<LogMessage>) -> Self {
        Self {
            file_handle: log_config.log_type,
            queue,
            worker_thread: thread::Builder::new().name(log_config.name).spawn(move || loop {
                let msg = receiver.recv();
                match msg {
                    Ok(LogMessage::LogMsg(msg)) => file_handle.write(msg),
                    Ok(LogMessage::Quit) => break,
                    Err(e) => panic!("The current LogWriter closed unexpectedly without sending a Quit first"),
                }
            }),
        }
    }
}

impl Drop for LogWriter {
    fn drop(&mut self) {
        let thread_stop_msg;
        self.queue.send(LogMessage::Quit).expect("The current LogWriter's queue was closed before joining thread");
        self.worker_thread.join().expect("Failed to join the worker thread when dropping current LogWriter");
    }
}
