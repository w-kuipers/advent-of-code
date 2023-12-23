use std::fs;

fn main() {
    println!("Day 3 - Part 1: {}", part1());
}

#[derive(Debug)]
struct Number {
    number: u32,
    line: usize,
    index_range: [usize; 2],
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
                    numbers.push(Number {
                        number: num,
                        line: y,
                        index_range: [lx, rx],
                    });
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
    let lines: Vec<String> = input_data.lines().map(String::from).collect();
    let numbers = get_numbers(input_data);

    for (i, number) in numbers.iter().enumerate() {
        // println!("{}, - {:?}", number.number, number.index_range);
        if number.line > 0 {
            println!("{}", lines[number.line - 1]);
        }
    }

    return answer;
}
