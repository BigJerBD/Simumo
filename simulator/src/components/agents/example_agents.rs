use components::generics::{Agent, TopologyState};
use components::systems::example_system::{ExampleProperties, ExampleControls};


/// Example of Agent doing a simple Action
///
///
pub struct ExampleAgent {}
impl Agent<ExampleControls, ExampleProperties> for ExampleAgent {
    fn chose_action(&mut self,
                    control: &mut ExampleControls,
                    _properties: &ExampleProperties,
                    _topology_state: &TopologyState)
    {
        control.energy += 1
    }
}