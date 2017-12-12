extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("first cli prog")
        .version("1.0")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("uses a value")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("config") {
        Some(v) => println!("you provided {}", v),
        _ => println!("Nothing provided!"),
    }
}
