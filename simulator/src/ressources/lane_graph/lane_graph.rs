use crate::commons::Curve;
use crate::ressources::lane_graph::{IntersectionData,LaneData,LaneEntry};
use petgraph::graphmap::DiGraphMap;
use petgraph::graphmap::GraphMap;
use petgraph::IntoWeightedEdge;
use specs::world;
use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

pub type NodeId = u64;
pub type EdgeId = (NodeId, NodeId);

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
        LaneEntry::new(lane, location, &mut self.entity_locations)
    }

    /// Get the lane as a mutable lane between two nodes
    ///
    pub fn lane_between_mut(&mut self, location: (NodeId, NodeId)) -> LaneEntry {
        LaneEntry::new(
            self.graph.index_mut(location),
            location,
            &mut self.entity_locations,
        )
    }
}

/*
impl GraphBase for LaneGraph {
    type EdgeId = EdgeId;
    type NodeId = NodeId;
}

impl IntoEdgeReferences for LaneGraph {
    type EdgeRef: EdgeRef<NodeId = Self::NodeId, EdgeId = Self::EdgeId, Weight = Self::EdgeWeight>;
    type EdgeReferences: Iterator<Item = Self::EdgeRef>;

    fn edge_references(self) -> Self::EdgeReferences {

    }
}

impl IntoEdges for LaneGraph {
    type Edges = Iterator<Item = Self::EdgeRef>;
}

impl IntoNeighbors for LaneGraph {
    type Neighbors = Iterator<Item = IntersectionData>;

    fn neighbors(self, nodeid: Self::NodeId) -> Self::Neighbors {
        let neighbors: Self::Neighbors =
            self.graph.neighbors(*nodeid).collect();
        IntoIterator::into_iter(neighbors)
    }
}
*/

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

        use crate::commons::Point2D;
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
                (
                    1,
                    3,
                    LaneData::new(
                        None,
                        None,
                        Curve::new(vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 }]),
                    ),
                ),
                (
                    2,
                    3,
                    LaneData::new(
                        None,
                        None,
                        Curve::new(vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 }]),
                    ),
                ),
                (
                    3,
                    4,
                    LaneData::new(
                        None,
                        None,
                        Curve::new(vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 }]),
                    ),
                ),
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
