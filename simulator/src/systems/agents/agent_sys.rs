use crate::systems::sys_prelude::*;

pub trait AgentSys: for<'c> System<'c> + TypeInfo + Send +Sync {}