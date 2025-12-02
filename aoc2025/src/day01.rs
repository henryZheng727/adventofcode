pub fn part_a(file: &String) -> i64 {
    let instructions = std::fs::read_to_string(file).expect("no file");

    let mut password = 0;
    let mut count = 50;

    for line in instructions.lines() {
        let (direction, val_str) = line.split_at(1);
        let val: i32 = val_str.trim().parse().expect("invalid file");

        match direction {
            "L" => count -= val,
            "R" => count += val,
            _ => break,
        }

        if (count % 100) == 0 {
            password += 1;
        }
    }

    return password;
}

pub fn part_b(file: &String) -> i64 {
    return 0;
}
