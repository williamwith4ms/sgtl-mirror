use clap::{Parser, builder::Str};
mod methods;
use methods::*;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// verbose mode
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    decode: bool,

    method: String,

    data: String,

}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Method: {}", args.method);
        println!("Data: {}", args.data);
        println!("Decode: {}", args.decode);
    }

    match args.method.as_str() {
        "echo" => {
            let result = methods::echo::echo(&args.data);
            println!("{}", result);
        },
        _ => {
            eprintln!("Unknown method: {}", args.method);
        }
    }
}

