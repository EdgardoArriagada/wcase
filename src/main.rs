mod args;
use args::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let result = match args {
        Args { flat: true, .. } => flat_case(&args.word),
        Args { upper: true, .. } => "upper case".into(),
        Args { camel: true, .. } => "camel case".into(),
        Args { pascal: true, .. } => "pascal case".into(),
        Args { snake: true, .. } => "snake case".into(),
        Args { all_caps: true, .. } => "all_caps case".into(),
        Args {
            camel_snake: true, ..
        } => "camel_snake case".into(),
        Args {
            pascal_snake: true, ..
        } => "pascal_snake case".into(),
        Args { kebab: true, .. } => "kebab case".into(),
        Args { train: true, .. } => "train case".into(),
        Args {
            http_header: true, ..
        } => "http_header case".into(),
        _ => "no match".into(),
    };

    println!("{}", result);
}

fn flat_case(word: &str) -> String {
    word.to_lowercase()
}
