use crate::ressources::EndTime;

#[derive(Deserialize)]
pub struct GeneralConfigurations {
    pub end_time: EndTime,
    pub seed: String,
}
