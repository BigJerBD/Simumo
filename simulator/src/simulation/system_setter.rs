use std::collections::HashMap;

use specs::Dispatcher;
use specs::DispatcherBuilder;
use specs::System;

/// similar to a dispatcher builder
/// but keep in memory the dependencies for more intelligent setup
///
/// # Fields
struct SystemSetter<'a, 'b> {
    dependencies: HashMap<String, Vec<String>>,
    dispatcher: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> SystemSetter<'a, 'b> {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dispatcher: DispatcherBuilder::new(),
        }
    }

    pub fn with<T>(mut self, system: T, system_type: String, name: &str, dep: &[&str]) -> Self
    where
        T: for<'c> System<'c> + Send + 'a,
    {
        self.add(system, system_type, name, dep);
        self
    }

    pub fn add<T>(&mut self, system: T, system_type: String, name: &str, dep: &[&str])
    where
        T: for<'c> System<'c> + Send + 'a,
    {
        self.dependencies
            .entry(system_type)
            .or_insert_with(Vec::new)
            .push(String::from(name));

        self.dispatcher.add(system, name, dep)
    }

    pub fn build(self) -> Dispatcher<'a, 'b> {
        self.dispatcher.build()
    }

    pub fn dependencies(&self) -> &HashMap<String, Vec<String>> {
        &self.dependencies
    }
}
