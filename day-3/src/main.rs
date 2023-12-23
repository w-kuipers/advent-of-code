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
            let mut rx = line_length;
            while rx > lx {
                let subline = &subline[lx..rx];

                if let Ok(num) = subline.parse::<u32>() {
                    numbers.push(Number {
                        number: num,
                        line: y,
                        index_range: [lx, rx - 1],
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

// Function to check if any adjacent character is symbol (in this case not number and not period)
fn check_if_symbol(line: String, index: usize) -> bool {
    let chars = line.chars();

    // Check if char at exact index is a symbol
    let same = chars.clone().nth(index).unwrap();
    if same.is_numeric() == false && same != '.' {
        return true;
    }

    // Check if char one left of exact index is symbol
    if index > 0 {
        let left = chars.clone().nth(index - 1).unwrap();
        if left.is_numeric() == false && left != '.' {
            return true;
        }
    }

    // Check if char one right of exact index is symbol
    if index < line.len() - 1 {
        let right = chars.clone().nth(index + 1).unwrap();
        if right.is_numeric() == false && right != '.' {
            return true;
        }
    }

    return false;
}

fn part1() -> u32 {
    let input_data = fs::read_to_string("input.txt").expect("Unable to read input data");

    let mut answer: u32 = 0;

    // Pupulate vector with numbers
    let lines: Vec<String> = input_data.lines().map(String::from).collect();
    let numbers = get_numbers(input_data);

    for number in numbers.iter() {
        // Process same line
        'ni: for number_index in number.index_range.iter() {
            let line_chars_above = &lines[number.line];

            if check_if_symbol(String::from(line_chars_above), *number_index) {
                answer += number.number;
                break 'ni;
            }
        }

        // Process line above
        if number.line > 0 {
            'ni: for number_index in number.index_range.iter() {
                let line_chars_above = &lines[number.line - 1];

                if check_if_symbol(String::from(line_chars_above), *number_index) {
                    answer += number.number;
                    break 'ni;
                }
            }
        }

        // Process line below
        if number.line < lines.len() - 1 {
            'ni: for number_index in number.index_range.iter() {
                let line_chars_above = &lines[number.line + 1];

                if check_if_symbol(String::from(line_chars_above), *number_index) {
                    answer += number.number;
                    break 'ni;
                }
            }
        }
    }

    return answer;
}
