use crate::systems::sys_prelude::*;

pub trait PhysicSys: for<'c> System<'c> + TypeInfo + Send + Sync {}