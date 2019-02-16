use specs::prelude::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use uuid::Uuid;

/// This `struct` should have the same structure as a configs.json files.
#[derive(Deserialize)]
struct Configs {
    seed: String,
}

//Todo: Handle properly if some errors happen.
//Todo: seed should be set with laze_static. BigJ is working on that actually.
// Of course, there are going to be more configs.
/// Set the configurations from configurations file as ressources.
pub fn set_configs(world: &mut World) {
    let configs: Configs;
    let mut seed: Uuid = Uuid::new_v4();

    if env::args().nth(1).is_some() {
        configs = read_configs_from_json_file().unwrap();
        if !configs.seed.is_empty() {
            seed = Uuid::parse_str(&configs.seed).unwrap();
        }
    }
    world.add_resource(seed);
}

//Todo: Handle properly if can't find path.
/// Create a Configs from the configurations file.
fn read_configs_from_json_file() -> Result<Configs, Box<Error>> {
    let args_path = env::args().nth(1).unwrap();
    let config_path = Path::new(&args_path); //we assume the first argument is always going to be the configuration path.
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let configs = serde_json::from_reader(reader)?;
    Ok(configs)
}
