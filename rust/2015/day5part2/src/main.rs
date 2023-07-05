use std::fs::File;
use std::io::Read;
use fancy_regex::Regex;

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input.");
    let data_array: Vec<&str> = contents.split('\n').collect();
    let santa_string: Vec<&str> = data_array.iter().map(|s| s.trim()).collect();
    let mut result: u32 = 0;

    let re1 = Regex::new(r"(..).*\1").unwrap();
    let re2 = Regex::new(r"(.).\1").unwrap();
    
    for i in santa_string {
        let mut points: (u32, u32) = (0, 0);

        if re1.is_match(i).unwrap() {
            points.0 = 1;
        }
        if re2.is_match(i).unwrap() {
            points.1 = 1;
        }
        
        if points.0 >= 1 && points.1 >= 1 {
            result += 1;
        }
        //println!("{}{}", result1, result2);
    }
    println!("{}", result)
}
