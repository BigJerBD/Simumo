use specs::Dispatcher;
use specs::DispatcherBuilder;

use crate::systems::clock;
use crate::systems::events::EventsHookUpdate;
use crate::systems::events::EventsUpdate;
use crate::systems::loggers::logger_sys::DirectoryStructure;
use crate::systems::loggers::LoggerSystem;
use crate::systems::loggers::types::NdJsonLogger;
use crate::systems::mobility;
use crate::systems::recorders;
use crate::systems::renderer::DrawClear;
use crate::systems::renderer::DrawRectangles;
use crate::systems::statics::LightsUpdate;

pub fn make_base_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(EventsHookUpdate, "eventshook_system", &[])
        .with(LightsUpdate, "color_update", &[])
        //.with(systems::logging::print_sys::PrintLog, "print", &[])
        .with(mobility::StandardMobilitySystem, "pos_update", &[])

//        .with(recorders::CarPosRecSystem::new(0.5),
//              "log_car",
//              &["pos_update"])
//        .with(LoggerSystem::<NdJsonLogger>::from_directory_struct(
//            &DirectoryStructure {
//                directory: "/home/bigjerbd".to_string(),
//                files: vec![String::from("CarPosition")],
//            }
//        ), "logger_sys", &["log_car"])
        .with_barrier()
        .with(EventsUpdate, "events_update", &[])
        .with(clock::StandardClockSystem, "clock_sys", &[])
        .build()
}

pub fn make_render_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with_thread_local(DrawClear)
        .with_thread_local(DrawRectangles)
        .build()
}
