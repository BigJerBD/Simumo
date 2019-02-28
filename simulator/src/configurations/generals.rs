use crate::ressources::generals;
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

        if !self.log_directory.is_empty() {
            let log_directory = self.log_directory.to_string();
            generals::M_LOG_DIRECTORY
                .lock()
                .unwrap()
                .replace(log_directory);
        } else {
            panic!("Json configuration file must have a log directory path in generals attributs.")
        }
    }
}
