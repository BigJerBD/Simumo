use crate::components::constant::CarType;
use crate::components::log_record::LogRecord;
use crate::components::Position;
use crate::ressources;

use simumo_derive::simusystem;
use specs::prelude::{Entities, Join, LazyUpdate, Read, ReadStorage, System};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
#[derive(Default)]
pub struct CarPositionRecorderSystem {
    capture_freq: f64,
}
impl CarPositionRecorderSystem {
    pub fn new(capture_freq: f64) -> Self {
        Self { capture_freq }
    }
}
impl<'a> System<'a> for CarPositionRecorderSystem {
    type SystemData = (
        Read<'a, ressources::Clock>,
        Entities<'a>,
        ReadStorage<'a, CarType>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    /// the run process select the right logger for every
    /// records
    fn run(&mut self, (clock, entities, cars, positions, updater): Self::SystemData) {
        //do a modulo to do it only on a certain frequency...

        for (entity, _, position) in (&entities, &cars, &positions).join() {
            let log_info = entities.create();
            // todo :: currently doesnt serialize until we fix metric problems
            updater.insert(
                log_info,
                LogRecord::new(
                    clock.get_time(),
                    entity.id(),
                    String::from("CarPosition"),
                    Box::new(position.clone()),
                ),
            );
        }
    }
}
