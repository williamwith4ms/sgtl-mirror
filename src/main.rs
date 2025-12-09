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
    let args: Args = Args::parse();

    if args.verbose {
        println!("Method: {}", args.method);
        println!("Data: {}", args.data);
        println!("Decode: {}", args.decode);
    }

    match args.method.as_str() {
        "echo" => {
            let result: &str = methods::echo::echo(&args.data);
            println!("{}", result);
        },
        "base64" => {
            if args.decode {
                match methods::base64::base64_decode(&args.data) {
                    Ok(decoded) => println!("{}", decoded),
                    Err(e) => eprintln!("base64 error: {}", e),
                }
            } else {
                let encoded: String = methods::base64::base64_encode(&args.data);
                println!("{}", encoded);
            }
        },
        _ => {
            eprintln!("Unknown method: {}", args.method);
        }
    }
}

