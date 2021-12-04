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
    let values: Vec<i32> = lines
        .iter()
        .map(|str| str.trim().parse::<i32>().unwrap())
        .collect();
    let mut increases: i32 = 0;
    let mut prev = i32::MAX;

    // Part 1
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

    // Part 2
    let window_size = 3;
    let mut prev_sum: i32 = i32::MAX;
    let mut current_sum: i32;
    increases = 0;
    for i in 0..(values.len() - window_size + 1) {
        current_sum = values[i..(i + window_size)].iter().sum();
        if current_sum > prev_sum {
            increases += 1;
        }
        prev_sum = current_sum;
    }
    println!("Number of increases (window version): {:?}", increases);
}
