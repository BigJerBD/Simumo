use crate::ressources;

use specs::prelude::{System, Write};
use simumo_derive::simusystem;
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct StandardClockSystem;
impl<'a> System<'a> for StandardClockSystem {
    type SystemData = Write<'a, ressources::Clock>;

    fn run(&mut self, mut clock: Self::SystemData) {
        clock.update();
    }
}
