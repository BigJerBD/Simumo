use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Index;
use std::ops::IndexMut;

use petgraph::graphmap::DiGraphMap;
use petgraph::graphmap::GraphMap;
use petgraph::IntoWeightedEdge;
use specs::world;

use crate::metrics::Fdim;
use crate::osmgraph_api::OsmGraphApi;
use crate::osmgraph_api::PythonOsmGraphApi;
use dim::si::{Meter, MeterPerSecond, M};

use crate::topology::curve::Curve;

pub type IntersectionId = u64;
/// used for convenience
type NodeId = IntersectionId;

/// The Identifier of the entities in the graph
/// it uses the entities ID of specs
pub type EntityId = world::Index;

/// A GraphMap of the map of the lane
///
/// # Fields
///
/// * `graph` - graph containing information of the lanes base on the `IntersectionId`
/// * `intersections` -  mapping of the intersection data based on their `IntersectionId`
/// * `entity_locations` - locations of the entities in the graph
///
pub struct LaneGraph {
    pub graph: DiGraphMap<NodeId, LaneData>,
    pub intersections: HashMap<NodeId, IntersectionData>,
    pub entity_locations: HashMap<EntityId, (NodeId, NodeId)>,
}

impl LaneGraph {
    pub fn new<I1, I2>(nodes: I1, edges: I2) -> Self
    where
        I1: Iterator<Item = (NodeId, IntersectionData)>,
        I2: IntoIterator,
        I2::Item: IntoWeightedEdge<LaneData, NodeId = NodeId>,
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
            .map(|(id, (lon, lat))| (*id, IntersectionData::new(*lon + 100.0, *lat + 50.0)))
            .collect();

        let edges: Vec<(_, _, _)> = osmgraph
            .get_edges()
            .unwrap()
            .iter()
            // todo :: replace the none by the valid values
            .map(|(from, to)| (*from, *to, LaneData::new(None, None, None)))
            .collect();

        Self::new(nodes.into_iter(), edges.into_iter())
    }

    /// Take the entity in front of the lane `from`
    /// and put it at the back of the lane `to`
    ///
    pub fn node_forward(&mut self, from: (NodeId, NodeId), to: (NodeId, NodeId)) {
        let front_entity = { self.lane_between_mut(from).pop_front() };
        self.lane_between_mut(to).push_back(front_entity);
    }

    /// forward a tuple of three node
    ///
    pub fn segment_forward(&mut self, (from, middle, to): (NodeId, NodeId, NodeId)) {
        self.node_forward((from, middle), (middle, to));
    }

    /// Take the selected entity ID in the end of a lane
    /// and then move it to the front of an other lane
    ///
    pub fn entity_forward(&mut self, entity: EntityId, destination: NodeId) {
        let (begin, end) = self.entity_locations[&entity];
        self.segment_forward((begin, end, destination));
    }

    // Method to access

    pub fn intersections(&self) -> &HashMap<NodeId, IntersectionData> {
        &self.intersections
    }
    pub fn lanes(&self) -> &DiGraphMap<NodeId, LaneData> {
        &self.graph
    }
    pub fn entity_locations(&self) -> &HashMap<EntityId, (NodeId, NodeId)> {
        &self.entity_locations
    }

    /// get a reference of the intersection
    ///
    pub fn intersection(&self, entity: NodeId) -> &IntersectionData {
        &self.intersections[&entity]
    }

    /// get a mutable reference on the intersection
    ///
    pub fn intersection_mut(&mut self, entity: NodeId) -> &mut IntersectionData {
        self.intersections.get_mut(&entity).unwrap()
    }

    /// get the lane with entity id
    ///
    pub fn lane(&self, entity: EntityId) -> &LaneData {
        let location = self.entity_locations[&entity];
        self.graph.index(location)
    }

    /// get the lane between two node
    ///
    pub fn lane_between(&self, location: (NodeId, NodeId)) -> &LaneData {
        self.graph.index(location)
    }

    /// get the lane as a mutable lane based on the entityId
    ///
    pub fn lane_mut(&mut self, entity: EntityId) -> LaneEntry {
        let location = self.entity_locations[&entity];
        let lane = self.graph.index_mut(location);
        LaneEntry {
            lane,
            lane_location: location,
            entity_locations: &mut self.entity_locations,
        }
    }

    /// Get the lane as a mutable lane between two nodes
    ///
    pub fn lane_between_mut(&mut self, location: (NodeId, NodeId)) -> LaneEntry {
        LaneEntry {
            lane: self.graph.index_mut(location),
            lane_location: location,
            entity_locations: &mut self.entity_locations,
        }
    }
}

/// Access Entry that allows to modify the LaneMap while keeping its integrity
///
/// # Fields
///
/// * `lane` :  mut ref of the currently selected lane
/// * `lane_location` : location of the lane in the graph
/// * `entity_locations` : mut ref of the mapping of all entity locations
///
/// those reference will of course be released when releasing LaneEntry
pub struct LaneEntry<'a, 'b> {
    lane: &'b mut LaneData,
    lane_location: (NodeId, NodeId),
    entity_locations: &'a mut HashMap<EntityId, (NodeId, NodeId)>,
}

impl<'a, 'b> LaneEntry<'a, 'b> {
    pub fn lane(&self) -> &LaneData {
        self.lane
    }

    fn push_back(&mut self, entity: EntityId) {
        self.entity_locations.insert(entity, self.lane_location);
        self.lane.push_back(entity);
    }
    fn pop_front(&mut self) -> EntityId {
        let entity = self.lane.pop_front();
        self.entity_locations.remove(&entity);
        entity
    }

    fn pop_if_front(&mut self, entity: EntityId) -> Option<EntityId> {
        let _ = self.lane.pop_if_front(entity)?;
        self.entity_locations.remove(&entity);
        Some(entity)
    }
}

/// Contains all the information of a lane in the map
///
/// # Fields
///
/// * `entity_queue` - ordered queue giving the order of the contained elements
/// * `width` - width of the lane
/// * `max_speed` - max speed of the lane
/// * `curve` - curve of the lane
///
/// note :: `width`,`max_speed` and `curve`are options because we
///     are not garrenteed yet to have it for everylane
#[derive(Clone, Debug)]
pub struct LaneData {
    entity_queue: VecDeque<EntityId>,
    //todo :: consider if all the specific data  (width,max_speed,etc)
    // should be wrapped in a generic this way we could  abstract street info
    // from the graph w
    pub width: Option<Meter<Fdim>>,
    pub max_speed: Option<MeterPerSecond<Fdim>>,
    pub curve: Option<Curve>,
}

impl LaneData {
    pub fn new(
        width: Option<Meter<Fdim>>,
        max_speed: Option<MeterPerSecond<Fdim>>,
        curve: Option<Curve>,
    ) -> Self {
        Self {
            entity_queue: VecDeque::new(),
            width,
            max_speed,
            curve,
        }
    }

    /// get a reference of the queue
    ///
    pub fn queue(&self) -> &VecDeque<EntityId> {
        &self.entity_queue
    }

    /// Insert a entity at the beginning of the lane
    ///
    /// note :: we use the back of de entity queue because
    ///         it makes more sense in our context
    pub fn push_back(&mut self, entity: EntityId) {
        self.entity_queue.push_back(entity);
    }

    /// pop an entity at the end of the lane
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

        self.entity_queue[pos + 1]
    }
}

///  Contains all the information of an intersection in the map
///
///  # Fields
///
/// * `position` - position in longitude latitude
/// * `contained_entity` - Index referencing to the contained entity
///
#[derive(Clone, Debug)]
pub struct IntersectionData {
    position: (f64, f64),
    contained_entity: Option<EntityId>,
}

impl IntersectionData {
    pub fn new(lon: f64, lat: f64) -> Self {
        Self {
            position: (lon, lat),
            contained_entity: None,
        }
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
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
    fn lane_map_triangle() -> LaneGraph {
        let node = IntersectionData::new(10.0, 10.0);

        LaneGraph::new(
            [
                (1u64, node.clone()),
                (2u64, node.clone()),
                (3u64, node.clone()),
                (4u64, node.clone()),
            ]
            .to_vec()
            .into_iter(),
            &[
                (1, 3, LaneData::new(None, None, None)),
                (2, 3, LaneData::new(None, None, None)),
                (3, 4, LaneData::new(None, None, None)),
            ],
        )
    }

    #[test]
    fn push_valid() {
        let mut graph = lane_map_triangle();
        graph.lane_between_mut((1, 3)).push_back(1);
        graph.lane_between_mut((1, 3)).push_back(2);
        let mut lane = graph.lane_between_mut((1, 3));
        assert_eq!(lane.lane().queue().len(), 2);
        assert_eq!(lane.pop_front(), 1);
        assert_eq!(lane.pop_front(), 2);
        assert!(lane.lane().queue().is_empty());
    }

    #[test]
    fn node_forward_gives_2314() {
        let mut graph = lane_map_triangle();
        graph.lane_between_mut((1, 3)).push_back(1);
        graph.lane_between_mut((1, 3)).push_back(2);
        graph.lane_between_mut((2, 3)).push_back(3);
        graph.lane_between_mut((3, 4)).push_back(4);

        graph.node_forward((1, 3), (3, 4));
        graph.node_forward((2, 3), (3, 4));
        graph.node_forward((1, 3), (3, 4));

        let lane = graph.lane_between((3, 4));
        assert_eq!(lane.queue().get(0).unwrap(), &4);
        assert_eq!(lane.queue().get(1).unwrap(), &1);
        assert_eq!(lane.queue().get(2).unwrap(), &3);
        assert_eq!(lane.queue().get(3).unwrap(), &2);

        assert_eq!(lane.queue().len(), 4);
        assert!(graph.lane_between((1, 3)).queue().is_empty());
        assert!(graph.lane_between((2, 3)).queue().is_empty());
    }

}
