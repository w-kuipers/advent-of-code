use std::fs;

fn main() {
    println!("Day 4 - Part 1: {}", part1());
    println!("Day 4 - Part 2: {}", part2());
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

#[derive(Debug, Clone)]
struct Card {
    amount: usize,
    wins: usize,
}

fn part2() -> usize {
    let input_data = fs::read_to_string("input.txt").expect("Unable to read input data");

    let mut answer: usize = 0;

    let mut cards: Vec<Card> = Vec::new();

    // Get original cards
    for line_raw in input_data.lines() {
        let mut line = line_raw[8..].split("|");

        let winning_numbers_raw = line.next().unwrap();
        let owned_numbers_raw = line.next().unwrap();

        let mut winning_numbers: Vec<u8> = Vec::new();

        for winning_number in winning_numbers_raw.split(" ") {
            if let Ok(num) = winning_number.parse::<u8>() {
                winning_numbers.push(num);
            }
        }

        let mut owned_numbers: Vec<u8> = Vec::new();

        for owned_number in owned_numbers_raw.split(" ") {
            if let Ok(num) = owned_number.parse::<u8>() {
                if winning_numbers.contains(&num) {
                    owned_numbers.push(num);
                }
            }
        }

        cards.push(Card {
            amount: 1,
            wins: owned_numbers.len(),
        });
    }

    // Get won cards
    let mut c: usize = 0;
    while c < cards.len() {
        if cards[c].wins > 0 {
            for i in 0..cards[c].wins {
                cards[c + (i + 1)].amount += cards[c].amount;
            }
        }
        c += 1;
    }

    // Count cards
    for card in cards {
        answer += card.amount; 
    }

    return answer;
}
