use std::marker::PhantomData;
use std::cell::Cell;
use std::cell::RefCell;


//todo :: remove this when a concrete topology will be established
pub struct TopologyState {}

/// Trait of an Agent
///
/// Agents can chose action based on :
/// - The topology state
/// - The available control
/// - The current property of the system
pub trait Agent<C, P> {
    fn chose_action(
        &mut self,
        control: &mut C,
        properties: &P,
        topology_state: &TopologyState);
}

/// Trait of System of properties
///
/// The properties can be modified by some controls
/// or by an internal update
pub trait System<C, P> {
    fn create_controls(&self) -> C;
    fn get_properties(&self) -> &P;
    fn update(&mut self, controls: &C);
}


/// A Container for an agent and a system
///
pub struct Component<A, S, C, P>
    where A: Agent<C, P>,
          S: System<C, P> {
    _property_marker: PhantomData<P>,

    _component_id: i64,
    _system: S,
    _agent: RefCell<A>,
    _next_action: Cell<Option<C>>,
}

impl<A, S, C, P> Component<A, S, C, P>
    where A: Agent<C, P>,
          S: System<C, P>
{
    pub fn new(id: i64, system: S, agent: A) -> Component<A, S, C, P> {
        Component {
            _property_marker: PhantomData,
            _component_id: id,
            _system: system,
            _agent: RefCell::new(agent),
            _next_action: Cell::new(None),
        }
    }

    pub fn get_properties(&self) -> &P {
        &self._system.get_properties()
    }

    pub fn get_id(&self) -> &i64 {
        &self._component_id
    }

    /// Makes the inside agent choose his next action.
    ///
    /// The choice can be based on the properties, controls, and topology's state.
    pub fn chose_action(&self, topology_state: &TopologyState) {
        let properties = self._system.get_properties();
        let mut controls = self._system.create_controls();

        //note :: by design RefCell check at runtime the number of borrow
        //based on our design it might note be necessary
        let mut agent = self._agent.borrow_mut();
        agent.chose_action(
            &mut controls,
            properties,
            topology_state,
        );

        self._next_action.set(Some(controls));
    }


    /// Takes the next Control in a RefCell,
    /// then update the system with it.
    pub fn update(&mut self) {
        let next_action = self._next_action.get_mut();

        match next_action {
            Some(action) => self._system.update(action),
            None => panic!("No next control for the agent"),
        }
    }
}







