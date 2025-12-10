use clap::CommandFactory;
use clap_complete::{generate_to, shells};
use std::path::Path;

mod args;

fn main() {
    let mut cmd = args::Args::command();

    let out_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "completions".into());
    let out_dir = Path::new(&out_dir);

    std::fs::create_dir_all(out_dir).expect("failed to create completions dir");

    let bin_name = "sgtl";

    generate_to(shells::Bash, &mut cmd, bin_name, out_dir).unwrap();
    generate_to(shells::Zsh, &mut cmd, bin_name, out_dir).unwrap();
    generate_to(shells::Fish, &mut cmd, bin_name, out_dir).unwrap();
}
