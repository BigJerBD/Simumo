use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Index;
use std::ops::IndexMut;

use petgraph::graphmap::DiGraphMap;
use petgraph::graphmap::GraphMap;
use petgraph::IntoWeightedEdge;
use specs::world;

use crate::osmgraph_api::OsmGraphApi;
use crate::osmgraph_api::PythonOsmGraphApi;

pub type IntersectionId = u64;
/// used for convenience
type NodeId = IntersectionId;

/// The Identifier of the entities in the graph
/// it uses the entities ID of specs
pub type EntityId = world::Index;

/// A GraphMap of the map of the street
///
/// # Fields
///
/// * `graph` - graph containing information of the streets base on the `IntersectionId`
/// * `intersections` -  mapping of the intersection data based on their `IntersectionId`
/// * `entity_locations` - locations of the entities in the graph
///
pub struct StreetMap {
    pub graph: DiGraphMap<NodeId, StreetData>,
    pub intersections: HashMap<NodeId, IntersectionData>,
    pub entity_locations: HashMap<EntityId, (NodeId, NodeId)>,
}

impl StreetMap {
    pub fn new<I1, I2>(nodes: I1, edges: I2) -> Self
    where
        I1: Iterator<Item = (NodeId, IntersectionData)>,
        I2: IntoIterator,
        I2::Item: IntoWeightedEdge<StreetData, NodeId = NodeId>,
    {
        Self {
            graph: GraphMap::from_edges(edges),
            intersections: nodes.collect::<HashMap<_, _>>(),
            entity_locations: HashMap::new(),
        }
    }

    /// Uses the osmgraph api to make the graph
    ///
    /// todo :: find a way to make this function more generic?
    pub fn from_pyosmgraph(lon: f64, lat: f64, zoom: i64) -> Self {
        let osmgraph = *PythonOsmGraphApi::query_graph(lon, lat, zoom).unwrap();

        let nodes: Vec<(_, _)> = osmgraph
            .get_nodes()
            .unwrap()
            .iter()
            .map(|(id, (lon, lat))| (*id, IntersectionData::new(*lon, *lat)))
            .collect();

        let edges: Vec<(_, _, _)> = osmgraph
            .get_edges()
            .unwrap()
            .iter()
            .map(|(from, to)| (*from, *to, StreetData::new()))
            .collect();

        Self::new(nodes.into_iter(), edges.into_iter())
    }

    /// Take the entity in front of the street `from`
    /// and put it at the back of the street `to`
    ///
    pub fn node_forward(&mut self, from: (NodeId, NodeId), to: (NodeId, NodeId)) {
        let front_entity = { self.street_between_mut(from).pop_front() };
        self.street_between_mut(to).push_back(front_entity);
    }

    /// forward a tuple of three node
    ///
    pub fn segment_forward(&mut self, (from, middle, to): (NodeId, NodeId, NodeId)) {
        self.node_forward((from, middle), (middle, to));
    }

    /// Take the selected entity ID in the end of a street
    /// and then move it to the front of an other street
    ///
    pub fn entity_forward(&mut self, entity: EntityId, destination: NodeId) {
        let (begin, end) = *self.entity_locations.get(&entity).unwrap();
        self.segment_forward((begin, end, destination));
    }

    // Method to access

    /// get a reference of the intersection
    ///
    pub fn intersection(&self, entity: NodeId) -> &IntersectionData {
        self.intersections.get(&entity).unwrap()
    }

    /// get a mutable reference on the intersection
    ///
    pub fn intersection_mut(&mut self, entity: NodeId) -> &mut IntersectionData {
        self.intersections.get_mut(&entity).unwrap()
    }

    /// get the street with entity id
    ///
    pub fn street(&self, entity: EntityId) -> &StreetData {
        let location = self.entity_locations.get(&entity).unwrap();
        self.graph.index(*location)
    }

    /// get the street between two node
    ///
    pub fn street_between(&self, location: (NodeId, NodeId)) -> &StreetData {
        self.graph.index(location)
    }

    /// get the street as a mutable street based on the entityId
    ///
    pub fn street_mut(&mut self, entity: EntityId) -> StreetEntry {
        let location = self.entity_locations.get(&entity).unwrap();
        let street = self.graph.index_mut(*location);
        StreetEntry {
            street,
            street_location: *location,
            entity_locations: &mut self.entity_locations,
        }
    }

    /// Get the street as a mutable street between two nodes
    ///
    pub fn street_between_mut(&mut self, location: (NodeId, NodeId)) -> StreetEntry {
        StreetEntry {
            street: self.graph.index_mut(location),
            street_location: location,
            entity_locations: &mut self.entity_locations,
        }
    }
}

/// Access Entry that allows to modify the StreetMap while keeping its integrity
///
/// # Fields
///
/// * `street` :  mut ref of the currently selected street
/// * `street_location` : location of the street in the graph
/// * `entity_locations` : mut ref of the mapping of all entity locations
///
/// those reference will of course be released when releasing StreetEntry
pub struct StreetEntry<'a, 'b> {
    street: &'b mut StreetData,
    street_location: (NodeId, NodeId),
    entity_locations: &'a mut HashMap<EntityId, (NodeId, NodeId)>,
}

impl<'a, 'b> StreetEntry<'a, 'b> {
    pub fn street(&self) -> &StreetData {
        self.street
    }

    fn push_back(&mut self, entity: EntityId) {
        self.entity_locations.insert(entity, self.street_location);
        self.street.push_back(entity);
    }
    fn pop_front(&mut self) -> EntityId {
        let entity = self.street.pop_front();
        self.entity_locations.remove(&entity);
        entity
    }

    fn pop_if_front(&mut self, entity: EntityId) -> Option<EntityId> {
        let _ = self.street.pop_if_front(entity)?;
        self.entity_locations.remove(&entity);
        Some(entity)
    }
}

/// Contains all the information of a street in the map
///
/// # Fields
///
/// * `entity_queue` - ordered queue giving the order of the contained elements
///
#[derive(Clone)]
pub struct StreetData {
    entity_queue: VecDeque<EntityId>,
}

impl StreetData {
    pub fn new() -> Self {
        Self {
            entity_queue: VecDeque::new(),
        }
    }

    /// get a reference of the queue
    ///
    pub fn queue(&self) -> &VecDeque<EntityId> {
        &self.entity_queue
    }

    /// Insert a entity at the beginning of the street
    ///
    /// note :: we use the back of de entity queue because
    ///         it makes more sense in our context
    pub fn push_back(&mut self, entity: EntityId) {
        self.entity_queue.push_back(entity);
    }

    /// pop an entity at the end of the street
    ///
    ///
    pub fn pop_front(&mut self) -> EntityId {
        self.entity_queue.pop_front().unwrap()
    }

    /// remove if the entity is in front of the queue
    ///
    /// todo :: result instead of option?
    pub fn pop_if_front(&mut self, entity: EntityId) -> Option<EntityId> {
        let front_entity = self.entity_queue.front()?;
        if *front_entity != entity {
            None
        } else {
            self.entity_queue.pop_front()
        }
    }

    /// give the entity which is in front of an other entity
    ///
    pub fn in_front_of(&self, entity: EntityId) -> EntityId {
        let pos = self.entity_queue.iter().position(|x| x == &entity).unwrap();

        *self.entity_queue.get(pos + 1).unwrap()
    }
}

///  Contains all the information of an intersection in the map
///
///  # Fields
///
/// * `position` - position in longitude latitude
/// * `contained_entity` - Index referencing to the contained entity
///
#[derive(Clone)]
pub struct IntersectionData {
    position: (f64, f64),
    contained_entity: Option<EntityId>,
}

impl IntersectionData {
    fn new(lon: f64, lat: f64) -> Self {
        Self {
            position: (lon, lat),
            contained_entity: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// get a map in a triangle :
    /// it uses stub position in the nodes
    ///
    ///   1
    ///    \
    ///     3 --> 4
    ///    /
    ///   2
    ///
    fn streetmap_triangle() -> StreetMap {
        let node = IntersectionData::new(10.0, 10.0);

        StreetMap::new(
            [
                (1u64, node.clone()),
                (2u64, node.clone()),
                (3u64, node.clone()),
                (4u64, node.clone()),
            ]
            .to_vec()
            .into_iter(),
            &[
                (1, 3, StreetData::new()),
                (2, 3, StreetData::new()),
                (3, 4, StreetData::new()),
            ],
        )
    }

    #[test]
    fn push_valid() {
        let mut graph = streetmap_triangle();
        graph.street_between_mut((1, 3)).push_back(1);
        graph.street_between_mut((1, 3)).push_back(2);
        let mut street = graph.street_between_mut((1, 3));
        assert_eq!(street.street().queue().len(), 2);
        assert_eq!(street.pop_front(), 1);
        assert_eq!(street.pop_front(), 2);
        assert!(street.street().queue().is_empty());
    }

    #[test]
    fn node_forward_gives_2314() {
        let mut graph = streetmap_triangle();
        graph.street_between_mut((1, 3)).push_back(1);
        graph.street_between_mut((1, 3)).push_back(2);
        graph.street_between_mut((2, 3)).push_back(3);
        graph.street_between_mut((3, 4)).push_back(4);

        graph.node_forward((1, 3), (3, 4));
        graph.node_forward((2, 3), (3, 4));
        graph.node_forward((1, 3), (3, 4));

        let street = graph.street_between((3, 4));
        assert_eq!(street.queue().get(0).unwrap(), &4);
        assert_eq!(street.queue().get(1).unwrap(), &1);
        assert_eq!(street.queue().get(2).unwrap(), &3);
        assert_eq!(street.queue().get(3).unwrap(), &2);

        assert_eq!(street.queue().len(), 4);
        assert!(graph.street_between((1, 3)).queue().is_empty());
        assert!(graph.street_between((2, 3)).queue().is_empty());
    }

}
