use crate::components::dynamic::*;
use specs::{System, ReadStorage, Join};

pub struct PrintLog;
impl<'a> System<'a> for PrintLog {
   type SystemData = ReadStorage<'a, Position>;

   fn run(&mut self, positions: Self::SystemData) {
       for pos in positions.join() {
           println!("{:#?}", pos);
       }
   }
}
