use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input_file = fs::File::open("src/bin/day1/input.txt").expect("No such file!");
    let buf_reader = BufReader::new(input_file);
    let lines: Vec<String> = buf_reader
        .lines()
        .map(|l| l.expect("Parse failure"))
        .collect();

    let mut increases: i32 = 0;
    let mut prev = i32::MAX;
    for line in lines {
        match line.trim().parse::<i32>() {
            Ok(value) => {
                if prev < value {
                    increases += 1;
                }
                prev = value;
            }
            _ => {}
        }
    }

    println!("Number of increases: {:?}", increases);
}
