mod args;
use std::fmt;

use args::Args;

extern crate regex;
use regex::Regex;

use clap::Parser;

static CAMEL_REGEX: &str = r"^[a-z]+(?:[A-Z][a-z]+)*$";
static PASCAL_REGEX: &str = r"^[A-Z][a-z]+(?:[A-Z][a-z]+)*$";

#[derive(Debug)]
enum Case {
    Flat,
    Upper,
    Camel,
    Pascal,
    Snake,
    AllCaps,
    Kebab,
    Train,
    Spaced,
    None,
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Case::Flat => write!(f, "flat"),
            Case::Upper => write!(f, "upper"),
            Case::Camel => write!(f, "camel"),
            Case::Pascal => write!(f, "pascal"),
            Case::Snake => write!(f, "snake"),
            Case::AllCaps => write!(f, "all_caps"),
            Case::Kebab => write!(f, "kebab"),
            Case::Train => write!(f, "train"),
            Case::Spaced => write!(f, "spaced"),
            Case::None => write!(f, "none"),
        }
    }
}

impl PartialEq for Case {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Case::Flat, Case::Flat) => true,
            (Case::Upper, Case::Upper) => true,
            (Case::Camel, Case::Camel) => true,
            (Case::Pascal, Case::Pascal) => true,
            (Case::Snake, Case::Snake) => true,
            (Case::AllCaps, Case::AllCaps) => true,
            (Case::Kebab, Case::Kebab) => true,
            (Case::Train, Case::Train) => true,
            (Case::Spaced, Case::Spaced) => true,
            (Case::None, Case::None) => true,
            _ => false,
        }
    }
}

fn main() {
    let args = Args::parse();

    let case = get_case(&args.word);

    if case == Case::None {
        println!("Invalid input");
        return;
    }

    let result = match args {
        Args { flat: true, .. } => flat_case(&args.word),
        Args { upper: true, .. } => upper_case(&args.word),
        Args { camel: true, .. } => camel_case(&args.word, case),
        Args { pascal: true, .. } => pascal_case(&args.word, case),
        Args { snake: true, .. } => snake_case(&args.word, case),
        Args { all_caps: true, .. } => all_caps_case(&args.word, case),
        Args { kebab: true, .. } => kebab_case(&args.word, case),
        Args { train: true, .. } => train_case(&args.word, case),
        Args { spaced: true, .. } => spaced_case(&args.word, case),
        _ => case.to_string(),
    };

    println!("{}", result);
}

fn get_case(word: &str) -> Case {
    if word.contains('-') && word.contains('_') {
        return Case::None;
    }

    let word_lower = word.to_lowercase();

    if !word.contains('-') && !word.contains('_') {
        if word.contains(' ') && word_lower == word {
            return Case::Spaced;
        }

        if word_lower == word {
            return Case::Flat;
        } else if word.to_uppercase() == word {
            return Case::Upper;
        }

        let camel_case = Regex::new(CAMEL_REGEX).unwrap();
        if camel_case.is_match(word) {
            return Case::Camel;
        }

        let pascal_case = Regex::new(PASCAL_REGEX).unwrap();
        if pascal_case.is_match(word) {
            return Case::Pascal;
        }
    }

    if word.contains('_') {
        if word_lower == word {
            return Case::Snake;
        } else if word.to_uppercase() == word {
            return Case::AllCaps;
        }
    }

    if word.contains('-') {
        if word_lower == word {
            return Case::Kebab;
        } else if word.to_uppercase() == word {
            return Case::Train;
        }
    }

    return Case::None;
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

fn flat_word(word: &str) -> String {
    word.replace("-", "").replace("_", "").replace(" ", "")
}

fn flat_case(word: &str) -> String {
    flat_word(word).to_lowercase()
}

fn upper_case(word: &str) -> String {
    flat_word(word).to_uppercase()
}

fn camel_case(word: &str, case: Case) -> String {
    match case {
        Case::Camel => return word.to_string(),
        Case::Pascal => return lower_first_letter(word),
        Case::Flat => return word.to_string(),
        Case::Upper => return word.to_string(),
        _ => (),
    }

    let mut result = String::new();
    let mut first = true;

    for part in word.split(|c| c == '-' || c == '_' || c == ' ') {
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

fn pascal_case(word: &str, case: Case) -> String {
    match case {
        Case::Camel => return capitalize_first_letter(word),
        Case::Pascal => return word.to_string(),
        Case::Flat => return word.to_string(),
        Case::Upper => return word.to_string(),
        _ => (),
    }

    capitalize_first_letter(&camel_case(word, case))
}

fn snake_case(word: &str, case: Case) -> String {
    match case {
        Case::Snake => return word.to_string(),
        Case::AllCaps => return word.to_lowercase(),
        Case::Flat => return word.to_string(),
        Case::Upper => return word.to_string(),
        Case::Kebab => return word.replace("-", "_"),
        Case::Train => return word.replace("-", "_").to_lowercase(),
        Case::Spaced => return word.replace(" ", "_").to_lowercase(),
        _ => (),
    }

    let mut result = String::new();
    let mut first = true;

    for c in word.chars() {
        if first {
            result.push(c.to_lowercase().nth(0).unwrap());
            first = false;
            continue;
        }

        if c.is_uppercase() {
            result.push('_');
            result.push(c.to_lowercase().nth(0).unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

fn all_caps_case(word: &str, case: Case) -> String {
    match case {
        Case::Snake => return word.to_uppercase(),
        Case::AllCaps => return word.to_string(),
        Case::Flat => return word.to_uppercase(),
        Case::Upper => return word.to_uppercase(),
        Case::Kebab => return word.replace("-", "_").to_uppercase(),
        Case::Train => return word.replace("-", "_").to_uppercase(),
        _ => (),
    }

    snake_case(word, case).to_uppercase()
}

fn kebab_case(word: &str, case: Case) -> String {
    match case {
        Case::Snake => return word.replace("_", "-"),
        Case::AllCaps => return word.replace("_", "-").to_lowercase(),
        Case::Flat => return word.to_string(),
        Case::Upper => return word.to_lowercase(),
        Case::Kebab => return word.to_string(),
        Case::Train => return word.to_lowercase(),
        _ => (),
    }

    snake_case(word, case).replace("_", "-")
}

fn train_case(word: &str, case: Case) -> String {
    match case {
        Case::Snake => return word.replace("_", "-").to_uppercase(),
        Case::AllCaps => return word.replace("_", "-"),
        Case::Flat => return word.to_uppercase(),
        Case::Upper => return word.to_string(),
        Case::Kebab => return word.to_uppercase(),
        Case::Train => return word.to_string(),
        _ => (),
    }

    snake_case(word, case).replace("_", "-").to_uppercase()
}

fn spaced_case(word: &str, case: Case) -> String {
    match case {
        Case::Snake => return word.replace("_", " "),
        Case::AllCaps => return word.replace("_", " ").to_lowercase(),
        Case::Flat => return word.to_string(),
        Case::Upper => return word.to_lowercase(),
        Case::Kebab => return word.replace("-", " "),
        Case::Train => return word.replace("-", " ").to_lowercase(),
        Case::Spaced => return word.to_string(),
        _ => (),
    }

    snake_case(word, case).replace("_", " ")
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
        assert_eq!(flat_case("hello world"), "helloworld");
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
        assert_eq!(upper_case("hello world"), "HELLOWORLD");
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
        assert_eq!(camel_case_helper("hello world"), "helloWorld");
    }

    #[test]
    fn test_pascal_case() {
        fn pascal_case_helper(word: &str) -> String {
            pascal_case(word, get_case(word))
        }

        assert_eq!(pascal_case_helper("helloworld"), "helloworld"); // no pascal_case
        assert_eq!(pascal_case_helper("HELLOWORLD"), "HELLOWORLD"); // no pascal_case
        assert_eq!(pascal_case_helper("helloWorld"), "HelloWorld");
        assert_eq!(pascal_case_helper("HelloWorld"), "HelloWorld");
        assert_eq!(pascal_case_helper("hello_world"), "HelloWorld");
        assert_eq!(pascal_case_helper("HELLO_WORLD"), "HelloWorld");
        assert_eq!(pascal_case_helper("hello-world"), "HelloWorld");
        assert_eq!(pascal_case_helper("HELLO-WORLD"), "HelloWorld");
        assert_eq!(pascal_case_helper("hello world"), "HelloWorld");
    }

    #[test]
    fn test_snake_case() {
        fn snake_case_helper(word: &str) -> String {
            snake_case(word, get_case(word))
        }

        assert_eq!(snake_case_helper("helloworld"), "helloworld");
        assert_eq!(snake_case_helper("HELLOWORLD"), "HELLOWORLD");
        assert_eq!(snake_case_helper("helloWorld"), "hello_world");
        assert_eq!(snake_case_helper("HelloWorld"), "hello_world");
        assert_eq!(snake_case_helper("hello_world"), "hello_world");
        assert_eq!(snake_case_helper("HELLO_WORLD"), "hello_world");
        assert_eq!(snake_case_helper("hello-world"), "hello_world");
        assert_eq!(snake_case_helper("HELLO-WORLD"), "hello_world");
        assert_eq!(snake_case_helper("hello world"), "hello_world");
    }

    #[test]
    fn test_all_caps_case() {
        fn all_caps_case_helper(word: &str) -> String {
            all_caps_case(word, get_case(word))
        }

        assert_eq!(all_caps_case_helper("helloworld"), "HELLOWORLD");
        assert_eq!(all_caps_case_helper("HELLOWORLD"), "HELLOWORLD");
        assert_eq!(all_caps_case_helper("helloWorld"), "HELLO_WORLD");
        assert_eq!(all_caps_case_helper("HelloWorld"), "HELLO_WORLD");
        assert_eq!(all_caps_case_helper("hello_world"), "HELLO_WORLD");
        assert_eq!(all_caps_case_helper("HELLO_WORLD"), "HELLO_WORLD");
        assert_eq!(all_caps_case_helper("hello-world"), "HELLO_WORLD");
        assert_eq!(all_caps_case_helper("HELLO-WORLD"), "HELLO_WORLD");
        assert_eq!(all_caps_case_helper("hello world"), "HELLO_WORLD");
    }

    #[test]
    fn test_kebab_case() {
        fn kebab_case_helper(word: &str) -> String {
            kebab_case(word, get_case(word))
        }

        assert_eq!(kebab_case_helper("helloworld"), "helloworld");
        assert_eq!(kebab_case_helper("HELLOWORLD"), "helloworld");
        assert_eq!(kebab_case_helper("helloWorld"), "hello-world");
        assert_eq!(kebab_case_helper("HelloWorld"), "hello-world");
        assert_eq!(kebab_case_helper("hello_world"), "hello-world");
        assert_eq!(kebab_case_helper("HELLO_WORLD"), "hello-world");
        assert_eq!(kebab_case_helper("hello-world"), "hello-world");
        assert_eq!(kebab_case_helper("HELLO-WORLD"), "hello-world");
        assert_eq!(kebab_case_helper("hello world"), "hello-world");
    }

    #[test]
    fn test_train_case() {
        fn train_case_helper(word: &str) -> String {
            train_case(word, get_case(word))
        }

        assert_eq!(train_case_helper("helloworld"), "HELLOWORLD");
        assert_eq!(train_case_helper("HELLOWORLD"), "HELLOWORLD");
        assert_eq!(train_case_helper("helloWorld"), "HELLO-WORLD");
        assert_eq!(train_case_helper("HelloWorld"), "HELLO-WORLD");
        assert_eq!(train_case_helper("hello_world"), "HELLO-WORLD");
        assert_eq!(train_case_helper("HELLO_WORLD"), "HELLO-WORLD");
        assert_eq!(train_case_helper("hello-world"), "HELLO-WORLD");
        assert_eq!(train_case_helper("HELLO-WORLD"), "HELLO-WORLD");
        assert_eq!(train_case_helper("hello world"), "HELLO-WORLD");
    }

    #[test]
    fn tesp_spaced_case() {
        fn spaced_case_helper(word: &str) -> String {
            spaced_case(word, get_case(word))
        }

        assert_eq!(spaced_case_helper("helloworld"), "helloworld");
        assert_eq!(spaced_case_helper("HELLOWORLD"), "helloworld");
        assert_eq!(spaced_case_helper("helloWorld"), "hello world");
        assert_eq!(spaced_case_helper("HelloWorld"), "hello world");
        assert_eq!(spaced_case_helper("hello_world"), "hello world");
        assert_eq!(spaced_case_helper("HELLO_WORLD"), "hello world");
        assert_eq!(spaced_case_helper("hello-world"), "hello world");
        assert_eq!(spaced_case_helper("HELLO-WORLD"), "hello world");
        assert_eq!(spaced_case_helper("hello world"), "hello world");
    }

    #[test]
    fn test_get_case() {
        assert_eq!(get_case("helloworld"), Case::Flat);
        assert_eq!(get_case("HELLOWORLD"), Case::Upper);
        assert_eq!(get_case("helloWorld"), Case::Camel);
        assert_eq!(get_case("HelloWorld"), Case::Pascal);
        assert_eq!(get_case("hello_world"), Case::Snake);
        assert_eq!(get_case("HELLO_WORLD"), Case::AllCaps);
        assert_eq!(get_case("hello-world"), Case::Kebab);
        assert_eq!(get_case("HELLO-WORLD"), Case::Train);
        assert_eq!(get_case("hello world"), Case::Spaced);

        assert_eq!(get_case("hello-new_world"), Case::None);
        assert_eq!(get_case("hello-World"), Case::None);
    }
}
