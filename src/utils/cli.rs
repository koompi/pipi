use clap::{App, Arg};

// This function does:
// Declare argument list and specs
// Return a matched arguement

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn cmd_args<'a, 'b>() -> App<'a, 'b> {
    App::new("pipi")
        .version(VERSION)
        .author(AUTHOR)
        .arg(
            Arg::with_name("add")
                .help("Add package")
                .short("a")
                .long("add")
                .takes_value(true)
                .multiple(true),
        )
        // .arg(
        //     Arg::with_name("update")
        //         .help("Update applications")
        //         .short("u")
        //         .long("update"),
        // )
        .arg(
            Arg::with_name("remove")
                .help("Remove package")
                .short("r")
                .long("remove")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("build")
                .help("Build rootfs")
                .short("b")
                .long("build")
                // .takes_value(true)
                // .multiple(true),
        )
    .arg(
        Arg::with_name("update")
            .help("update the db")
            .short("u")
            .long("update")
            // .takes_value(true)
            // .multiple(true),
    )
    .arg(
        Arg::with_name("create")
            .help("Create database")
            .short("c")
            .long("create")
            .takes_value(true)
            .multiple(true),
    )
    .arg(
        Arg::with_name("new")
            .help("create working directory")
            .short("n")
            .long("new")
            .takes_value(true)
            .multiple(true),
    )
    .arg(
        Arg::with_name("iso")
            .help("Generate KOOMPI's ISO")
            .short("i")
            .long("iso")
            // .takes_value(true)
            // .multiple(true),
    )
}
