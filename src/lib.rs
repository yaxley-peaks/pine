use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub i_file: PathBuf,
    #[arg(short)]
    pub lines: Vec<String>,
    #[arg(short)]
    pub o_file: PathBuf,
}

pub fn read_file_into_lines(path: impl AsRef<Path>) -> Vec<String> {
    fs::read_to_string(path)
        .map_err(|err| {
            eprintln!("Error reading file: {}", err);
            exit(1);
        })
        .unwrap()
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}
