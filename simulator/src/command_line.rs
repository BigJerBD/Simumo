use argparse::{ArgumentParser, Store, StoreTrue};

#[derive(Default)]
pub struct CommandLineArguments {
    pub configuration_path: String, // change this for Path data. See Configurations as example
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

            parser.parse_args_or_exit();
        }
        options
    }
}
