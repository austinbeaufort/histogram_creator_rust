use crate::row_creator::get_row_and_new_num;
use std::cmp::max;

pub fn create_vertical_histogram(word_lengths: &[usize], max_length: usize) {
    let (base_row, zipped_rows) = get_zipped_and_base(max_length, word_lengths);
    for row in zipped_rows.iter().rev() {
        let formatted_row = format_row(row);
        println!("{formatted_row}");
    }
    println!("{base_row}");
}

fn get_zipped_and_base(max_length: usize, word_lengths: &[usize]) -> (String, Vec<String>) {
    let base_row = make_base_row(max_length);
    let all_rows = get_all_rows(word_lengths, max_length);
    let max_recurrence = get_max_recurrence(&all_rows);
    let zipped_rows = zip_em(all_rows, max_recurrence);
    (base_row, zipped_rows)
}

fn format_row(row: &str) -> String {
    let arr: Vec<&str> = row.split("").collect();
    let mut new_arr = Vec::new();
    for (i, item) in arr.into_iter().enumerate() {
        if i != 0 {
            new_arr.push(String::from(item) + "  ");
        } else {
            new_arr.push(String::from(item));
        }
    }
    new_arr.into_iter().collect::<Vec<String>>().join("")
}

fn make_base_row(max_length: usize) -> String {
    let mut base_row = String::new();
    for num in 1..(max_length + 1) {
        base_row.push_str(&get_base_row_value(num));
    }
    base_row
}

fn get_base_row_value(num: usize) -> String {
    let new_str = num.to_string() + " ";
    let single_digit_str = String::from("0") + &new_str;
    if num < 10 {
        single_digit_str
    } else {
        new_str
    }
}

fn get_all_rows(word_lengths: &[usize], max_length: usize) -> Vec<String> {
    let mut all_rows = Vec::new();
    for num in 1..(max_length + 1) {
        let symbol = "*";
        let (hist_row, _) = get_row_and_new_num(word_lengths, num, symbol);
        all_rows.push(hist_row);
    }
    all_rows
}

// COULD PROBABLY USE THE BUILT IN ZIP FUNCTION, BUILDING MY OWN FOR PRACTICE
fn zip_em(words: Vec<String>, max_recurrence: usize) -> Vec<String> {
    let mut zipped_items = Vec::new();
    for i in 0..(max_recurrence + 1) {
        let mut current_item = String::new();
        for word in &words {
            match word.as_str().chars().nth(i) {
                Some(val) => current_item.push_str(String::from(val).as_str()),
                None => current_item.push(' '),
            }
        }
        zipped_items.push(current_item);
    }
    zipped_items
}

fn get_max_recurrence(words: &Vec<String>) -> usize {
    let mut current_max = 0;
    for word in words {
        let length = word.len();
        current_max = max(length, current_max);
    }
    current_max
}
