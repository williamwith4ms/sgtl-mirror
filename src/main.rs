use clap::{Parser, ValueEnum};
mod methods;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// verbose mode
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    decode: bool,

    #[arg(value_enum)]
    method: Method,

    #[arg(short = 'f', long)]
    input_file: Option<String>,

    #[arg(short = 'o', long)]
    output_file: Option<String>,

    data: Option<String>,

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Method {
    Echo,
    Rot26,
    Base64,
    Sha256,
    Sha512,
    Sha384,
    Sha224,
    Sha512_256,
}

fn main() {
    let args: Args = Args::parse();

    let data: String = if args.data.is_none() && args.input_file.is_none() {
        eprintln!("Error: Either data or input_file must be provided.");
        std::process::exit(1);
    } else if args.data.is_some() && args.input_file.is_some() {
        eprintln!("Error: Provide either data or input_file, not both.");
        std::process::exit(1); 
    } else if args.data.is_none() && args.input_file.is_some() {
        // read from file
        let file_path: &str = args.input_file.as_ref().unwrap();
        std::fs::read_to_string(file_path).expect("Failed to read input file")
    } else {
        args.data.unwrap()
    };

    if args.verbose {
        println!("Method: {:?}", args.method);
        println!("input_file: {:?}", args.input_file);
        println!("Decode: {}", args.decode);
        println!("Data: {}", data);
    }

    let output: String = match args.method {
        Method::Echo | Method::Rot26 => {
            methods::echo::echo(&data).to_string()
        },
        Method::Base64 => {
            if args.decode {
                match methods::base64::base64_decode(&data) {
                    Ok(decoded) => decoded,
                    Err(e) => {eprintln!("base64 error: {}", e); std::process::exit(1);},
                }
            } else {
                methods::base64::base64_encode(&data)
            }
        },
        Method::Sha256 => methods::sha2::sha256_hash(&data),
        Method::Sha512 => methods::sha2::sha512_hash(&data),
        Method::Sha384 => methods::sha2::sha384_hash(&data),
        Method::Sha224 => methods::sha2::sha224_hash(&data),
        Method::Sha512_256 => methods::sha2::sha512_256_hash(&data),
    };

    if let Some(output_file) = args.output_file {
        std::fs::write(output_file, &output).expect("Failed to write to output file");
    } else {
        println!("{}", output);
    }


}

