#[derive(Deserialize)]
pub struct MapConfiguration {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: i64
}


#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Map {
    OsmGraph(MapConfiguration),
}
