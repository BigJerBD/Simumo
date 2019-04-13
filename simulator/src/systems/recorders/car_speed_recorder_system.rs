use crate::components::constant::CarType;
use crate::components::log_record::LogRecord;
use crate::components::Position;
use crate::ressources;
use crate::commons::CartesianCoord;
use crate::commons::PolarCoord;
use crate::components::types::dynamic::Speed;
use crate::ressources::lane_graph::LaneGraph;
use rts_logger::LogSender;
use simumo_derive::simusystem;
use specs::prelude::{Entities, Join, Read, ReadStorage, System};
use specs::ReadExpect;
use specs::Resources;
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct CarSpeedRecorderSystem {
    capture_freq: f64,
    #[serde(skip)]
    car_log: Option<LogSender>,
}

impl<'a> System<'a> for CarSpeedRecorderSystem {
    type SystemData = (
        Read<'a, ressources::Clock>,
        Entities<'a>,
        ReadStorage<'a, CarType>,
        ReadStorage<'a, Speed>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, LaneGraph>,
    );

    /// the run process select the right logger for every
    /// records
    fn run(&mut self, (clock, entities, cars, speed, positions, lane_graph): Self::SystemData) {
        //do a modulo to do it only on a certain frequency...
        if clock.get_time().value_unsafe % self.capture_freq == 0.0 {
            for (entity, _, speed, pos) in (&entities, &cars, &speed, &positions).join() {
                if let Some(data) = lane_graph.lane_between(pos.val.0) {
                    let cpoint = data.curve().get_location_at_percentage(pos.val.1);
                    let ccoord = CartesianCoord::from_float(cpoint.point().x, cpoint.point().y);
                    let pcoord = PolarCoord::from_cartesian(&ccoord);

                    let _record = LogRecord::new(
                        clock.get_time(),
                        entity.id(),
                        (pcoord.0.clone(), pcoord.1.clone()),
                        String::from("CarSpeed"),
                        Box::new(speed.clone()),
                    );

                    match &self.car_log {
                        Some(log) => log.log(Box::new(_record)),
                        None => (),
                    };
                }
            }
        }
    }

    fn setup(&mut self, _: &mut Resources) {
        self.car_log = Some(LogSender::new(String::from("car_speed")));
    }
}
