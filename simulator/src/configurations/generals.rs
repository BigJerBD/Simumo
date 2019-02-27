use crate::rng::seed;
use uuid::Uuid;

/// Represent the general configuration.
#[derive(Deserialize, Default)]
pub struct GeneralConfigurations {
    pub log_directory: String,
    pub seed: String,
}

impl GeneralConfigurations {
    pub fn setup(&self) {
        if !self.seed.is_empty() {
            let uuid = Uuid::parse_str(&self.seed).unwrap();
            seed::M_SEED.lock().unwrap().set(uuid);
        } else {
            seed::M_SEED.lock().unwrap().set(Uuid::new_v4());
        }
    }
}
