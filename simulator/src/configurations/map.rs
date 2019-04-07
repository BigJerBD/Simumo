/*! Represent the map on different type. */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use specs::World;

use crate::commons::CartesianCoord;
use crate::commons::Curve;
use crate::commons::Point2D;
use crate::commons::PolarCoord;
use crate::osmgraph_api::OsmGraphApi;
use crate::osmgraph_api::PythonOsmGraphApi;
use crate::ressources::generals;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;

///Define different type of map.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Map {
    PolarFileMap {
        path: String,
    },
    CartFileMap {
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
        let lanegraph = match self {
            Map::PolarFileMap { path } => lanegraph_from_filemap(path.to_string(), &|pt| {
                polarfloat_to_cartesiantuple((pt.1, pt.0))
            }),
            Map::CartFileMap { path } => lanegraph_from_filemap(path.to_string(), &|pt| pt),
            Map::OsmGraph {
                longitude,
                latitude,
                zoom,
            } => lanegraph_from_pyosmgraph(*latitude, *longitude, *zoom),
        };
        create_ressource_lanegraph(lanegraph, world);
    }
}

pub fn lanegraph_from_pyosmgraph(lat: f64, lon: f64, zoom: i64) -> LaneGraph {
    let osmgraph = *PythonOsmGraphApi::query_graph(lon, lat, zoom).unwrap();
    let osm_nodes = osmgraph.get_nodes().unwrap();

    let nodes: Vec<(_, _)> = osm_nodes
        .iter()
        .map(|(id, (lon, lat))| {
            let pos = polarfloat_to_cartesiantuple((*lat, *lon));
            (*id, IntersectionData::new(pos.0, pos.1))
        })
        .collect();

    let edges: Vec<(_, _, _)> = osmgraph
        .get_edges()
        .unwrap()
        .iter()
        .map(|(from, to)| {
            (*from, *to, {
                // TODO: Init actual curves, between intersections
                let (x_from, y_from) = osm_nodes[from];
                let (x_to, y_to) = osm_nodes[to];
                LaneData::new(
                    None,
                    None,
                    Curve::new(vec![Point2D::new(x_from, y_from), Point2D::new(x_to, y_to)]),
                )
            })
        })
        .collect();

    LaneGraph::new(nodes.into_iter(), edges.into_iter())
}

fn lanegraph_from_filemap(path: String, pt_conversion: &Fn((f64, f64)) -> (f64, f64)) -> LaneGraph {
    let path = Path::new(&path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let map: FileMap = serde_json::from_reader(reader).unwrap();

    let nodes : HashMap<_,_> = map.nodes
        .iter()
        .map(|(id, pt)| (*id, pt_conversion(*pt)))
        .collect();

    LaneGraph::new(
        nodes
            .iter()
            .map(|(id, pt)| (*id, IntersectionData::new(pt.0, pt.1))),
        map.edges.iter().map(|(from, to)| {
            (*from, *to, {
                // TODO: Init actual curves, between intersections
                let (x_from, y_from) = nodes[from];
                let (x_to, y_to) = nodes[to];
                LaneData::new(
                    None,
                    None,
                    Curve::new(vec![Point2D::new(x_from, y_from), Point2D::new(x_to, y_to)]),
                )
            })
        }),
    )
}

///Create the graph that will be display the in visual debugger
fn create_ressource_lanegraph(lanegraph: LaneGraph, world: &mut World) {
    let positions: Vec<(f64, f64)> = lanegraph
        .intersections
        .values()
        .map(|v| v.position())
        .collect();

    use std::f64::NAN;
    let bbox = generals::MapBbox {
        x1: positions.iter().map(|v| v.0).fold(NAN, f64::min),
        x2: positions.iter().map(|v| v.0).fold(NAN, f64::max),
        y1: positions.iter().map(|v| v.1).fold(NAN, f64::min),
        y2: positions.iter().map(|v| v.1).fold(NAN, f64::max),
    };
    world.add_resource(bbox);
    world.add_resource(lanegraph);
}

/// for convenience
fn polarfloat_to_cartesiantuple((lat, lon): (f64, f64)) -> (f64, f64) {
    let polar = PolarCoord::from_float(lat, lon);
    let cart = CartesianCoord::from_polar(&polar);
    (cart.x.value_unsafe, cart.y.value_unsafe)
}
