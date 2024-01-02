use std::fs;

fn main() {
    println!("Day 4 - Part 1: {}", part1())
}

fn part1() -> u32 {
    let input_data = fs::read_to_string("input.txt").expect("Unable to read input data");

    let mut answer: u32 = 0;

    for line_raw in input_data.lines() {
        let line = &mut line_raw[8..].split("|");

        let winning_numbers_raw = line.next().unwrap();
        let owned_numbers_raw = line.next().unwrap();

        let mut winning_numbers: Vec<u8> = Vec::new();

        for winning_number in winning_numbers_raw.split(" ") {
            if let Ok(num) = winning_number.parse::<u8>() {
                winning_numbers.push(num);
            }
        }

        let mut points: u32 = 0;
        for owned_number in owned_numbers_raw.split(" ") {
            if let Ok(num) = owned_number.parse::<u8>() {
                if winning_numbers.contains(&num) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points = points * 2;
                    }
                }
            }
        }
        answer += points;
    }

    return answer;
}
