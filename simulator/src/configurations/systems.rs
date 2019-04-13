/*! Represent systems from the configuration file.*/

use std::collections::HashMap;

use specs::DispatcherBuilder;

use crate::systems::agents::AgentSystem;
use crate::systems::clock::ClockSystem;
use crate::systems::controls::ControlSystem;
use crate::systems::mobility::MobilitySystem;
use crate::systems::physic::PhysicSystem;
use crate::systems::recorders::RecorderSystem;
use crate::systems::spawners::SpawnerSystem;
use crate::systems::SystemType;

#[derive(Deserialize)]
pub struct SystemsConfiguration {
    pub agents: Vec<AgentSystem>,
    pub clock: ClockSystem,
    pub controls: Vec<ControlSystem>,
    pub mobility: MobilitySystem,
    pub physic: PhysicSystem,
    pub recorders: Vec<RecorderSystem>,
    pub spawner: Option<SpawnerSystem>,
    //pub others : Vec<UnclassifiedSystem>
}

///todo :: consider if it should be implemented somewhere else
impl SystemsConfiguration {
    pub fn declare_systems(&self, system_mapping: &mut HashMap<String, Vec<String>>) {
        system_mapping.insert(AgentSystem::typename(), as_sysname_vec(&self.agents));
        system_mapping.insert(ClockSystem::typename(), vec![self.clock.system_name()]);
        system_mapping.insert(ControlSystem::typename(), as_sysname_vec(&self.controls));
        system_mapping.insert(
            MobilitySystem::typename(),
            vec![self.mobility.system_name()],
        );
        system_mapping.insert(PhysicSystem::typename(), vec![self.physic.system_name()]);
        system_mapping.insert(RecorderSystem::typename(), as_sysname_vec(&self.recorders));
        if let Some(spawner) = &self.spawner {
            system_mapping.insert(SpawnerSystem::typename(), vec![spawner.system_name()]);
        }
    }

    ///Setup all systems in the simulator
    pub fn setup_systems(
        self,
        builder: &mut DispatcherBuilder,
        systems: &HashMap<String, Vec<String>>,
    ) {
        info!("Setting in dispatcher : clock");
        self.clock.set_in_dispatcher(builder, systems);
        info!("Setting in dispatcher : agents");
        set_all_in_dispatcher(self.agents, builder, systems);
        info!("Setting in dispatcher : controls");
        set_all_in_dispatcher(self.controls, builder, systems);
        info!("Setting in dispatcher : physic");
        self.physic.set_in_dispatcher(builder, systems);
        info!("Setting in dispatcher : mobility");
        self.mobility.set_in_dispatcher(builder, systems);
        info!("Setting in dispatcher : recorders");
        set_all_in_dispatcher(self.recorders, builder, systems);
        info!("Setting in dispatcher : logger");
        if let Some(spawner) = self.spawner {
            spawner.set_in_dispatcher(builder, systems);
        }
    }
}

/// used for convenience to set a system in the simulator dispatcher
fn set_all_in_dispatcher<T: SystemType>(
    systems: Vec<T>,
    builder: &mut DispatcherBuilder,
    sys_mapping: &HashMap<String, Vec<String>>,
) {
    for sys in systems {
        sys.set_in_dispatcher(builder, sys_mapping);
    }
}

/// used for convenience
fn as_sysname_vec<T: SystemType>(systems: &[T]) -> Vec<String> {
    systems.iter().map(|a| a.system_name()).collect()
}
