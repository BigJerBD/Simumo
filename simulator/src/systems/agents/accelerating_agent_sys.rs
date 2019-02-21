use crate::components::agents::AcceleratingAgent;
use crate::components::controls::EnergyControl;

use crate::systems::agents::AgentSysType;
use crate::systems::sys_prelude::*;
use crate::systems::system_definition::SystemDefinition;

#[simusystem]
pub struct AcceleratingAgentSys;
impl SystemDefinition for AcceleratingAgent{}
impl AgentSysType for AcceleratingAgentSys{}


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
