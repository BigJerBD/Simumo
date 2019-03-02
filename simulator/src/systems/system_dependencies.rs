/// Struct that contains the systems that are pres
/// todo :: it might be better to be a hashmap, and to make
///  system definition add the dependency
pub struct SystemDependencies {
    pub clock: String,
    pub agents: Vec<String>,
    pub controls: Vec<String>,
    pub physic: String,
    pub mobility: String,
    pub recorders: Vec<String>,
    pub logger: String,
}

impl SystemDependencies {
    pub fn new() -> Self {
        Self {
            clock: String::from(""),
            agents: Vec::new(),
            controls: Vec::new(),
            physic: String::from(""),
            mobility: String::from(""),
            recorders: Vec::new(),
            logger: String::from(""),
        }
    }
}

/// Helper for convenience
trait StringRef {
    fn string_ref(&self) -> Vec<&str>;
}

impl StringRef for Vec<String> {
    fn string_ref(&self) -> Vec<&str> {
        self.iter().map(|s| &**s).collect()
    }
}
