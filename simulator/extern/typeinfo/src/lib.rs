pub trait TypeInfo {
    fn type_name() -> String;
    fn type_of(&self) -> String;
}
