use clap::Parser;
use gib_interface::run;

/// Simple program to get a solidity interface from a solidity contract
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Contract to make interface for
    #[arg(short, long)]
    contract_file: String,

    /// Functions you want to be in the interface
    #[arg(short, long)]
    function_names: Vec<String>,
}

fn main() {
    let args = Args::parse();
    run(args.contract_file, args.function_names);
}
