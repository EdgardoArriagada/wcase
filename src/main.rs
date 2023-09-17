mod args;
use args::Args;

extern crate regex;
use regex::Regex;

use clap::Parser;

static CAMEL_REGEX: &str = r"^[a-z]+(?:[A-Z][a-z]+)*$";
static PASCAL_REGEX: &str = r"^[A-Z][a-z]+(?:[A-Z][a-z]+)*$";

fn main() {
    let args = Args::parse();

    let case = get_case(&args.word);

    if case == "none" {
        println!("Invalid input");
        return;
    }

    let result = match args {
        Args { flat: true, .. } => flat_case(&args.word),
        Args { upper: true, .. } => upper_case(&args.word),
        Args { camel: true, .. } => camel_case(&args.word, case),
        Args { pascal: true, .. } => "pascal case".into(),
        Args { snake: true, .. } => "snake case".into(),
        Args { all_caps: true, .. } => "all_caps case".into(),
        Args { kebab: true, .. } => "kebab case".into(),
        Args { train: true, .. } => "train case".into(),
        _ => case,
    };

    println!("{}", result);
}

fn get_case(word: &str) -> String {
    if word.contains(' ') || word.contains('-') && word.contains('_') {
        return "none".into();
    }

    if !word.contains('-') && !word.contains('_') {
        if word.to_lowercase() == word {
            return "flat".into();
        } else if word.to_uppercase() == word {
            return "upper".into();
        }

        let camel_case = Regex::new(CAMEL_REGEX).unwrap();
        if camel_case.is_match(word) {
            return "camel".into();
        }

        let pascal_case = Regex::new(PASCAL_REGEX).unwrap();
        if pascal_case.is_match(word) {
            return "pascal".into();
        }
    }

    if word.contains('_') {
        if word.to_lowercase() == word {
            return "snake".into();
        } else if word.to_uppercase() == word {
            return "all_caps".into();
        }
    }

    if word.contains('-') {
        if word.to_lowercase() == word {
            return "kebab".into();
        } else if word.to_uppercase() == word {
            return "train".into();
        }
    }

    return "none".into();
}

fn capitalize_first_letter(word: &str) -> String {
    let mut v: Vec<char> = word.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    return v.into_iter().collect();
}

fn lower_first_letter(word: &str) -> String {
    let mut v: Vec<char> = word.chars().collect();
    v[0] = v[0].to_lowercase().nth(0).unwrap();
    return v.into_iter().collect();
}

fn flat_case(word: &str) -> String {
    word.replace("-", "").replace("_", "").to_lowercase()
}

fn upper_case(word: &str) -> String {
    word.replace("-", "").replace("_", "").to_uppercase()
}

fn camel_case(word: &str, case: String) -> String {
    match case.as_str() {
        "camel" => return word.to_string(),
        "pascal" => return lower_first_letter(word),
        "upper" => return word.to_string(),
        _ => (),
    }

    let mut result = String::new();
    let mut first = true;

    for part in word.split(|c| c == '-' || c == '_') {
        if first {
            result.push_str(&part.to_lowercase());
            first = false;
        } else {
            result.push_str(&part[..1].to_uppercase());
            result.push_str(&part[1..].to_lowercase());
        }
    }

    lower_first_letter(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_case() {
        assert_eq!(flat_case("helloworld"), "helloworld");
        assert_eq!(flat_case("HELLOWORLD"), "helloworld");
        assert_eq!(flat_case("helloWorld"), "helloworld");
        assert_eq!(flat_case("HelloWorld"), "helloworld");
        assert_eq!(flat_case("hello_world"), "helloworld");
        assert_eq!(flat_case("HELLO_WORLD"), "helloworld");
        assert_eq!(flat_case("hello-world"), "helloworld");
        assert_eq!(flat_case("HELLO-WORLD"), "helloworld");
    }

    #[test]
    fn test_upper_case() {
        assert_eq!(upper_case("helloworld"), "HELLOWORLD");
        assert_eq!(upper_case("HELLOWORLD"), "HELLOWORLD");
        assert_eq!(upper_case("helloWorld"), "HELLOWORLD");
        assert_eq!(upper_case("HelloWorld"), "HELLOWORLD");
        assert_eq!(upper_case("hello_world"), "HELLOWORLD");
        assert_eq!(upper_case("HELLO_WORLD"), "HELLOWORLD");
        assert_eq!(upper_case("hello-world"), "HELLOWORLD");
        assert_eq!(upper_case("HELLO-WORLD"), "HELLOWORLD");
    }

    #[test]
    fn test_camel_case() {
        fn camel_case_helper(word: &str) -> String {
            camel_case(word, get_case(word))
        }

        assert_eq!(camel_case_helper("helloworld"), "helloworld"); // no camel_case
        assert_eq!(camel_case_helper("HELLOWORLD"), "HELLOWORLD"); // no camel_case
        assert_eq!(camel_case_helper("helloWorld"), "helloWorld");
        assert_eq!(camel_case_helper("HelloWorld"), "helloWorld");
        assert_eq!(camel_case_helper("hello_world"), "helloWorld");
        assert_eq!(camel_case_helper("HELLO_WORLD"), "helloWorld");
        assert_eq!(camel_case_helper("hello-world"), "helloWorld");
        assert_eq!(camel_case_helper("HELLO-WORLD"), "helloWorld");
    }

    #[test]
    fn test_get_case() {
        assert_eq!(get_case("helloworld"), "flat");
        assert_eq!(get_case("HELLOWORLD"), "upper");
        assert_eq!(get_case("helloWorld"), "camel");
        assert_eq!(get_case("HelloWorld"), "pascal");
        assert_eq!(get_case("hello_world"), "snake");
        assert_eq!(get_case("HELLO_WORLD"), "all_caps");
        assert_eq!(get_case("hello-world"), "kebab");
        assert_eq!(get_case("HELLO-WORLD"), "train");

        assert_eq!(get_case("hello world"), "none");
        assert_eq!(get_case("hello-new_world"), "none");
        assert_eq!(get_case("hello-World"), "none");
    }
}
