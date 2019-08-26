extern crate clap;
use clap::{App, Arg, ArgMatches, values_t};
use std::path::Path;
use std::collections::{HashMap, VecDeque};

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

impl From<image::ImageError> for StitchError {
    fn from(error: image::ImageError) -> Self {
        StitchError {
            kind: String::from("image"),
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for StitchError {
    fn from(error: std::io::Error) -> Self {
        StitchError {
            kind: String::from("io"),
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
    for image_path in matches.values_of("IMAGE").expect("No images specified.") {
        if Path::new(image_path).exists() {
            image::open(image_path)?;
        } else {
            return Err(StitchError {
                kind: String::from("command-line"),
                message: format!("file does not exist: '{}'", image_path),
            })
        }
    }
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

    #[test]
    fn validate_args_errors_if_image_does_not_exist() {
        let args_vec = vec!["stitch", "-x", "0", "-y", "0", "one.png"];
        let matches = app_args().get_matches_from_safe(args_vec).unwrap();
        let validation_result = validate_args(&matches);
        assert!(validation_result.is_err());
        let error = validation_result.err().unwrap();
        assert_eq!("command-line", error.kind);
        assert_eq!("file does not exist: 'one.png'", error.message);
    }

    #[test]
    fn validate_args_errors_if_image_format_is_not_supported() {
        let mut project_path = std::env::current_dir().expect("current working dir");
        project_path.push("README.md");
        let args_vec = vec!["stitch", "-x", "0", "-y", "0", project_path.to_str().unwrap()];
        let matches = app_args().get_matches_from_safe(args_vec).unwrap();
        let validation_result = validate_args(&matches);
        assert!(validation_result.is_err());
        let error = validation_result.err().unwrap();
        assert_eq!("image", error.kind);
        assert_eq!(r#"The Decoder does not support the image format `Image format image/"md" is not supported.`"#, error.message);
    }

    #[test]
    fn calc_image_size_example() {
        let args_vec = vec!["stitch", "-x", "0", "-y", "0", "one.png", "-x", "256", "-y", "0", "two.png"];
        let matches = app_args().get_matches_from_safe(args_vec).unwrap();
        let x_coords: Vec<u64> = values_t!(matches.values_of("x"), u64).unwrap();
        let y_coords: Vec<u64> = values_t!(matches.values_of("y"), u64).unwrap();
        let coords: Vec<(_, _)> = x_coords.into_iter().zip(y_coords.into_iter()).collect();
        let dimensions = vec![(256u64, 256u64), (256u64, 256u64)];
        let (width, height) = calc_image_size(&dimensions, &coords);
        assert_eq!(512, width);
        assert_eq!(256, height);
    }
}

fn calc_image_size(dimensions: &Vec<(u64, u64)>, coords: &Vec<(u64, u64)>) -> (u64, u64) {
    let mut max_width = 0;
    let mut max_height = 0;
    for (i, dimension) in dimensions.iter().enumerate() {
        let width = dimension.0;
        let height = dimension.1;
        let projected_width = coords[i].0 + width;
        let projected_height = coords[i].1 + height;

        if projected_width > max_width {
            max_width = projected_width;
        }
        if projected_height > max_height {
            max_height = projected_height;
        }
    }
    (max_width, max_height)
}

fn build_output_name(args: &ArgMatches) -> String {
    let mut filename = String::new();
    for (i, input_path) in args.values_of("IMAGE").unwrap().enumerate() {
        if i != 0 {
            filename.push_str("-and-");
        }
        filename.push_str(input_path);
    }
    filename
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
    let default_output = build_output_name(&matches);
    let output_path = Path::new(matches.value_of("output").unwrap_or(&default_output));
    let mut image_map = HashMap::new();
    let mut coords_queue = VecDeque::new();
    let x_coords: Vec<u64> = values_t!(matches.values_of("x"), u64).unwrap();
    let y_coords: Vec<u64> = values_t!(matches.values_of("y"), u64).unwrap();
    let coords: Vec<(_, _)> = x_coords.into_iter().zip(y_coords.into_iter()).collect();
    let mut dimensions: Vec<(u64, u64)> = Vec::new();

    for (i, image_path) in matches.values_of("IMAGE").unwrap().enumerate() {
        // We can safely unwrap here since validate_args checks that this file exists for us.
        let img = image::open(image_path).unwrap().to_rgba();
        let img_dimensions = img.dimensions();
        dimensions.push((img_dimensions.0 as u64, img_dimensions.1 as u64));
        image_map.insert(image_path, img);
        coords_queue.push_back((coords[i].0, coords[i].1));
    }

    let (width, height) = calc_image_size(&dimensions, &coords);
    let mut output_buf: image::RgbaImage = image::ImageBuffer::new(width as u32, height as u32);

    for image_path in matches.values_of("IMAGE").expect("No images specified.") {
        let img = &image_map[image_path];
        let coords = coords_queue.pop_front().unwrap();
        for x in 0..img.width() {
            for y in 0..img.height() {
                output_buf.put_pixel(
                    (coords.0 + x as u64) as u32,
                    (coords.1 + y as u64) as u32,
                    *img.get_pixel(x, y));
            }
        }
    }
    output_buf.save(output_path)?;

    Ok(())
}
