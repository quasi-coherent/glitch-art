use clap::{App, Arg, SubCommand};

/// The smear subcommand.
pub fn smear_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("smear")
        .about("Smear a row of pixels through a region of the image")
        .arg(
            Arg::with_name("file")
                .long("file")
                .value_name("FILE")
                .help("Path to the file")
                .required(true),
        )
        .arg(
            Arg::with_name("row-min")
                .long("row-min")
                .value_name("INT")
                .help("Row to start smearing"),
        )
        .arg(
            Arg::with_name("row-max")
                .long("row-max")
                .value_name("INT")
                .help("Row to end smearing"),
        )
        .arg(
            Arg::with_name("col-min")
                .long("col-min")
                .value_name("INT")
                .help("Column to start smearing"),
        )
        .arg(
            Arg::with_name("col-max")
                .long("col-max")
                .value_name("INT")
                .help("Column to end smearing"),
        )
        .arg(
            Arg::with_name("smear-down")
                .long("smear-down")
                .takes_value(false)
                .help("Take the minimum row to draw the canvas with"),
        )
}

/// The pixelate subcommand.
pub fn pixelate_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("pixelate")
        .about("Pixelate a region of the image")
        .arg(
            Arg::with_name("file")
                .long("file")
                .value_name("FILE")
                .help("Path to the file")
                .required(true),
        )
        .arg(
            Arg::with_name("row-min")
                .long("row-min")
                .value_name("INT")
                .help("Row to start pixelation"),
        )
        .arg(
            Arg::with_name("row-max")
                .long("row-max")
                .value_name("INT")
                .help("Row to end pixelation"),
        )
        .arg(
            Arg::with_name("col-min")
                .long("col-min")
                .value_name("INT")
                .help("Column to start pixelation"),
        )
        .arg(
            Arg::with_name("col-max")
                .long("col-max")
                .value_name("INT")
                .help("Column to end pixelation"),
        )
        .arg(
            Arg::with_name("num-pixels")
                .long("num-pixels")
                .value_name("INT")
                .help("How many pixels should appear in the final canvas"),
        )
}
