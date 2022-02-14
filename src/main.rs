use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // The path to the must-gather.
    path: String,
}

fn main() {
    let cli = Cli::parse();

    println!("path: {:?}", cli.path);
}
