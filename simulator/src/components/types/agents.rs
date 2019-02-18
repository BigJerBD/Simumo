use crate::components::type_prelude::*;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct AcceleratingAgent {
    pub is_decelerating: bool,
}
