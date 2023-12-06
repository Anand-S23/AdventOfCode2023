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
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id = parts[0].split(" ").nth(1).unwrap().parse::<i32>().unwrap();

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        let games: Vec<&str> = parts[1].split("; ").collect();
        for game in games {
            let pulls: Vec<&str> = game.split(", ").collect();
            for pull in pulls {
                let single_color: Vec<&str> = pull.split(" ").collect();
                let color = single_color[1];
                let count = single_color[0].parse::<i32>().unwrap();

                if color == "red" && count > r {
                    r = count;
                } else if color == "green" && count > g {
                    g = count;
                } else if color == "blue" && count > b {
                    b = count; 
                }
            }
        }
        
        if r <= 12 && g <= 13 && b <= 14 {
            result += game_id;
        }
    }

    println!("{}", result);
}


fn part2(lines: std::str::Lines<'_>) {
    let mut result = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        let games: Vec<&str> = parts[1].split("; ").collect();
        for game in games {
            let pulls: Vec<&str> = game.split(", ").collect();
            for pull in pulls {
                let single_color: Vec<&str> = pull.split(" ").collect();
                let color = single_color[1];
                let count = single_color[0].parse::<i32>().unwrap();

                if color == "red" && count > r {
                    r = count;
                } else if color == "green" && count > g {
                    g = count;
                } else if color == "blue" && count > b {
                    b = count; 
                }
            }
        }

        result += r * g * b;
    }

    println!("{}", result);
}

