mod generator;
mod parse_dependence;

use clap::Parser;
use crate::generator::Generator;

fn main() {
    let config = generator::Configuration::parse();
    Generator::new(config).build();
    Generator::run();
}