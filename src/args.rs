use clap::{App, Arg};

pub fn app<'x>() -> App<'x> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Simple folder size calculator")
        .arg(
            Arg::new("unit")
                .long("unit")
                .short('u')
                .value_name("UNIT")
                .about("The unit of file size to output")
                .takes_value(true)
                .possible_values(&["b", "kb", "mb", "gb", "tb", "pb", "auto"])
        ).arg(
        Arg::new("path")
            .index(1)
            .takes_value(true)
            .value_name("PATH")
            .about("The path to the directory to scan")
            .default_missing_value(".")
    ).arg(
        Arg::new("depth")
            .long("depth")
            .short('d')
            .value_name("DEPTH")
            .about("The recessive depth to scan")
            .takes_value(true)
            .default_value("1")
    ).arg(
        Arg::new("files")
            .long("files")
            .short('f')
            .about("Include the size of files within the output")
    ).arg(
        Arg::new("empty")
            .long("empty")
            .short('e')
            .about("Ignore empty directories / files")
    ).arg(
        Arg::new("places")
            .long("round")
            .short('r')
            .value_name("ROUND")
            .about("The number of figures to round too.")
            .takes_value(true)
            .default_value("0")
            .allow_hyphen_values(true)
    ).arg(
        Arg::new("tree")
            .long("tree")
            .short('t')
            .about("Whether the search should show all results.")
    ).arg(
        Arg::new("sort")
            .long("sort")
            .short('s')
            .about("How the results should be sorted")
            .takes_value(true)
            .possible_values(&["size", "name", "created", "modified"])
    ).arg(
        Arg::new("invert")
            .short('i')
            .long("invert")
            .about("Invert sorted results")
    )
}