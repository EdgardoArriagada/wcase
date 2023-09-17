mod args;
use args::Args;

extern crate regex;
use regex::Regex;

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

fn get_case(word: &str) -> String {
    let mut result = String::new();
    if word.contains(' ') {
        return "none".into();
    }

    if !word.contains('-') || !word.contains('_') {
        if word.to_lowercase() == word {
            return "flat".into();
        } else if word.to_uppercase() == word {
            return "upper".into();
        }

        let camel_case = Regex::new(r"^[a-z]+(?:[A-Z][a-z]+)*$").unwrap();
        if camel_case.is_match(word) {
            return "camel".into();
        }

        let pascal_case = Regex::new(r"^[A-Z][a-z]+(?:[A-Z][a-z]+)*$").unwrap();
        if pascal_case.is_match(word) {
            return "pascal".into();
        }
    }

    if word.contains('-') {
        result.push_str("kebab");
    }

    return "none".into();
}

fn flat_case(word: &str) -> String {
    word.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_case() {
        assert_eq!(flat_case("Hello World"), "hello world");
    }

    #[test]
    fn test_get_case() {
        assert_eq!(get_case("helloworld"), "flat");
        assert_eq!(get_case("HELLOWORLD"), "upper");
        assert_eq!(get_case("hello world"), "none");
        assert_eq!(get_case("HelloWorld"), "pascal");
        assert_eq!(get_case("helloWorld"), "camel");
    }
}
