use crate::row_creator::get_row_and_new_num;

pub fn create_horizontal_histogram(word_lengths: &[usize], max_length: usize) {
    for num in 1..(max_length + 1) {
        let symbol = "|";
        let (hist_row, new_num) = get_row_and_new_num(word_lengths, num, symbol);
        println!("length {new_num}:  {hist_row:<5}");
    }
}
