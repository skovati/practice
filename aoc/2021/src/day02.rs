use std::str;
use std::fs;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn parse(v: Vec<&str>) -> Command {
        match v[0] {
            "forward" => Command::Forward(v[1].parse::<i32>().unwrap()),
            "down" => Command::Down(v[1].parse::<i32>().unwrap()),
            "up" => Command::Up(v[1].parse::<i32>().unwrap()),
            _ => panic!("Unsupported submarine command"),
        }
    }
}

struct Mission {
    aim: i32,
    depth: i32,
    horiz: i32,
}

impl Mission {
    fn update_no_aim(&mut self, command: Command) {
        match command {
            Command::Forward(amount) => self.horiz += amount,
            Command::Down(amount) => self.depth += amount,
            Command::Up(amount) => self.depth -= amount,
        }
    }

    fn update_with_aim(&mut self, command: Command) {
        match command {
            Command::Forward(amount) => {
                self.horiz += amount;
                self.depth += self.aim * amount;
            },
            Command::Down(amount) => self.aim += amount,
            Command::Up(amount) => self.aim -= amount,
        }
    }
}

fn main() {
    // part 1
    if let Ok(input) = fs::read_to_string("input/day02.txt") {
        let mut mission = Mission {aim: 0, depth: 0, horiz: 0};
        input.lines()
            .map(|l| l.split_whitespace()
                .collect::<Vec<&str>>())
            .for_each(|command| mission.update_no_aim(Command::parse(command)));
        println!("part 1: horiz: {}, depth: {}, product: {}", mission.horiz, mission.depth, mission.horiz * mission.depth);
    }

    // part 2
    if let Ok(input) = fs::read_to_string("input/day02.txt") {
        let mut mission = Mission {aim: 0, depth: 0, horiz: 0};
        input.lines()
            .map(|l| l.split_whitespace()
                .collect::<Vec<&str>>())
            .for_each(|command| mission.update_with_aim(Command::parse(command)));
        println!("part 2: horiz: {}, depth: {}, product: {}", mission.horiz, mission.depth, mission.horiz * mission.depth);
    }
}
