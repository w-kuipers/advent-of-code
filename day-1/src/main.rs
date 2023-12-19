use std::fs;

fn main() {
    println!("Day 1 - Part 1: {}", part1());
    println!("Day 1 - Part 2: {}", part2());
}

fn part1() -> i32 {
    let input_data = fs::read_to_string("input.txt").expect("Unable to read file data");

    let mut answer: i32 = 0;

    // Loop through lines
    for line in input_data.lines() {
        let mut gathered = String::from("");
        let combined: String;

        // Get only numerical characters from line
        for char in line.chars() {
            if char.is_numeric() {
                gathered.push(char);
            }
        }

        // If only one char, append it to itself
        if gathered.len() == 1 {
            combined = gathered.clone() + &gathered;

        // If multiple chars, append last to first
        } else {
            combined = String::from(gathered.chars().nth(0).unwrap())
                + &String::from(gathered.chars().last().unwrap());
        }

        // Convert to integer
        let line_value: i32 = combined.parse().unwrap();

        // Add to total
        answer += line_value;
    }

    return answer;
}

fn part2() -> i32 {
    let input_data = fs::read_to_string("input.txt").expect("Unable to read file data");

    let mut answer: i32 = 0;

    // Will be used for comparing
    let numbers = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    for line in input_data.lines() {
        let mut first = None;

        'out: for i in 0..line.len() {
            for (index, num) in numbers.iter().enumerate() {
                // If the current numbers char length is higher then line char length, it can not
                // be in there so no need to proceed
                if i + num.len() > line.len() {
                    continue;
                }

                if line[i..i + num.len()] == **num {
                    first = Some(1 + index / 2);
                    break 'out;
                }
            }
        }
        let Some(first) = first else {
            panic!("Invalid input");
        };

        let mut last = None;

        'out: for i in (0..line.len()).rev() {
            for (index, num) in numbers.iter().enumerate() {
                // If the current numbers char length is higher then line char length, it can not
                // be in there so no need to proceed
                if i + num.len() > line.len() {
                    continue;
                }

                if line[i..i + num.len()] == **num {
                    last = Some(1 + index / 2);
                    break 'out;
                }
            }
        }
        let Some(last) = last else {
            panic!("Invalid input");
        };

        answer += 10 * first as i32 + last as i32;
    }

    return answer;
}
