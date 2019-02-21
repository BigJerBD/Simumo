use crate::components::agents::AcceleratingAgent;
use crate::components::controls::EnergyControl;
use crate::systems::agents::AgentSystemType;
use crate::systems::sys_prelude::*;

#[simusystem]
pub struct AcceleratingAgentSystem;

impl AgentSystemType for AcceleratingAgentSystem {}

impl<'a> System<'a> for AcceleratingAgentSystem {
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
