use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").expect("Invalid file path");
    let lines = file_contents.lines();

    part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: std::str::Lines<'_>) {
    let mut result = 0;

    for line in lines {
        let mut two_string = String::new();

        let numbers: String = line
            .chars()
            .filter(|&x| x >= '0' && x <= '9')
            .collect();

        two_string.push(numbers.chars().nth(0).unwrap());
        two_string.push(numbers.chars().last().unwrap());
        result += two_string.parse::<i32>().unwrap();
    }

    println!("{}", result);
}

fn part2(lines: std::str::Lines<'_>) {
    let mut result = 0;

    for line in lines {
        let mut collection = String::new();

        let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let numbers_chars = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for i in 0..line.len() {
            let current_char = line.chars().nth(i).unwrap();

            if current_char >= '0' && current_char <= '9' {
                collection.push(current_char);
            }
            else if current_char == 'o' || current_char == 't' || current_char == 'f' || current_char == 's' || current_char == 'e' || current_char == 'n' {
                for j in 0..numbers.len() {
                    let number = numbers[j];
                    if i + number.len() <= line.len() && (*number).eq(&line[i..i+number.len()]) {
                        collection.push(numbers_chars[j]);
                        break;
                    }
                }
            }
        }

        let mut two_string = String::new();
        two_string.push(collection.chars().nth(0).unwrap());
        two_string.push(collection.chars().last().unwrap());
        result += two_string.parse::<i32>().unwrap();
    }

    println!("{}", result);
}
