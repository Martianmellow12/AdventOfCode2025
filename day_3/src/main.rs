use std::fs;

#[derive(Debug)]
struct Bank {
    batteries: Vec<u32>,
}

impl Bank {
    fn max_power(&self, num_digits: usize) -> u64 {
        let mut chosen_digits: Vec<u32> = Vec::new();
        let mut left_bound: usize = 0;
        let mut right_bound: usize = self.batteries.len() - num_digits;

        for _ in 0..num_digits {
            // Select the next digit
            let max_val = *self.batteries[left_bound..=right_bound].iter().max().unwrap();
            let max_idx = self.batteries[left_bound..=right_bound].iter().position(|&val| val == max_val).unwrap();

            // Add it to the result and adjust the bounds
            chosen_digits.push(max_val);
            left_bound += max_idx + 1;
            right_bound += 1;
        }
        chosen_digits.iter()
                     .map(|&val| val.to_string())
                     .collect::<String>()
                     .parse()
                     .unwrap()
    }
}

fn main() {
    // Load the input from the file
    let data = fs::read_to_string("input.txt").unwrap();
    let bank_strs: Vec<&str> = data.split("\n").collect();
    let mut banks: Vec<Bank> = Vec::new();

    for bank_str in bank_strs.iter() {
        let mut new_bank = Bank { batteries: Vec::new() };
        for idx in 0..bank_str.len() {
            new_bank.batteries.push(bank_str.chars().nth(idx).unwrap().to_digit(10).unwrap());
        }
        banks.push(new_bank);
    }

    // Part 1
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for bank in banks {
        total_p1 += bank.max_power(2);
        total_p2 += bank.max_power(12)
    }
    println!("Part 1: {}", total_p1);
    println!("Part 2: {}", total_p2);
}
