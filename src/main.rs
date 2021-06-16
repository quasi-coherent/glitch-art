use clap::{App, AppSettings};
use image::{DynamicImage, GenericImageView};
use std::path::{Path, PathBuf};

use glitch_art::cmds;
use glitch_art::effects;
use glitch_art::Canvas;

fn main() {
    let matches = App::new("glitch-art")
        .about("A suite of glitch effects to apply to a photo")
        .version("0.1.0")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Daniel Donohue")
        .subcommand(cmds::smear_command())
        .subcommand(cmds::pixelate_command())
        .get_matches();

    match matches.subcommand() {
        ("smear", Some(smear_matches)) => {
            let file = smear_matches.value_of("file").unwrap();
            let mut img = image::open(file).expect("Unable to decode image.");
            let rmin = smear_matches.value_of("row-min");
            let rmax = smear_matches.value_of("row-max");
            let cmin = smear_matches.value_of("col-min");
            let cmax = smear_matches.value_of("col-max");
            let canvas = validate_canvas_args(&img, rmin, rmax, cmin, cmax);
            let smear_down = smear_matches.is_present("smear-down");
            let glitched = effects::smear::build(&mut img, canvas, smear_down);
            let out = out_path(file, "smeared").expect("Invalid input path");
            glitched.save(out).expect("Could not save image")
        }
        ("pixelate", Some(pixelate_matches)) => {
            let file = pixelate_matches.value_of("file").unwrap();
            let mut img = image::open(file).expect("Unable to decode image.");
            let rmin = pixelate_matches.value_of("row-min");
            let rmax = pixelate_matches.value_of("row-max");
            let cmin = pixelate_matches.value_of("col-min");
            let cmax = pixelate_matches.value_of("col-max");
            let canvas = validate_canvas_args(&img, rmin, rmax, cmin, cmax);
            let num_pixels = pixelate_matches.value_of("num-pixels");
            let pixelate = validate_pixelate_args(&canvas, num_pixels);
            let glitched = effects::pixelate::build(&mut img, &pixelate);
            let out = out_path(file, "pixelated").expect("Invalid input path");
            glitched.save(out).expect("Could not save image")
        }
        _ => unreachable!(),
    };
}

/// Form the output path from the input path.
fn out_path(in_path: &str, suffix: &str) -> Option<String> {
    let p = Path::new(in_path);
    let parent = p.parent().unwrap();
    let name = p.file_stem().unwrap();
    let extension = p.extension().unwrap();

    let mut path = PathBuf::new();
    path.push(parent);
    let new_name = format!("{}-{}", name.to_str().unwrap(), suffix);
    path.push(new_name);
    path.set_extension(extension);

    let out = path.to_str().map(|x| x.to_string());
    out
}

/// Parse the command line options to return the area of the photo to apply glitch effects.
fn validate_canvas_args(
    img: &DynamicImage,
    rmin: Option<&str>,
    rmax: Option<&str>,
    cmin: Option<&str>,
    cmax: Option<&str>,
) -> Canvas {
    let (w, h) = img.dimensions();

    let rmin: u32 = rmin.map_or(0, |n| {
        n.parse().expect("row-min must be a nonnegative integer")
    });
    let rmax: u32 = rmax.map_or(h - 1, |n| {
        n.parse().expect("row-max must be a nonnegative integer")
    });
    let cmin: u32 = cmin.map_or(0, |n| {
        n.parse().expect("col-min must be a nonnegative integer")
    });
    let cmax: u32 = cmax.map_or(w - 1, |n| {
        n.parse().expect("col-max must be a nonnegative integer")
    });

    if rmax >= h {
        panic!("row-max cannot be greater than the image height, {}", h)
    };
    if cmax >= w {
        panic!("col-max cannot be greater than the image width, {}", w)
    };

    Canvas {
        row_min: rmin,
        row_max: rmax,
        col_min: cmin,
        col_max: cmax,
    }
}

fn validate_pixelate_args<'a>(
    c: &'a Canvas,
    npixels: Option<&str>,
) -> effects::pixelate::Pixelate<'a> {
    let npixels: u32 = npixels.map_or(1, |n| {
        n.parse()
            .expect("pixel-width must be a nonnegative integer")
    });

    effects::pixelate::Pixelate {
        canvas: c,
        num_pixels: npixels,
    }
}
