pub fn get_row_and_new_num(
    word_lengths: &[usize],
    num: usize,
    symbol: &str,
) -> (String, String) {
    let count = word_lengths.iter().filter(|x| x == &&num).count();
    let hist_row = get_row(count, symbol);
    let new_num = format_num(num);
    (hist_row, new_num)
}

fn get_row(count: usize, symbol: &str) -> String {
    let mut row = String::new();
    for _ in 0..(count) {
        row.push_str(symbol);
    }
    row
}

fn format_num(num: usize) -> String {
    if num < 10 {
        return " ".to_string() + num.to_string().as_str();
    }
    num.to_string()
}
