use std::collections::HashMap;

use crate::components::constant::CarType;
use crate::components::dynamic::Position;
use crate::components::log_record::LogRecord;
use crate::ressources::clock;
use crate::systems::recorders::RecorderSysType;
use crate::systems::sys_prelude::*;
use crate::systems::system_definition::invalid_field;

#[simusystem]
#[derive(Default)]
pub struct CarPosRec {
    capture_freq: f64,
}

impl SystemDefinition for CarPosRec {
    fn initialize(&mut self, config: HashMap<String, FieldValue>,
                  _general_config: HashMap<String, String>) -> Result<(), InvalidNameError> {
        self.capture_freq = match config
            .get("capture_frequency")
            .ok_or(invalid_field("aa"))?
            {
                FieldValue::StringVal(val) => val.parse::<f64>(),
                FieldValue::ArrayVal(_) => panic!("error")
            }.unwrap();


        Ok(())
    }
}

impl RecorderSysType for CarPosRec {}
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
