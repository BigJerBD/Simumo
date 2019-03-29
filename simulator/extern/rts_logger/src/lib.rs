use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::fs::File;



/// The logger manager that allow to
///
struct AsyncLogManager;
impl AsyncLogManager{

    fn with_loggers() -> Self {

    }

    fn add_logger(&mut self){

    }


}

impl Drop for AsyncLogManager{
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
struct AsyncLogSender{}
impl AsyncLogSender{}


/// Log writer that
///
struct AsyncLogWriter {
    file_handle : File,
    thread_handle : JoinHandle<()>,
}
impl AsyncLogWriter {

    fn new() -> Self {
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


// -- add function to start  a logger manager
// -- lazy_static?
// -- plusieurs thread dans le logger manager?
// -- drain?
// -- handling different files?
// -- mutex necessaire
// -- reference mapping only in the lazy_static?


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
