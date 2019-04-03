/*! Represent the map on different type. */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::commons::CartesianCoord;
use crate::commons::PolarCoord;
use crate::ressources::generals::MapBbox;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;

///Define different type of map.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Map {
    FileMap {
        path: String,
    },
    ///Constructed from Open Street Map API.
    OsmGraph {
        longitude: f64,
        latitude: f64,
        zoom: i64,
    },
}

///Represent the structure of a file map.
#[derive(Deserialize)]
struct FileMap {
    edges: Vec<(u64, u64)>,
    nodes: HashMap<u64, (f64, f64)>,
}

impl Map {
    pub fn forward_ressources(&self) -> (LaneGraph, MapBbox) {
        match self {
            Map::FileMap { path } => {
                let lane_graph: LaneGraph = lanemap_from_file_map(path.to_string());
                let bbox: MapBbox = compute_bounding_box(&lane_graph);
                (lane_graph, bbox)
            }
            Map::OsmGraph {
                longitude,
                latitude,
                zoom,
            } => {
                let pos = polarfloat_to_cartesiantuple(*latitude, *longitude);
                let lane_graph: LaneGraph = LaneGraph::from_pyosmgraph(pos.0, pos.1, *zoom);
                let bbox: MapBbox = compute_bounding_box(&lane_graph);
                (lane_graph, bbox)
            }
        }
    }
}

fn lanemap_from_file_map(path: String) -> LaneGraph {
    let path = Path::new(&path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let map: FileMap = serde_json::from_reader(reader).unwrap();

    LaneGraph::new(
        map.nodes.iter().map(|(id, (lon, lat))| {
            let pos = polarfloat_to_cartesiantuple(*lat, *lon);
            (*id, IntersectionData::new(pos.0, pos.1))
        }),
        map.edges
            .iter()
            .map(|(from, to)| (*from, *to, LaneData::new(None, None, None))),
    )
}

///Create the graph that will be display the in visual debugger
fn compute_bounding_box(lanegraph: &LaneGraph) -> MapBbox {
    let positions: Vec<(f64, f64)> = lanegraph
        .intersections
        .values()
        .map(|v| v.position())
        .collect();

    MapBbox {
        x1: positions.iter().map(|v| v.0).fold(std::f64::NAN, f64::min),
        x2: positions.iter().map(|v| v.0).fold(std::f64::NAN, f64::max),
        y1: positions.iter().map(|v| v.1).fold(std::f64::NAN, f64::min),
        y2: positions.iter().map(|v| v.1).fold(std::f64::NAN, f64::max),
    }
}

/// for convenience
fn polarfloat_to_cartesiantuple(lat: f64, lon: f64) -> (f64, f64) {
    let polar = PolarCoord::from_float(lat, lon);
    let cart = CartesianCoord::from_polar(&polar);
    (cart.x.value_unsafe, cart.y.value_unsafe)
}
