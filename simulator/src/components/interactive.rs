use specs::prelude::{Component,VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct AgentType(pub Box<str>);