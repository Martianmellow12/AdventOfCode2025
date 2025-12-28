use std::fs;

#[derive(Debug)]
struct BeamSet {
    beams: Vec<u64>,
    split_cnt: u64,
    timeline_cnt: u64,
}

impl BeamSet {
    fn new(init_str: &str) -> BeamSet {
        let mut beams: Vec<u64> = Vec::new();
        for c in init_str.chars() {
            beams.push(if c == 'S' {1} else {0});
        }
        BeamSet {
            beams,
            split_cnt: 0,
            timeline_cnt: 0,
        }
    }

    fn step(&mut self, in_str: &str) {
        for idx in 0..self.beams.len() {
            if (self.beams[idx] > 0) && (in_str.chars().nth(idx).unwrap() == '^') {
                self.beams[idx-1] += self.beams[idx];
                self.beams[idx+1] += self.beams[idx];
                self.beams[idx] = 0;
                self.split_cnt += 1;
            }
        }

        // The number of possible timelines is the total number of leaf nodes,
        // i.e., the number of beams per position
        self.timeline_cnt = self.beams.iter().sum();
    }
}

fn main() {
    // Load the input
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.split("\n").collect();
    let mut beamset = BeamSet::new(lines[0]);

    // Solution
    for line in lines[1..].iter() {
        beamset.step(line);
    }
    println!("Part 1: {}", beamset.split_cnt);
    println!("Part 2: {}", beamset.timeline_cnt);
}
