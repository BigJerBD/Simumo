use crate::systems::sys_prelude::*;

pub trait ClockSys: for<'c> System<'c> + TypeInfo + Send +Sync {}
