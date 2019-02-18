use crate::systems::sys_prelude::*;

pub trait MobilitySys: for<'c> System<'c> + TypeInfo + Send + Sync {}