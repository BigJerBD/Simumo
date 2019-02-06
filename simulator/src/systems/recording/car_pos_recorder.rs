use specs::Entities;
use specs::Resources;
use specs::{Join, Read, ReadStorage, System};

use crate::components::constant::CarType;
use crate::components::dynamic::Position;
use crate::components::log_record::CarPositionLog;
use crate::components::log_record::LogRecord;
use crate::ressources::{clock, generals};
use specs::LazyUpdate;
use specs::WriteStorage;

pub struct CarPosRec {
    capture_freq: f64,
}

impl CarPosRec {
    pub fn new(capture_freq: f64) -> Self {
        Self { capture_freq }
    }
}

impl<'a> System<'a> for CarPosRec {
    type SystemData = (
        Read<'a, clock::Clock>,
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
            let log_data = Box::new(CarPositionLog {
                id: entity.id(),
                y: position.y,
                x: position.x,
            });
            updater.insert(log_info, LogRecord::new(log_data))
        }
    }
}
