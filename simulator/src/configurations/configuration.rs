use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use uuid::Uuid;
use crate::rng::seed;

/// This `struct` should have the same structure as a configs.json files.
#[derive(Deserialize)]
struct Configs {
    seed: String,
}

//Todo: Handle properly if some errors happen.
// Of course, there are going to be more configs.
/// Set the configurations as internal state.
pub fn set_internals_configs() {
    let configs: Configs;
    let mut seed: Uuid = Uuid::new_v4();

    if env::args().nth(1).is_some() {
        configs = fetch_configs_from_json_file().unwrap();
        if !configs.seed.is_empty() {
            seed = Uuid::parse_str(&configs.seed).unwrap();
        }
    }
    seed::M_SEED.lock().unwrap().set(seed);
}

//Todo: Handle properly if can't find path.
/// Create a Configs data from the configurations file.
fn fetch_configs_from_json_file() -> Result<Configs, Box<Error>> {
    let args_path = env::args().nth(1).unwrap();
    let config_path = Path::new(&args_path); //we assume the first argument is always going to be the configuration path.
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let configs = serde_json::from_reader(reader)?;
    Ok(configs)
}
