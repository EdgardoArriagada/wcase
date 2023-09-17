mod args;
use args::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();

    match args {
        Args { flat: true, .. } => println!("flat case"),
        Args { upper: true, .. } => println!("upper case"),
        Args { camel: true, .. } => println!("camel case"),
        Args { pascal: true, .. } => println!("pascal case"),
        Args { snake: true, .. } => println!("snake case"),
        Args { all_caps: true, .. } => println!("all_caps case"),
        Args {
            camel_snake: true, ..
        } => println!("camel_snake case"),
        Args {
            pascal_snake: true, ..
        } => println!("pascal_snake case"),
        Args { kebab: true, .. } => println!("kebab case"),
        Args { train: true, .. } => println!("train case"),
        Args {
            http_header: true, ..
        } => println!("http_header case"),
        _ => println!("no match"),
    }
}
