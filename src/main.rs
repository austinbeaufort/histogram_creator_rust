/*
HORIZONTAL HISTOGRAM OF WORD FREQUENCIES IN A file
*/
use std::{cmp::max, env, fs};
mod horizontal_histogram;
mod row_creator;
mod vertical_histogram;

fn main() {
    println!("================= HISTOGRAM CREATOR ==================");
    let (path, hist_type) = get_args();
    let contents = get_file_contents(&path);
    let words = contents.split_whitespace().map(String::from).collect();
    let word_lengths = format_words(words);
    let max_length = get_max(&word_lengths);
    print_histogram(&word_lengths, max_length, &hist_type);
}

fn get_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    check_args_err(&args);
    let path = String::from(&args[1]);
    let hist_type = String::from(&args[2]);
    (path, hist_type)
}

fn print_histogram(word_lengths: &[usize], max_length: usize, hist_type: &str) {
    let err = "invalid hist type, please enter horizontal or vertical as the 2nd arg.";
    match hist_type {
        "horizontal" => horizontal_histogram::create_horizontal_histogram(word_lengths, max_length),
        "vertical" => vertical_histogram::create_vertical_histogram(word_lengths, max_length),
        _ => panic!("{err}"),
    }
}

fn check_args_err(args: &Vec<String>) {
    let err = "Program takes 2 args, must add a file path
        as first arg and histogram type (vertical or horizontal) as 2nd arg.";
    if args.len() != 3 {
        panic!("{err}");
    }
}

fn get_max(word_lengths: &Vec<usize>) -> usize {
    let mut current_max = 0;
    for num in word_lengths {
        current_max = max(num.to_owned(), current_max);
    }
    current_max
}

fn get_file_contents(path: &String) -> String {
    let err = "cannot read file.";
    fs::read_to_string(path).expect(err)
}

// FUNCS TO GET ARRAY OF FORMATTED WORDS
fn format_words(words: Vec<String>) -> Vec<usize> {
    words.into_iter().map(get_formatted_length).collect()
}

fn get_formatted_length(word: String) -> usize {
    let no_punctuation_word = remove_punctuation(word);
    no_punctuation_word.len()
}

fn remove_punctuation(word: String) -> String {
    let invalid_chars = ['!', '.', ',', ':'];
    word.chars()
        .filter(|ch| !invalid_chars.contains(ch))
        .collect()
}
