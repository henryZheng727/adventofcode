mod day01;

use std::env;

// Usage: `cargo run [fileA] [fileB] [day]`
fn main() {
    let args: Vec<String> = env::args().collect();

    // get the Part A file
    let file_part_a = match args.get(1) {
        Some(name) => name,
        None => return,
    };

    // get the Part B file
    let file_part_b = match args.get(2) {
        Some(name) => name,
        None => return,
    };

    // determine what day to run
    let day: i64 = match args.get(3) {
        Some(name) => name.to_string().parse().unwrap_or_default(),
        None => return,
    };

    // run the day
    match day {
        1 => run(day01::part_a, file_part_a, day01::part_b, file_part_b),
        _ => return,
    }
}

fn run(part_a: fn(&String) -> i64, file_a: &String, part_b: fn(&String) -> i64, file_b: &String) {
    println!("{}", part_a(file_a));
    println!("{}", part_b(file_b));
}
