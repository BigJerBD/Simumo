use specs::{Component, Entity, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[derive(Component, TypeInfo, Debug)]
#[storage(VecStorage)]
pub struct Observers {
    pub list: Vec<&'static Entity>
}

impl Observers {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }
    pub fn get_list(&self) -> &Vec<&Entity>{
        &self.list
    }
}