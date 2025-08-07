use std::{env, fs::read_to_string};

use regex::Regex;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args.get(1).expect("provide path to html file");
    let html = read_to_string(path).unwrap();
    let content_regex = Regex::new(r"(?s)<body>(.*)</body>").unwrap();
    let body = content_regex
        .captures(&html)
        .unwrap()
        .get(1)
        .expect("html file with body")
        .as_str()
        .trim()
        .replace('\n', " ");

    println!("{}", body);
}
