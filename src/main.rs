extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("tsin")
        .author("Micheal Li")
        .about("Picture service agent")
        .subcommand(SubCommand::with_name("serve")
            .about("service agent")
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("serve") => {
            println!("Hi, Tsin!");
        },
        _ => {},
    }
}
