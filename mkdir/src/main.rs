extern crate clap;
use clap::{App, Arg};

fn main() {
    let mut exitval: i32 = 0;

    let args = App::new("mkdir")
        .version("0.1.0")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .takes_value(true)
                .help("Set the file permission bits of the newly-created directory to the specified mode value"),
        )
        .arg(
            Arg::with_name("do_path")
                .short("p")
                .long("path")
                .help("Create any missing intermediate pathname components"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increase output verbosity"),
        )
        .arg(
            Arg::with_name("dir")
                .required(true)
                .index(1)
                .multiple(true)
                .help("Directories to be created"),
        )
        .get_matches();

    let dirs: Vec<&str> = args.values_of("dir").unwrap().collect();
    for dir in dirs {
        match mkdir(dir, args.is_present("do_path")) {
            Err(e) => {
                println!("error: {}: {}", dir, e);
                exitval += 1;
            }
            Ok(_) => {
                if args.is_present("verbose") {
                    println!("{}", dir);
                }
            },
        };
    }

    std::process::exit(exitval);
}

fn mkdir(dir: &str, do_path: bool) -> std::io::Result<()> {
    if do_path {
        return std::fs::create_dir_all(dir);
    }
    std::fs::create_dir(dir)
}
