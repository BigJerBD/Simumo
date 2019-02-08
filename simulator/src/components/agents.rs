use specs::prelude::{Component, VecStorage};
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct AcceleratingAgent {
    pub is_decelerating: bool,
}
