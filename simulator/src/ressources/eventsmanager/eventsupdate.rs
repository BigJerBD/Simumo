use specs::prelude::*;
use crate::{Identifier};
use super::EventsManager;

pub struct EventsHookUpdate;
impl<'a> System<'a> for EventsHookUpdate {
    type SystemData = (
        Write<'a, EventsManager>,
        Entities<'a>,
        ReadStorage<'a, Identifier>
    );

    fn run(&mut self, (mut eventsmanager, entities, identifiers): Self::SystemData) {
        /*for (entity, identifier, observableslist) in (&entities, &identifiers, &mut observables).join() {
            let currentObservables: &Vec<&Entity> = observableslist.get_list();
            for currentObservable in currentObservables {
                
            }
        }*/
    }
}

pub struct EventsUpdate;
impl<'a> System<'a> for EventsUpdate {
    type SystemData = (
        Write<'a, EventsManager>
    );

    fn run(&mut self, mut eventsmanager: Self::SystemData) {
        eventsmanager.swap_events();
    }
}