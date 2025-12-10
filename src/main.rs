use clap::Parser;
mod args;
mod methods;
use crate::args::{Args, Method};

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
        Method::Echo { data: _ } | Method::Rot26 { data: _ } => {
            methods::echo::echo(&input_data).to_string()
        }
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
        Method::Sha256 { data: _ } => {
            if args.decode {
                eprintln!(
                    "Error: SHA256 is one-way (trust me, if i could decode sha i wouldn't be doing this)"
                );
                std::process::exit(1);
            }
            methods::sha2::sha256_hash(&input_data)
        }
        Method::Sha512 { data: _ } => {
            if args.decode {
                eprintln!(
                    "Error: SHA512 is one-way (trust me, if i could decode sha i wouldn't be doing this)"
                );
                std::process::exit(1);
            }
            methods::sha2::sha512_hash(&input_data)
        }
        Method::Sha384 { data: _ } => {
            if args.decode {
                eprintln!(
                    "Error: SHA384 is one-way (trust me, if i could decode sha i wouldn't be doing this)"
                );
                std::process::exit(1);
            }
            methods::sha2::sha384_hash(&input_data)
        }
        Method::Sha224 { data: _ } => {
            if args.decode {
                eprintln!(
                    "Error: SHA224 is one-way (trust me, if i could decode sha i wouldn't be doing this)"
                );
                std::process::exit(1);
            }
            methods::sha2::sha224_hash(&input_data)
        }
        Method::Sha512_256 { data: _ } => {
            if args.decode {
                eprintln!(
                    "Error: SHA512/256 is one-way (trust me, if i could decode sha i wouldn't be doing this)"
                );
                std::process::exit(1);
            }
            methods::sha2::sha512_256_hash(&input_data)
        }
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
            method: Method::Echo {
                data: Some("Test data".to_string()),
            },
        };
        let result = std::panic::catch_unwind(|| {
            get_data(&args);
        });
        assert!(result.is_err());
    }
}
