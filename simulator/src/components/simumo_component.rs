use specs::Component;
use typeinfo::TypeInfo;
use specs::World;

pub trait SimumoComponent: TypeInfo + Component {
    fn register(&self, simulation: &mut World, isdone: &mut bool);
}