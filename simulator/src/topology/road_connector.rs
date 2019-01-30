use super::RoadConnectorId;

#[derive(Default, Debug)]
pub struct RoadConnector {
    pub id: RoadConnectorId,
}

impl RoadConnector {
    pub fn new() -> Self {
        RoadConnector {
            id: RoadConnectorId::new(0),
        }
    }
}
