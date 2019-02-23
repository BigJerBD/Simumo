pub trait TypeInfo {
    const typename : &'static str;
    fn type_of(&self) -> &'static str{
        Self::typename
    }
    fn typestring() -> String{
        String::from(Self::typename)
    }
}
