use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input.");

    let mut house_map: HashMap<String, i32> = HashMap::new();

    let mut rows: i32 = 0;

    let mut columns: i32 = 0;

    for i in contents.chars() {
        match i {
            '^' => {
                columns += 1;
            }
            'v' => {
                columns -= 1;
            }
            '>' => {
                rows += 1;
            }
            '<' => {
                rows -= 1;
            }
            _ => {}
        }

        let location = format!("c{}r{}", columns, rows);

        let value = house_map.entry(location.clone()).or_insert(0);
        *value += 1;
    }

    for (key, value) in &house_map {
        println!("Key: {}, Value {}", key, value);
    }
    let count = house_map.len();

    println!("Amount of houses: {}", count)
}
