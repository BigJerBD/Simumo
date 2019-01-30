//
//use components::dynamic::*;
//struct PrintLog;
//impl<'a> System<'a> for SpeedUpdate {
//    type SystemData = (ReadStorage<'a, Position>
//    );
//
//    fn run(&mut self, (mut vel, acc): Self::SystemData) {
//        for (vel, acc) in (&mut vel, &acc).join() {
//            vel.0 +=  acc.0;
//        }
//    }
//}
//
