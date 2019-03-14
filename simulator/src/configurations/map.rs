use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use specs::World;

use crate::ressources::generals;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use dim::si::M;

#[derive(Deserialize)]
struct FileMap {
    edges: HashMap<u64, u64>,
    nodes: HashMap<u64, (f64, f64)>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Map {
    FileMap {
        path: String,
    },
    OsmGraph {
        longitude: f64,
        latitude: f64,
        zoom: i64,
    },
}

impl Map {
    pub fn forward_ressources(&self, world: &mut World) {
        match self {
            Map::FileMap { path } => {
                create_ressource_lanegraph(lanemap_from_path(path.to_string()), world)
            }
            Map::OsmGraph {
                longitude,
                latitude,
                zoom,
            } => create_ressource_lanegraph(
                LaneGraph::from_pyosmgraph(*longitude, *latitude, *zoom),
                world,
            ),
        }
    }
}

fn lanemap_from_path(path: String) -> LaneGraph {
    let path = Path::new(&path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let map: FileMap = serde_json::from_reader(reader).unwrap();
    LaneGraph::new(
        map.nodes
            .iter()
            .map(|(id, (lon, lat))| (*id, IntersectionData::new(*lon, *lat))),
        map.edges
            .iter()
            .map(|(from, to)| (*from, *to, LaneData::new(None, None, None))),
    )
}

fn create_ressource_lanegraph(lanegraph: LaneGraph, world: &mut World) {
    let positions: Vec<(f64, f64)> = lanegraph
        .intersections
        .values()
        .map(|v| v.position())
        .collect();

    world.add_resource(generals::MapBbox {
        x1: positions.iter().map(|v| v.0).fold(std::f64::NAN, f64::min),
        x2: positions.iter().map(|v| v.0).fold(std::f64::NAN, f64::max),
        y1: positions.iter().map(|v| v.1).fold(std::f64::NAN, f64::min),
        y2: positions.iter().map(|v| v.1).fold(std::f64::NAN, f64::max),
    });
    world.add_resource(lanegraph);
}
