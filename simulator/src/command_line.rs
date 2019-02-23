use argparse::{ArgumentParser, Store, StoreTrue};

#[derive(Default)]
pub struct CommandLineArguments {
    pub configuration_path: String, // change this for Path data. See Configurations as example
    pub logger_path: String,
    pub verbose: bool,
}

impl CommandLineArguments {
    pub fn parse() -> Self {
        let mut options = Self::default();
        {
            // this block limits scope of borrows by ap.refer() method
            let mut parser = ArgumentParser::new();
            parser.set_description("Command-line options");
            parser
                .refer(&mut options.configuration_path)
                .add_option(&["-c"], Store, "Json configuration file path")
                .required();

            parser.refer(&mut options.logger_path).add_option(
            &["-l"],
            Store,
            "Logger files output. The default path is: TODO:add default path same as LOG_DIRECTORY in src/general.rs",
        );

            // Todo: When ready, use this code beceause we want log file path to be specify.
            /*parser.refer(&mut options.logger_path).add_option(
                &["-l"],
                Store,
                "Logger files output. The default path is: TODO:add default path same as LOG_DIRECTORY in src/general.rs",
            ).required();*/

            parser.refer(&mut options.verbose).add_option(
                &["-v"],
                StoreTrue,
                "Show details about arguments execution and probably other things.",
            );
            parser.parse_args_or_exit();
        }
        options
    }
}
