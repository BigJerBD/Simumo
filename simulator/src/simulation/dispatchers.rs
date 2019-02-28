use piston_window::OpenGL;
use specs::Dispatcher;
use specs::DispatcherBuilder;

use crate::systems::renderer::DrawClear;
use crate::systems::renderer::DrawRectangles;
use crate::systems::mobility;
use crate::systems::clock;


pub fn make_base_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        //.with(systems::logging::print_sys::PrintLog, "print", &[])
        .with(mobility::StandardMobilitySystem, "pos_update", &[])
        //.with(systems::recorders::CarPosRecSystem::new(0.5), "log_car", &["pos_update"])
        //.with(logger, "logger_sys", &["log_car"])
        .with_barrier()
        .with(clock::StandardClockSystem, "clock_sys", &[])
        .build()
}


pub fn make_render_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with_thread_local(DrawClear)
        .with_thread_local(DrawRectangles)
        .build()
}