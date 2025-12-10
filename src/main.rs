use clap::{Parser, Subcommand};
mod methods;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// verbose mode
    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(short, long, global = true)]
    decode: bool,

    #[arg(short = 'i', long = "input", global = true)]
    input_file: Option<String>,

    #[arg(short = 'o', long = "output", global = true)]
    output_file: Option<String>,

    /// subcommand to choose method
    #[command(subcommand)]
    method: Method,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Method {
    #[command(
        name = "echo", 
        about = "Returns the input unchanged", 
        long_about = None, 
        after_help = "Example: sgtl echo -d 'Hello, world!'")]
    Echo {
        data: Option<String>,
    },
    #[command(
        name = "rot26", 
        about = "Applies ROT26 cipher (no change)", 
        long_about = "Applies ROT26 cipher (no change), this is actually just an alias for echo.", 
        after_help = "Example: sgtl rot26 -d 'Hello, world!'")]
    Rot26 {
        data: Option<String>,
    },
    #[command(
        name = "base64", 
        about = "Encodes/Decodes Base64", 
        long_about = None, 
        after_help = "Example: sgtl base64 -d 'Hello, world!'")]
    Base64 {
        data: Option<String>,
    },
    #[command(
        name = "sha256", 
        about = "Computes SHA-256 hash", 
        long_about = None, 
        after_help = "Example: sgtl sha256 -d 'Hello, world!'")]
    Sha256 {
        data: Option<String>,
    },
    #[command(
        name = "sha512", 
        about = "Computes SHA-512 hash", 
        long_about = None, 
        after_help = "Example: sgtl sha512 -d 'Hello, world!'")]
    Sha512 {
        data: Option<String>,
    },
    #[command(
        name = "sha384", 
        about = "Computes SHA-384 hash", 
        long_about = None, 
        after_help = "Example: sgtl sha384 -d 'Hello, world!'")]
    Sha384 {
        data: Option<String>,
    },
    #[command(
        name = "sha224", 
        about = "Computes SHA-224 hash", 
        long_about = None, 
        after_help = "Example: sgtl sha224 -d 'Hello, world!'")]
    Sha224 {
        data: Option<String>,
    },
    #[command(
        name = "sha512_256", 
        about = "Computes SHA-512/256 hash", 
        long_about = None, 
        after_help = "Example: sgtl sha512_256 -d 'Hello, world!'")]
    Sha512_256 {
        data: Option<String>,
    },
    #[command(
        name = "caesar", 
        about = "Applies Caesar cipher with a specified shift", 
        long_about = None, 
        after_help = "Example: sgtl caesar 3 'Hello, world!'")]
    Caesar {
        shift: i8,
        data: Option<String>,
    },
}

impl Method {
    fn data(&self) -> &Option<String> {
        match self {
            Method::Echo { data }
            | Method::Rot26 { data }
            | Method::Base64 { data }
            | Method::Sha256 { data }
            | Method::Sha512 { data }
            | Method::Sha384 { data }
            | Method::Sha224 { data }
            | Method::Sha512_256 { data }
            | Method::Caesar { data, .. } => data,
        }
    }
}


fn get_data(args: &Args) -> String {
    let file = &args.input_file;
    let data = args.method.data();


    match (file, data) {
        (Some(_), Some(_)) => {
            panic!("Cannot provide both input file and direct data");
        }
        (Some(file_path), None) => {
            std::fs::read_to_string(file_path).expect("Failed to read input file")
        }
        (None, Some(d)) => d.to_owned(),
        (None, None) => {
            panic!("Must provide either input file or direct data");
        }
    }
}


fn main() {
    let args: Args = Args::parse();

    if args.verbose {
        println!("Method: {:?}", args.method);
        println!("input_file: {:?}", args.input_file);
        println!("Decode: {}", args.decode);
    }

    let input_data: String = get_data(&args);

    if args.verbose {
        println!("Input data: {}", input_data);
    }

    let output: String = match &args.method {
        Method::Echo { data: _ } | Method::Rot26 { data: _ } => methods::echo::echo(&input_data).to_string(),
        Method::Base64 { data: _ } => {
            if args.decode {
                match methods::base64::base64_decode(&input_data) {
                    Ok(decoded) => decoded,
                    Err(e) => {
                        eprintln!("base64 error: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                methods::base64::base64_encode(&input_data)
            }
        }
        Method::Sha256 { data: _ } => methods::sha2::sha256_hash(&input_data),
        Method::Sha512 { data: _ } => methods::sha2::sha512_hash(&input_data),
        Method::Sha384 { data: _ } => methods::sha2::sha384_hash(&input_data),
        Method::Sha224 { data: _ } => methods::sha2::sha224_hash(&input_data),
        Method::Sha512_256 { data: _ } => methods::sha2::sha512_256_hash(&input_data),
        Method::Caesar { data: _, shift } => {
            if args.decode {
                methods::caesar::caesar_decipher(&input_data, *shift)
            } else {
                methods::caesar::caesar_encipher(&input_data, *shift)
            }
        }
    };

    if let Some(output_file) = args.output_file {
        std::fs::write(output_file, &output).expect("Failed to write to output file");
    } else {
        println!("{}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_from_args() {
        let args = Args {
            verbose: false,
            decode: false,
            input_file: None,
            output_file: None,
            method: Method::Echo {
                data: Some("Test data".to_string()),
            },
        };
        let data = get_data(&args);
        assert_eq!(data, "Test data");
    }

    #[test]
    fn test_get_data_from_args_with_caesar() {
        let args = Args {
            verbose: false,
            decode: false,
            input_file: None,
            output_file: None,
            method: Method::Caesar {
                shift: 5,
                data: Some("Caesar data".to_string()),
            },
        };
        let data = get_data(&args);
        assert_eq!(data, "Caesar data");
    }

    #[test]
    fn test_error_when_data_and_file_missing() {
        let args = Args {
            verbose: false,
            decode: false,
            input_file: None,
            output_file: None,
            method: Method::Echo { data: None },
        };
        let result = std::panic::catch_unwind(|| {
            get_data(&args);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_error_when_data_and_file_both_present() {
        let args = Args {
            verbose: false,
            decode: false,
            input_file: Some("test.txt".to_string()),
            output_file: None,
            method: Method::Echo { data: Some("Test data".to_string()) },
        };
        let result = std::panic::catch_unwind(|| {
            get_data(&args);
        });
        assert!(result.is_err());
    }
}