use std::fs;

struct Knob {
    setting: i32,
    zero_count: u32,
    zero_stop_count: u32,
}

#[derive(Debug)]
struct Adjustment {
    direction: Direction,
    amount: u32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Knob {
    fn turn(&mut self, adjustment: &Adjustment) {
        let increment: i32 = match adjustment.direction {
            Direction::Left => -1,
            Direction::Right => 1
        };
        for _ in 0..adjustment.amount {
            self.setting += increment;
            self.setting = if self.setting < 0 {99} else {self.setting};
            self.setting = if self.setting > 99 {0} else {self.setting};
            self.zero_count = if self.setting == 0 {self.zero_count + 1} else {self.zero_count};
        }
        self.zero_stop_count = if self.setting == 0 {self.zero_stop_count + 1} else {self.zero_stop_count};
    }
}

fn main() {
    // Read the input and convert it into direction/amount
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.split("\n").collect();
    let mut adjustments: Vec<Adjustment> = Vec::new();

    for i in lines.iter() {
        let adjustment = Adjustment {
            direction: if i.chars().nth(0).unwrap() == 'L' {Direction::Left} else {Direction::Right},
            amount: i[1..].to_string().parse::<u32>().unwrap(),
        };
        adjustments.push(adjustment);
    }

    // Perform adjustments on the knob
    let mut knob = Knob {
        setting: 50,
        zero_count: 0,
        zero_stop_count: 0,
    };

    for i in adjustments.iter() {
        knob.turn(i);
    }

    println!("Part 1: {}", knob.zero_stop_count);
    println!("Part 2: {}", knob.zero_count);
}
