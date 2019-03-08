pub mod curve;
mod road;
mod road_connector;

use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::Graph;
pub use road::Road;
pub use road_connector::RoadConnector;

pub type RoadId = EdgeIndex;
pub type RoadConnectorId = NodeIndex;

#[derive(Debug)]
pub struct Topology {
    pub road_graph: Graph<RoadConnector, Road>,
}

impl Topology {
    // Creates a dummy topology until we can import OSM data
    pub fn new() -> Topology {
        let graph = Graph::<RoadConnector, Road>::new();
        let mut topology = Topology { road_graph: graph };

        let center = topology.add_road_connector(RoadConnector::new());
        let north = topology.add_road_connector(RoadConnector::new());
        let east = topology.add_road_connector(RoadConnector::new());
        let south = topology.add_road_connector(RoadConnector::new());
        let west = topology.add_road_connector(RoadConnector::new());
        let north_east = topology.add_road_connector(RoadConnector::new());
        let north_west = topology.add_road_connector(RoadConnector::new());
        let south_east = topology.add_road_connector(RoadConnector::new());
        let south_west = topology.add_road_connector(RoadConnector::new());

        topology.add_road(north, center, Road::new());
        topology.add_road(east, center, Road::new());
        topology.add_road(west, center, Road::new());
        topology.add_road(south, center, Road::new());
        topology.add_road(north, north_east, Road::new());
        topology.add_road(north, north_west, Road::new());
        topology.add_road(east, north_east, Road::new());
        topology.add_road(east, south_east, Road::new());
        topology.add_road(south, south_east, Road::new());
        topology.add_road(south, south_west, Road::new());
        topology.add_road(west, south_west, Road::new());
        topology.add_road(west, north_west, Road::new());

        topology
    }

    pub fn add_road_connector(&mut self, road_connector: RoadConnector) -> RoadConnectorId {
        let id = self.road_graph.add_node(road_connector);
        self.road_graph[id].id = id;
        id
    }

    pub fn add_road(&mut self, from: RoadConnectorId, to: RoadConnectorId, road: Road) -> RoadId {
        let id = self.road_graph.add_edge(from, to, road);
        self.road_graph[id].id = id;
        id
    }
}
