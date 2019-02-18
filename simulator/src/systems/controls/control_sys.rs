use crate::systems::sys_prelude::*;

pub trait ControlSys: for<'c> System<'c> + TypeInfo + Send + Sync {}