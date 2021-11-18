use std::error::Error;
use clap::{Arg, App};
use simple_error::SimpleError;

pub struct Config {
    pub database_filename: String
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let matches = App::new("KeePass PinePhone")
            .version("0.0")
            .author("Julien de Charentenay <julien@charentenay.me>")
            .about("PinePhone app for KeePass")
            .arg(Arg::with_name("database_filename")
                 .long("database")
                 .help("Specify the filepath to the keepass file")
                 .value_name("PATH/FILE")
                 .takes_value(true))
            .get_matches();

        let database_filename = matches.value_of("database_filename");

        if database_filename.is_none() {
            return Err(Box::new(SimpleError::new("Database filename is missing")));
        }

        Ok( Config { database_filename: database_filename.unwrap().to_string() } )
    }
}
