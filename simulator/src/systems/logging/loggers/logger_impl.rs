use serde::Serialize;

/// used to create
/// specific Logger Implementation
pub trait LoggerImpl {
    fn open(filename: &str) -> Self;
    fn write<S: Serialize>(&mut self, record: S);
}
