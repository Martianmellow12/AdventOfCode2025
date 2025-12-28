use std::fs;

#[derive(Debug)]
struct NumGroup {
    num_vec: Vec<Vec<u64>>,
    op_vec: Vec<String>,
}

impl NumGroup {
    fn load(mut input: Vec<Vec<String>>) -> NumGroup {
        let op_vec = input.pop().unwrap();
        let mut num_vec: Vec<Vec<u64>> = Vec::new();

        for i in input.iter() {
            num_vec.push(i.iter().map(|str| str.parse::<u64>().unwrap()).collect());
        }

        NumGroup {
            num_vec: num_vec,
            op_vec: op_vec,
        }
    }

    fn operate(numbers: Vec<u64>, op: &str) -> u64 {
        match op {
            "+" => { numbers.iter().fold(0, |acc, &val| acc + val) },
            "*" => { numbers.iter().fold(1, |acc, &val| acc * val) },
            _ => panic!("Unknown operator")
        }
    }

    fn solve(&self) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();
        for idx in 0..self.num_vec[0].len() {
            let mut numbers: Vec<u64> = Vec::new();
            for vec in self.num_vec.iter() {
                numbers.push(vec[idx]);
            }
            results.push(NumGroup::operate(numbers, self.op_vec[idx].as_str()));
        }
        results
    }
}

#[derive(Debug)]
struct NumOpSet {
    num_vec: Vec<u64>,
    op: char,
}

impl NumOpSet {
    fn load(strings: &Vec<String>) -> NumOpSet {
        let mut num_vec: Vec<u64> = Vec::new();

        // Parse the op and first number
        let op = strings[0].chars().nth(strings[0].len()-1).unwrap();
        num_vec.push(strings[0][0..strings[0].len()-1].parse().unwrap());

        // Parse the remaining numbers
        for val in strings[1..].iter() {
            num_vec.push(val.parse().unwrap());
        }
        
        NumOpSet { num_vec, op }
    }

    fn solve(&self) -> u64 {
        match self.op {
            '+' => { self.num_vec.iter().fold(0, |acc, &val| acc + val) },
            '*' => { self.num_vec.iter().fold(1, |acc, &val| acc * val) },
            _ => panic!("Unknown operator")
        }
    }
}

fn main() {
    // Load the input
    let data = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = data.split("\n").map(|str| str.to_string()).collect();
    let mut values: Vec<Vec<String>> = Vec::new();
    for line in lines.iter() {
        let mut result: Vec<String> = line.split(" ").map(|str| str.to_string()).collect();
        result.retain(|str| str != "");
        values.push(result);
    }
    let group = NumGroup::load(values);

    // Part 1
    let solved = group.solve();
    println!("Part 1: {}", solved.iter().sum::<u64>());

    // Part 2
    // Turn each line into a list of numbers, operators, or empty string slices
    let mut queues: Vec<Vec<&str>> = Vec::new();
    for line in lines.iter() {
        let queue: Vec<&str> = line.split("").map(|val| if val == " " {""} else {val}).collect();
        queues.push(queue);
    }

    // Generate a list of empty string slices, first numbers with the op as the last character, and numbers
    let mut opstrs: Vec<String> = Vec::new();
    for idx in 1..queues[0].len() {
        let mut newstr = String::new();
        for queue in queues.iter() {
            if let Some(c) = queue[idx].chars().nth(0) {
                newstr.push(c);
            }
        }
        opstrs.push(newstr);
    }

    // Convert the list into a list of NumOpSets
    let mut sets: Vec<NumOpSet> = Vec::new();
    let mut set_vals: Vec<String> = Vec::new();
    for opstr in opstrs.iter() {
        if opstr == "" {
            sets.push(NumOpSet::load(&set_vals));
            set_vals = Vec::new();
        }
        else {
            set_vals.push(opstr.to_string());
        }
    }
    
    // Solve and sum all sets
    let mut total = 0;
    for i in sets {
        total += i.solve();
    }
    println!("Part 2: {}", total);
}
