use crate::components::type_prelude::*;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct AcceleratingAgent {
    pub is_decelerating: bool,
}

/*#[derive(Serialize, Deserialize, Debug)]
pub struct ImmobileAgent {
    pub id: Identifier,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstantSpeedAgent {
    pub id: Identifier,
    pub position: Position,
    pub speed: Speed,
}*/