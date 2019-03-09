use specs::Dispatcher;
use specs::DispatcherBuilder;

use crate::systems::clock;
use crate::systems::events::EventsHookUpdate;
use crate::systems::events::EventsUpdate;
use crate::systems::mobility;
use crate::systems::renderer::DrawClear;
use crate::systems::renderer::DrawMap;
use crate::systems::renderer::DrawTrafficLights;
use crate::systems::renderer::DrawVehicles;
use crate::systems::print::PrintSystem;
use crate::systems::spawners::SpawnerSystem;
use crate::systems::statics::LightsUpdate;

pub fn make_base_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(SpawnerSystem, "spawner_system", &[])
        .with(EventsHookUpdate, "eventshook_system", &[])
        .with(LightsUpdate, "color_update", &[])
        //.with(systems::logging::print_sys::PrintLog, "print", &[])
        .with(mobility::StandardMobilitySystem, "pos_update", &[])
        //.with(systems::recorders::CarPosRecSystem::new(0.5), "log_car", &["pos_update"])
        //.with(logger, "logger_sys", &["log_car"])
        .with_barrier()
        .with(EventsUpdate, "events_update", &[])
        .with(clock::StandardClockSystem, "clock_sys", &[])
        //.with(PrintSystem, "print_sys", &[])
        .build()
}

pub fn make_render_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with_thread_local(DrawClear)
        .with_thread_local(DrawMap)
        .with_thread_local(DrawTrafficLights)
        .with_thread_local(DrawVehicles)
        .build()
}
