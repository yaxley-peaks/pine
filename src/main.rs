use clap::Parser;
use pine::Args;
fn main() {
    let args = Args::parse();
    println!("{args:?}");
    println!("Hello, world!");
}
