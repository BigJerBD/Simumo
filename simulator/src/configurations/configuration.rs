use crate::command_line::arguments;
use crate::rng::seed;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use uuid::Uuid;

/// This `struct` should have the same structure as a configs.json files.
#[derive(Deserialize)]
struct Configs {
    seed: String, // Todo: add other type of json config file.
}

//Todo: Handle properly if some errors happen.
// Of course, there are going to be more configs.
/// Set the configurations as internal state.
pub fn set_internals_configs(options: &arguments::CommandLineArguments) {
    let configs: Configs;
    let mut seed: Uuid = Uuid::new_v4();

    if !options.configuration_path.is_empty() {
        configs = fetch_configs_from_json_file(&options.configuration_path).unwrap();
        if !configs.seed.is_empty() {
            seed = Uuid::parse_str(&configs.seed).unwrap();
        }
    }
    seed::M_SEED.lock().unwrap().set(seed);

    if options.verbose
    //Todo: Change println to write in log file.
    {
        println!("Seed use: {}", seed); // One day, we will preint all configs.
    }
}

//Todo: Handle error properly if can't find path.
/// Create a Configs data from the configurations file.
fn fetch_configs_from_json_file(args_path: &String) -> Result<Configs, Box<Error>> {
    let config_path = Path::new(&args_path);
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let configs = serde_json::from_reader(reader)?;
    Ok(configs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_seed_from_mutex() {
        let mut command_line_arguments: arguments::CommandLineArguments = Default::default();
        set_internals_configs(&command_line_arguments);

        assert!(!seed::SEED.is_nil());
    }
}
