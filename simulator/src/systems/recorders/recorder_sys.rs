use crate::systems::sys_prelude::*;

pub trait RecorderSys: for<'c> System<'c> + TypeInfo + Send + Sync {}