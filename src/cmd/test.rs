use crate::{get_book_dir, get_build_opts};
use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::errors::Result;
use mdbook::MDBook;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("test")
        .about("Tests that a book's Rust code samples compile")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'Output directory for the book{n}\
             Relative paths are interpreted relative to the book's root directory.{n}\
             If omitted, mdBook uses build.build-dir from book.toml or defaults to `./book`.'",
        )
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg(Arg::with_name("library-path")
            .short("L")
            .long("library-path")
            .value_name("dir")
            .takes_value(true)
            .require_delimiter(true)
            .multiple(true)
            .empty_values(false)
            .help("A comma-separated list of directories to add to {n}the crate search path when building tests"))
        .arg_from_usage("-l, --language=[language] 'Language to render the compiled book in.{n}\
                         Only valid if the [languages] table in the config is not empty.{n}\
                         If omitted, defaults to the language with `default` set to true.'")
}

// test command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let library_paths: Vec<&str> = args
        .values_of("library-path")
        .map(std::iter::Iterator::collect)
        .unwrap_or_default();
    let book_dir = get_book_dir(args);
    let build_opts = get_build_opts(args);
    let mut book = MDBook::load_with_build_opts(&book_dir, build_opts)?;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.test(library_paths)?;

    Ok(())
}
