extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("stitch")
    .version("1.0")
    .about("Sitches images together.")
    .arg(Arg::with_name("x")
        .help("The X-coordinate.")
        .short("x")
        .case_insensitive(true)
        .default_value("0")
        .takes_value(true)
        .multiple(true)
    )
    .arg(Arg::with_name("y")
        .help("The Y-coordinate.")
        .short("y")
        .case_insensitive(true)
        .default_value("0")
        .takes_value(true)
        .multiple(true)
    )
    .arg(Arg::with_name("output")
        .help("The path to the created image.\nDefaults to a concatenation of the input filenames.")
        .short("o")
        .long("output")
        .takes_value(true)
        .multiple(false)
    )
    .arg(Arg::with_name("IMAGE")
        .help("The path to the image.")
        .multiple(true)
        .min_values(1)
    )
    .get_matches();
    println!("{:?}", matches);
}
