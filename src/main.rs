mod gitter;
mod datter;
mod systemer;

use clap::Parser;
use datter::Datter;
use gitter::Gitter;

#[derive(Parser, Debug)]
#[clap(name = "git-history-polisher")]
#[clap(author = "bitSheriff <bitSheriff@protonmail.com>")]
#[clap(version = "0.1")]
#[clap(about = "Polishes your git history for GitHub", long_about = None)]
struct Args {
    /// path to the git repository
    #[clap(short = 'p', long = "path", value_parser)]
    path: String,

    /// Start date
    #[clap(short = 's', long = "start", value_parser)]
    start_date: String,

    /// End date
    #[clap(short = 'e', long = "end", value_parser)]
    end_date: String,

    /// Commits only on workdays
    #[clap(
        short = 'w',
        long = "only-workdays",
        value_parser,
        default_value_t = false
    )]
    workdays: bool,

    /// Commits per day
    #[clap(
        short = 'c',
        long = "commits-per-day",
        value_parser,
        default_value_t = 1.0
    )]
    count: f32,

    /// Min Commits per day
    #[clap(short = 'm', long = "min", value_parser, default_value_t = 0)]
    min_com: u16,

    /// Max Commits per day
    #[clap(short = 'M', long = "max", value_parser, default_value_t = 0)]
    max_com: u16,

    /// File to be changed
    #[clap(short = 'f', long = "file", value_parser, default_value_t = String::from("foo.txt"))]
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("Arguments parsed: {:#?}", args);

    let mut date_module = Datter::new(args.start_date, args.end_date, args.workdays, args.count);
    let mut git_module = Gitter::new(args.path, args.file, args.min_com, args.max_com);

    println!("Datter: {:#?}", date_module);
    println!("Gitter: {:#?}", git_module);
    // Iterate through the wanted days
    loop {
        match date_module.get_next_date() {
            Err(result_date) => println!("has finished"),
            Ok(result_date) => {
                // println!("Iterated Date: {:#?}", result_date);
                // commit on the iterated date
                git_module.commit_on_date(&result_date);
            }
        }

        // loop termination
        if date_module.get_finished() == true
        {
            git_module.clean_up();
            break;
        }
    }

}
