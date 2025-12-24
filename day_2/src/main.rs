use std::fs;

enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    fn is_invalid_pt1(num: u64) -> bool {
        let num_str = num.to_string();
        
        // No odd numbers of digits
        if (num_str.len() % 2) == 1 {
            return false;
        }

        // Split the number in half and compare
        let split_point = num_str.len()/2;
        let a = &num_str[0..split_point];
        let b = &num_str[split_point..];

        a == b
    }

    fn is_invalid_pt2(num: u64) -> bool {
        let num_str = num.to_string();

        // Iterate over each split point
        for i in 1..=num_str.len()/2 {
            let snippet = &num_str[0..i];
            let needed_reps = num_str.len()/snippet.len();

            // If the snippet can't fit, skip checking it
            if (num_str.len() % snippet.len()) != 0 {
                continue;
            }

            // See if the snippet is present as many times as possible
            if num_str.matches(snippet).count() == needed_reps {
                return true;
            }
        }
        false
    }

    fn get_invalid(&self, part: Part) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        for id in self.lower..=self.upper {
            let is_invalid = match part {
                Part::Part1 => Range::is_invalid_pt1(id),
                Part::Part2 => Range::is_invalid_pt2(id),
            };
            if is_invalid {
                result.push(id);
            }
        }
        result
    }
}

fn main() {
    // Load the input
    let mut ranges: Vec<Range> = Vec::new();
    let data = fs::read_to_string("input.txt").unwrap();
    let range_strs: Vec<&str> = data.split(",").collect();
    for i in range_strs.iter() {
        let range_pair: Vec<&str> = i.split("-").collect();
        let range = Range {
            lower: range_pair.get(0)
                             .unwrap()
                             .to_string()
                             .parse::<u64>()
                             .unwrap(),
            upper: range_pair.get(1)
                             .unwrap()
                             .to_string()
                             .parse::<u64>()
                             .unwrap(),
        };
        ranges.push(range);
    }

    // Part 1
    let mut invalid_ids: Vec<u64> = Vec::new();
    for range in ranges.iter() {
        invalid_ids.append(&mut range.get_invalid(Part::Part1));
    }
    let sum: u64 = invalid_ids.iter().sum();
    println!("Part 1: {:?}", sum);

    // Part 2
    let mut invalid_ids: Vec<u64> = Vec::new();
    for range in ranges.iter() {
        invalid_ids.append(&mut range.get_invalid(Part::Part2));
    }
    let sum: u64 = invalid_ids.iter().sum();
    println!("Part 2: {:?}", sum);
}
