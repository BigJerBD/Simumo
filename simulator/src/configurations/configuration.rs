use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::generals::GeneralConfigurations;
use super::map::MapConfiguration;
use super::systems::SystemsConfiguration;

/// Represent the root level configuration.
///
/// Todo: Can't handle empty field in serialization.
#[derive(Deserialize)]
pub struct Configuration {
    pub generals: GeneralConfigurations,
    pub map: MapConfiguration,
    pub systems: SystemsConfiguration,
}

impl Configuration {
    pub fn from_path(args_path: &str) -> Result<Self, Box<Error>> {
        let config_path = Path::new(&args_path);
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::rng::seed;

//     use super::*;

//     #[test]
//     #[ignore] // Todo: unignore when lazy_static no more. Baby don't hurt me no more...
//     fn null_seed_is_set() {
//         let _config = Configuration {
//             generals: Generals {
//                 log_directory: "".to_string(),
//                 seed: "".to_string(),
//             },
//             systems: SystemsConfiguration::default(),
//         };
//         assert!(seed::SEED.is_nil());
//     }

//     #[test]
//     fn setted_seed_is_set() {
//         let config = Configuration {
//             generals: GeneralConfigurations {
//                 log_directory: "".to_string(),
//                 seed: "2d524fe8-55f2-4406-bbf2-8b6568871aa2".to_string(),
//             },
//             systems: SystemsConfiguration::default(),
//         };
//         config.setup();
//         assert_eq!(seed::SEED.to_string(), config.generals.seed);
//     }
// }
