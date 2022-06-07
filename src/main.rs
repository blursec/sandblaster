use clap::{crate_authors, crate_name, crate_version, App, Arg};

#[derive(Debug)]
struct Config {
    url: String,
    filename: String,
}

fn config_from_args() -> Config {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Todo : about section")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help("todo set url")
            .takes_value(true))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("todo set url")
            .takes_value(true))
        .get_matches();
    Config {
        url: matches.value_of("url").unwrap().to_string(),
        filename: matches.value_of("file").unwrap().to_string() }
}

fn main() {
    let config = config_from_args();
    println!("{:?}", config);
}