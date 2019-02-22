use argparse::{ArgumentParser, Store};

#[derive(Default)]
pub struct Options {
    configuration_path: String, // change this for Path data. See Configurations as example
    logger_path: String,
    python_script_path: String,
}

pub fn parse_arguments() -> Options {
    let mut options: Options = Default::default();
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
            "Output path of logger files. The default path is: TODO:add default path same as LOG_DIRECTORY in src/general.rs. ",
        );

        parser.refer(&mut options.python_script_path).add_option(
            &["-p"],
            Store,
            "Python script path to be execute",
        );
        parser.parse_args_or_exit();
    }

    options
}
