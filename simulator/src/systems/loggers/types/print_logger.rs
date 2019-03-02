
#[derive(Deserialize)]
pub struct PrintLogger;
impl LoggerType for PrintLogger {
    fn open(_: &str) -> Self {
        Self
    }

    fn write<S: Serialize>(&mut self, record: S) {
        let mut json = serde_json::to_string(&record).unwrap();
        print!(json)
    }
}
