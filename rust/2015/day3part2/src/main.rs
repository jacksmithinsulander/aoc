use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input.");

    let chunks = contents.chars().collect::<Vec<char>>();

    let mut house_map: HashMap<(i32, i32), i32> = HashMap::new();

    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);

    house_map.insert(santa_pos, 1);

    for (i, chunk) in chunks.iter().enumerate() {
        let position = if i % 2 == 0 {
            &mut santa_pos
        } else {
            &mut robot_pos
        };

        match chunk {
            '^' => {
                position.1 += 1;
            }
            'v' => {
                position.1 -= 1;
            }
            '>' => {
                position.0 += 1;
            }
            '<' => {
                position.0 -= 1;
            }
            _ => {}
        }

        let value = house_map.entry(*position).or_insert(0);
        *value += 1;
    }

    let count = house_map.len();

    println!("Amount of houses: {}", count)
}
