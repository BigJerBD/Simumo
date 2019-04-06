use std::thread::JoinHandle;

/// Log writer that
///
/// # Fields
///
///
pub struct LogWriter {
    //file_handle: RefCell<LoggerType>,
    thread_handle: JoinHandle<()>,
}
impl LogWriter {
    pub fn new() -> Self {
        unimplemented!()
        //let sender = thread::spawn(
        //    move || {
        //    //
        //});
    }
}
impl Drop for LogWriter {
    fn drop(&mut self) {
        unimplemented!()
    }
}
