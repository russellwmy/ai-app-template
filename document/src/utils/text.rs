use regex::Regex;

pub fn split_into_sentences(text: &str) -> Vec<String> {
    let terminators = vec!['.', '?', '!', ';'];

    let parts = text
        .split_inclusive(&terminators[..])
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let mut results: Vec<String> = vec![];
    for part in parts {
        let first_char = part.trim().chars().next();
        let mut text = match first_char {
            Some(first_char) => {
                let mut buf = String::new();
                if !first_char.is_ascii_uppercase() && !results.is_empty() {
                    buf.push_str(&results.pop().unwrap());
                }
                buf
            }
            None => String::new(),
        };
        text.push_str(&part);
        results.push(text);
    }
    results
}

#[allow(dead_code)]
pub(crate) fn clean_text(text: &str) -> String {
    text.chars().filter(|c| !c.is_ascii_control()).collect()
}

#[allow(dead_code)]
pub(crate) fn all_uppercase(text: &str) -> bool {
    text.chars()
        .filter(|x| x.is_alphabetic())
        .all(|x| x.is_uppercase())
}

pub(crate) fn count_word(text: &str) -> usize {
    text.split_whitespace()
        .filter(|x| x.chars().all(|y| y.is_alphabetic()))
        .count()
}

pub(crate) fn count_space(text: &str) -> usize {
    let re = Regex::new(r"\s").unwrap();
    re.captures_iter(&text).count()
}

pub(crate) fn count_multi_space(text: &str) -> usize {
    let re = Regex::new(r"\s{2}").unwrap();
    re.captures_iter(&text).count()
}

pub(crate) fn check_start_with_reference_number(text: &str) -> bool {
    let re = Regex::new(r"^\[\d+\]").unwrap();
    re.is_match(text)
}

pub(crate) fn check_start_with_bullet(text: &str) -> bool {
    let re = Regex::new(r"^[-|•|‣|⁃|●|○|∙]").unwrap();
    re.is_match(text)
}
