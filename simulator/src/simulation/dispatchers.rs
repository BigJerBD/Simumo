use specs::Dispatcher;
use specs::DispatcherBuilder;
use crate::systems::unclassified::EventsHookUpdate;
use crate::systems::unclassified::EventsUpdate;
use crate::systems::unclassified::SpawnerSystem;
use crate::systems::renderer::DrawClear;
use crate::systems::renderer::DrawMap;
use crate::systems::renderer::DrawTrafficLights;
use crate::systems::renderer::DrawVehicles;
use crate::systems::controls::LightControl;

pub fn add_starting_systems(dispatcher_builder: &mut DispatcherBuilder) {
    dispatcher_builder.add_barrier();
    dispatcher_builder.add(SpawnerSystem, "spawner_system", &[]);
    dispatcher_builder.add(EventsHookUpdate, "eventshook_system", &[]);
    dispatcher_builder.add(LightControl, "color_update", &[]);
    dispatcher_builder.add_barrier();
}

pub fn add_ending_systems(dispatcher_builder: &mut DispatcherBuilder) {
    dispatcher_builder.add_barrier();
    dispatcher_builder.add(EventsUpdate, "events_update", &[]);
    dispatcher_builder.add_barrier();
}

pub fn make_render_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with_thread_local(DrawClear)
        .with_thread_local(DrawMap)
        .with_thread_local(DrawTrafficLights)
        .with_thread_local(DrawVehicles)
        .build()
}
