use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    let input_file = File::open("src/bin/day2/input.txt").expect("No such file!");
    let buf_reader = BufReader::new(input_file);
    let lines: Vec<String> = buf_reader
        .lines()
        .map(|l| l.expect("Parse failure"))
        .collect();

    let mut h_pos: i32 = 0;
    let mut h_depth: i32 = 0;

    println!("Part 1:");

    for line in &lines {
        let substrings: Vec<&str> = line.split_whitespace().collect();
        let value = substrings[1].parse::<i32>().unwrap();
        match substrings[0] {
            "forward" => h_pos += value,
            "up" => h_depth -= value,
            "down" => h_depth += value,
            _ => {}
        }
    }

    println!("Horizontal Pos: {:?}", h_pos);
    println!("Horizontal Depth: {:?}", h_depth);
    println!("Combined: {:?}", h_pos * h_depth);

    println!("Part 2:");

    h_pos = 0;
    h_depth = 0;
    let mut h_aim = 0;
    for line in &lines {
        let substrings: Vec<&str> = line.split_whitespace().collect();
        let value = substrings[1].parse::<i32>().unwrap();
        match substrings[0] {
            "forward" => {
                h_pos += value;
                h_depth += h_aim * value;
            }
            "up" => h_aim -= value,
            "down" => h_aim += value,
            _ => {}
        }
    }

    println!("Horizontal Pos: {:?}", h_pos);
    println!("Horizontal Depth: {:?}", h_depth);
    println!("Horizontal Aim: {:?}", h_aim);
    println!("Combined: {:?}", h_pos * h_depth);
}
