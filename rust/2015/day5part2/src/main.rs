use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input.");
    let data_array: Vec<&str> = contents.split('\n').collect();
    let trimmed_data_array: Vec<&str> = data_array
        .iter()
        .map(|s| s.trim_start_matches(' '))
        .collect();
    let mut result: u32 = 0;
}
