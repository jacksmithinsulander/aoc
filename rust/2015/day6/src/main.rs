use regex::Regex;

struct Instructor {
    instruction: String,
    start: Vec<usize>,
    end: Vec<usize>,
}

fn main() {
    //let data = include_str!("./inmput.txt");

    let data = "turn off 854,56 through 965,591";
    let parser = Regex::new(
        r"(?P<inst>turn off|turn on|toggle) (?P<start>\d+,\d+) through (?P<end>\d+,\d+)",
    )
    .unwrap();
    let captures = parser.captures(data).ok_or("no matches").unwrap();
    let instruction = captures.name("inst").map_or("", |m| m.as_str()).to_string();
    let start: Vec<usize> = captures
        .name("start")
        .map_or("", |m| m.as_str())
        .split(',')
        .map(|s| s.parse().expect("Parse error, {s} not an int"))
        .collect();
    let end: Vec<usize> = captures
        .name("end")
        .map_or("", |m| m.as_str())
        .split(',')
        .map(|s| s.parse().expect("Parse error, {s} not an int"))
        .collect();

    let i: Instruction {
        instruction: instruction,
        start: start,
        end: end,
    }
    println!("Part 1: {:?}", start);
    println!("Part 2: {:?}", end)
}
