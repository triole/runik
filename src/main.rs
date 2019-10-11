use std::process::exit;
#[macro_use]
extern crate clap;

mod config;
mod processor;

fn main() {
    let yaml = load_yaml!("../.argsprod.yaml").to_owned();
    let args = olib_argparse::Argparse::init(yaml);
    let config = config::Config::init(&args);

    let p = processor::Processor::init(config);

    if let Err(err) = p.run() {
        eprintln!("{}", err);
        exit(1);
    }
}
