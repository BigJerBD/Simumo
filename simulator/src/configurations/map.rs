#[derive(Deserialize)]
pub struct OsmConfiguration {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: i64
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Map {
    OsmGraph(OsmConfiguration),
}
