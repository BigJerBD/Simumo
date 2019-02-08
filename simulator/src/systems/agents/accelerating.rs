use crate::components::agents::*;
use crate::components::controls::*;
use specs::prelude::*;

pub struct AcceleratingAgentSys;

impl<'a> System<'a> for AcceleratingAgentSys {
    type SystemData = (
        WriteStorage<'a, AcceleratingAgent>,
        WriteStorage<'a, EnergyControl>,
    );

    fn run(&mut self, (mut agnts, mut ctrls): Self::SystemData) {
        for (mut agnt, mut ctrl) in (&mut agnts, &mut ctrls).join() {
            if ctrl.0 > 100 {
                agnt.is_decelerating = true;
            } else if ctrl.0 < -100 {
                agnt.is_decelerating = false;
            }

            if agnt.is_decelerating {
                ctrl.0 -= 1
            } else {
                ctrl.0 += 1
            }
        }
    }
}
