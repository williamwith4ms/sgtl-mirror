use clap::Parser;
mod methods;

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
        "echo" | "rot26" => {
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
        "sha256" => {
            let hash: String = methods::sha2::sha256_hash(&args.data);
            println!("{}", hash);
        },
        "sha512" => {
            let hash: String = methods::sha2::sha512_hash(&args.data);
            println!("{}", hash);
        },
        "sha384" => {
            let hash: String = methods::sha2::sha384_hash(&args.data);
            println!("{}", hash);
        },
        "sha224" => {
            let hash: String = methods::sha2::sha224_hash(&args.data);
            println!("{}", hash);
        },
        "sha512_256" => {
            let hash: String = methods::sha2::sha512_256_hash(&args.data);
            println!("{}", hash);
        },
        _ => {
            eprintln!("Unknown method: {}", args.method);
        }
    }
}

