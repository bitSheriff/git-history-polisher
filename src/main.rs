mod gitter;
mod datter;
mod systemer;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "git-history-polisher")]
#[clap(author = "bitSheriff <bitSheriff@protonmail.com>")]
#[clap(version = "0.1")]
#[clap(about = "Polishes your git history for GitHub", long_about = None)]
struct Args {
   /// path to the git repository
   #[clap(short = 'p', long="path", value_parser)]
   path: String,

   /// Start date
   #[clap(short = 's', long="start", group = "date", value_parser)]
   start_date: String,
    
   /// End date
   #[clap(short = 'e', long="end",group = "date", value_parser)]
   end_date: String,
    
   /// Commits only on workdays
   #[clap(short = 'w', long = "only-workdays", value_parser, default_value_t = false)]
   workdays: bool,
    
   /// Commits per day
   #[clap(short = 'c', long = "commits-per-day", value_parser, default_value_t = 1.0)]
   count: f32,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world! {:#?}", args);

    if args.workdays == true
    {
        println!("Sorry not implemented yet")
    }
}
