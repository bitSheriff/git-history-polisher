mod gitter;
mod datter;
mod systemer;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// path to the git repository
   #[clap(short = 'p', long="path", value_parser)]
   path: String,

   /// Start date
   #[clap(short = 's', long="start", value_parser)]
   start_date: String,
    
   /// End date
   #[clap(short = 'e', long="end", value_parser)]
   end_date: String,
    
   /// Commits per day
   #[clap(short, long, value_parser, default_value_t = 1.0)]
   count: f32,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world! {:#?}", args);
}
