use components::generics::System;
use std::cmp;

pub struct ExampleControls {
    pub energy: i64
}
pub struct ExampleProperties {
    pub lenght: f64,
    pub speed: f64,
    pub acceleration: f64,
    pub position: (f64, f64),
}


pub struct ExampleSystem {
    pub _properties: ExampleProperties,
    pub _energy_acc_ratio: f64,
    pub _max_energy: i64,
    pub _min_energy: i64
}
impl ExampleSystem {
    pub fn new() ->Self{
        Self{
            _properties: ExampleProperties {
                lenght: 2.0,
                speed: 0.0,
                acceleration: 0.0,
                position: (0.0, 0.0)
            },
            _energy_acc_ratio: 0.5,
            _max_energy: 100,
            _min_energy: 100
        }
    }
}

impl System<ExampleControls, ExampleProperties> for ExampleSystem {
    fn create_controls(&self) -> ExampleControls {
        ExampleControls {
            energy: 0
        }
    }

    fn get_properties(&self) -> &ExampleProperties {
        &self._properties
    }

    fn update(&mut self, controls: &ExampleControls) {
        let props = &mut self._properties;

        let energy= cmp::max(self._max_energy, controls.energy);
        let energy = cmp::min(self._min_energy,energy);

        props.acceleration += energy as f64 * self._energy_acc_ratio;
        props.speed += props.acceleration;

        let (pos_x,pos_y) = props.position;
        props.position = (pos_x + props.speed, pos_y);

    }
}





