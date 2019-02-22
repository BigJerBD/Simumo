use argparse::{ArgumentParser, Store, StoreTrue};
// use std::path::Path;

use crate::configuration;
use crate::ressources::generals;

#[derive(Default)]
pub struct CommandLineArguments {
    pub configuration_path: String, // change this for Path data. See Configurations as example
    pub logger_path: String,
    pub python_script_path: String,
    pub verbose: bool,
}

pub fn execute_arguments() {
    let options = parse_arguments();
    configuration::set_internals_configs(&options);

    if !options.logger_path.is_empty() {
        // Todo: check if path is correct.
        // let logger_path = Path::new(&options.logger_path);
        let logger_path = options.logger_path;
        generals::M_LOG_DIRECTORY.lock().unwrap().replace(logger_path);
    }

}

fn parse_arguments() -> CommandLineArguments {
    let mut options: CommandLineArguments = Default::default();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut parser = ArgumentParser::new();
        parser.set_description("Command-line options");
        parser.refer(&mut options.configuration_path).add_option(
            &["-c"],
            Store,
            "Json configuration file path",
        );

        parser.refer(&mut options.logger_path).add_option(
            &["-l"],
            Store,
            "Logger files output. The default path is: TODO:add default path same as LOG_DIRECTORY in src/general.rs",
        );

        /*parser.refer(&mut options.logger_path).add_option(
            &["-l"],
            Store,
            "Logger files output. The default path is: TODO:add default path same as LOG_DIRECTORY in src/general.rs",
        ).required();*/ // Todo: When ready, use this code beceause we want log file path to be specify.

        parser.refer(&mut options.verbose).add_option(
            &["-v"],
            StoreTrue,
            "Show details about arguments execution and probably other things.",
        );
        parser.parse_args_or_exit();
    }
    options
}
