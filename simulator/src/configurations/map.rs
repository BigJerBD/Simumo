/*! Represent the map on different type. */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use specs::World;

use crate::commons::CartesianCoord;
use crate::commons::PolarCoord;
use crate::ressources::generals;
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
    ///Create world ressources depending on type.
    pub fn forward_ressources(&self, world: &mut World) {
        match self {
            Map::FileMap { path } => {
                create_ressource_lanegraph(lanemap_from_file_map(path.to_string()), world)
            }
            Map::OsmGraph {
                longitude,
                latitude,
                zoom,
            } => {
                let pos = polarfloat_to_cartesiantuple(*latitude, *longitude);
                create_ressource_lanegraph(LaneGraph::from_pyosmgraph(pos.0, pos.1, *zoom), world)
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
fn create_ressource_lanegraph(lanegraph: LaneGraph, world: &mut World) {
    let positions: Vec<(f64, f64)> = lanegraph
        .intersections
        .values()
        .map(|v| v.position())
        .collect();

    let bbox = generals::MapBbox {
        x1: positions.iter().map(|v| v.0).fold(std::f64::NAN, f64::min),
        x2: positions.iter().map(|v| v.0).fold(std::f64::NAN, f64::max),
        y1: positions.iter().map(|v| v.1).fold(std::f64::NAN, f64::min),
        y2: positions.iter().map(|v| v.1).fold(std::f64::NAN, f64::max),
    };
    world.add_resource(bbox);
    world.add_resource(lanegraph);
}

/// for convenience
fn polarfloat_to_cartesiantuple(lat: f64, lon: f64) -> (f64, f64) {
    let polar = PolarCoord::from_float(lat, lon);
    let cart = CartesianCoord::from_polar(&polar);
    (cart.x.value_unsafe, cart.y.value_unsafe)
}
