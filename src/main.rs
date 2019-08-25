extern crate clap;
use clap::{App, Arg, ArgMatches, values_t};
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use image::GenericImageView;

type Result<T> = std::result::Result<T, StitchError>;

#[derive(Debug, Clone)]
struct StitchError {
    kind: String,
    message: String,
}

impl std::fmt::Display for StitchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stitch generic error")
    }
}

// This is important for other errors to wrap this one.
impl std::error::Error for StitchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<clap::Error> for StitchError {
    fn from(error: clap::Error) -> Self {
        StitchError {
            kind: String::from("clap"),
            message: error.to_string(),
        }
    }
}

fn validate_args<'a>(matches: &'a ArgMatches) -> Result<&'a ArgMatches<'a>> {
    let image_count = matches.occurrences_of("IMAGE");
    if image_count != matches.occurrences_of("x") {
        return Err(StitchError {
            kind: String::from("command-line"),
            message: format!("-x specified {} times, expected {}", matches.occurrences_of("x"), image_count),
        })
    }
    values_t!(matches.values_of("x"), u64)?;
    if image_count != matches.occurrences_of("y") {
        return Err(StitchError {
            kind: String::from("command-line"),
            message: format!("-y specified {} times, expected {}", matches.occurrences_of("y"), image_count),
        })
    }
    values_t!(matches.values_of("y"), u64)?;
    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_coordinate_must_be_integer() {
        let args_vec = vec!["stitch", "-x", "e", "-y", "0", "one.png"];
        let matches = app_args().get_matches_from_safe(args_vec).unwrap();
        let validation_result = validate_args(&matches);
        assert!(validation_result.is_err());
        let error = validation_result.err().unwrap();
        assert_eq!("clap", error.kind);
        assert_eq!("error: Invalid value: The argument 'e' isn't a valid value\n", error.message);
    }

    #[test]
    fn y_coordinate_must_be_integer() {
        let args_vec = vec!["stitch", "-x", "0", "-y", "u", "one.png"];
        let matches = app_args().get_matches_from_safe(args_vec).unwrap();
        let validation_result = validate_args(&matches);
        assert!(validation_result.is_err());
        let error = validation_result.err().unwrap();
        assert_eq!("clap", error.kind);
        assert_eq!("error: Invalid value: The argument 'u' isn't a valid value\n", error.message);
    }

    #[test]
    fn x_coordinate_for_every_image() {
        let args_vec = vec!["stitch", "-y", "0", "one.png", "-x", "256", "-y", "256", "two.png"];
        let matches = app_args().get_matches_from_safe(args_vec).unwrap();
        let validation_result = validate_args(&matches);
        assert!(validation_result.is_err());
        let error = validation_result.err().unwrap();
        assert_eq!("command-line", error.kind);
        assert_eq!("-x specified 1 times, expected 2", error.message);
    }

    #[test]
    fn y_coordinate_for_every_image() {
        let args_vec = vec!["stitch", "-x", "0", "one.png", "-x", "256", "-y", "256", "two.png"];
        let matches = app_args().get_matches_from_safe(args_vec).unwrap();
        let validation_result = validate_args(&matches);
        assert!(validation_result.is_err());
        let error = validation_result.err().unwrap();
        assert_eq!("command-line", error.kind);
        assert_eq!("-y specified 1 times, expected 2", error.message);
    }
}

fn app_args<'a>() -> App<'a, 'a> {
    App::new("stitch")
        .version("1.0")
        .about("Sitches images together.")
        .arg(Arg::with_name("x")
            .help("The X-coordinate.")
            .short("x")
            .case_insensitive(true)
            .default_value("0")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1))
        .arg(Arg::with_name("y")
            .help("The Y-coordinate.")
            .short("y")
            .case_insensitive(true)
            .default_value("0")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1))
        .arg(Arg::with_name("output")
            .help("The path to the created image.\nDefaults to a concatenation of the input filenames.")
            .short("o")
            .long("output")
            .takes_value(true)
            .multiple(false))
        .arg(Arg::with_name("IMAGE")
            .help("The path to the image.")
            .multiple(true)
            .min_values(1))
}

fn main() -> Result<()> {
    let matches = app_args().get_matches();
    let matches = validate_args(&matches)?;
    let output_path = Path::new(r"output.png");
    let output_file = File::create(output_path).unwrap();
    let ref mut w = BufWriter::new(output_file);

    let mut max_width = 0;
    let mut max_height = 0;

    for image in matches.values_of("IMAGE").expect("No images specified.") {
        println!("{}", image);
        let img = image::open(image).unwrap();
        let (width, height) = img.dimensions();
        println!("\t{:?}", img.dimensions());

        if width > max_width {
            max_width = width;
        }
        if height > max_height {
            max_height = height;
        }
    }
    Ok(())
}
