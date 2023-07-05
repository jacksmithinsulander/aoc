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
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let special_rules = ['b', 'd', 'q', 'y'];

    for i in trimmed_data_array {
        let mut last_letter: char = ' ';
        let mut points: (u32, u32, u32) = (0, 1, 0);
        let mut vowel_count: u32 = 0;
        for letter in i.chars() {
            if last_letter == letter {
                points.0 = 1;
            }
            if special_rules.contains(&letter) && last_letter == ((letter as u8) - 1) as char {
                points.1 = 0;
                break;
            }
            if vowels.contains(&letter) {
                vowel_count += 1;
            }
            if vowel_count == 3 {
                points.2 = 1;
            }
            last_letter = letter;
        }
        if points >= (1, 1, 1) {
            result += 1;
            println!("{} is a nice string with", i);
        }
    }
    println!("Result is : {}", result);
}
