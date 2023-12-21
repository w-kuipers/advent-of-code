use std::fs;

fn main() {
    println!("Day 3 - Part 1: {}", part1());
}

#[derive(Debug)]
struct Number {
    number: u32,
}

fn get_numbers(input_data: String) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for (y, line) in input_data.lines().enumerate() {
        let line_length = line.len();
        let subline = &line[0..];

        let mut lx = 0;

        while lx < line_length {
            let mut rx = line_length - 1;
            while rx > lx {
                let subline = &subline[lx..rx];

                if let Ok(num) = subline.parse::<u32>() {
                    println!("{num} {lx} - {rx}");

                    numbers.push(Number { number: num });
                    lx += subline.len();
                }
                rx -= 1;
            }
            lx += 1;
        }
    }

    return numbers;
}

fn part1() -> i32 {
    let input_data = fs::read_to_string("input-simple.txt").expect("Unable to read input data");

    let mut answer: i32 = 0;

    // Pupulate vector with numbers
    let numbers = get_numbers(input_data);

    // for number in numbers {
    //     println!("{}", number.number);
    // }

    return answer;
}
