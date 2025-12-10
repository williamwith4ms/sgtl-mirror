use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// verbose mode
    #[arg(short, long, global = true)]
    pub(crate) verbose: bool,
    /// decode mode
    #[arg(short, long, global = true)]
    pub(crate) decode: bool,

    /// input file
    #[arg(short = 'i', long = "input", global = true)]
    pub(crate) input_file: Option<String>,

    /// output file
    #[arg(short = 'o', long = "output", global = true)]
    pub(crate) output_file: Option<String>,

    /// subcommand to choose method
    #[command(subcommand)]
    pub(crate) method: Method,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Method {
    #[command(
        name = "echo", 
        about = "Returns the input unchanged", 
        long_about = None,
        after_help = "Example: sgtl echo -d 'Hello, world!'")]
    Echo { data: Option<String> },
    #[command(
        name = "rot26",
        about = "Applies ROT26 cipher (no change)",
        long_about = "Applies ROT26 cipher (no change), this is actually just an alias for echo.",
        after_help = "Example: sgtl rot26 -d 'Hello, world!'"
    )]
    Rot26 { data: Option<String> },
    #[command(
        name = "base64", 
        about = "Encodes/Decodes Base64", 
        long_about = None,
        after_help = "Example: sgtl base64 -d 'Hello, world!'")]
    Base64 { data: Option<String> },
    #[command(
        name = "sha256", 
        about = "Computes SHA-256 hash", 
        long_about = None,
        after_help = "Example: sgtl sha256 -d 'Hello, world!'")]
    Sha256 { data: Option<String> },
    #[command(
        name = "sha512", 
        about = "Computes SHA-512 hash", 
        long_about = None,
        after_help = "Example: sgtl sha512 -d 'Hello, world!'")]
    Sha512 { data: Option<String> },
    #[command(
        name = "sha384", 
        about = "Computes SHA-384 hash", 
        long_about = None,
        after_help = "Example: sgtl sha384 -d 'Hello, world!'")]
    Sha384 { data: Option<String> },
    #[command(
        name = "sha224", 
        about = "Computes SHA-224 hash", 
        long_about = None,
        after_help = "Example: sgtl sha224 -d 'Hello, world!'")]
    Sha224 { data: Option<String> },
    #[command(
        name = "sha512_256", 
        about = "Computes SHA-512/256 hash", 
        long_about = None,
        after_help = "Example: sgtl sha512_256 -d 'Hello, world!'")]
    Sha512_256 { data: Option<String> },
    #[command(
        name = "caesar", 
        about = "Applies Caesar cipher with a specified shift", 
        long_about = None,
        after_help = "Example: sgtl caesar 3 'Hello, world!'")]
    Caesar { shift: i8, data: Option<String> },
}

#[allow(dead_code)]
impl Method {
    pub fn data(&self) -> &Option<String> {
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
