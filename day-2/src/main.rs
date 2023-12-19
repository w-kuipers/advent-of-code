use std::fs;

fn main() {
    println!("Day 2 - Part 1: {:?}", part1());
}

fn part1() -> i32 {
    let mut answer: i32 = 0;
    let input_data = fs::read_to_string("input.txt").expect("Unable to read file data");

    for line in input_data.lines() {
        let mut split1 = line.split(": ");
        let game_id = split1.next().unwrap().replace("Game ", "");
        let game = split1.next().unwrap();

        let mut green_max = 0;
        let mut blue_max = 0;
        let mut red_max = 0;

        for game_part in game.split("; ") {
            for color in game_part.split(", ") {
                let mut color_split = color.split(" ");
                let amount = color_split.next().unwrap().parse::<i32>().unwrap();
                let color = color_split.next().unwrap();

                if color == "green" && amount > green_max {
                    green_max = amount;
                }
                if color == "blue" && amount > blue_max {
                    blue_max = amount;
                }
                if color == "red" && amount > red_max {
                    red_max = amount;
                }
            }
        }

        if red_max <= 12 && green_max <= 13 && blue_max <= 14 {
            answer += game_id.parse::<i32>().unwrap();
        }
    }

    return answer;
}
