use std::fs;

#[derive(Debug)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    fn in_bounds(&self, val: u64) -> bool {
        (val >= self.lower) && (val <= self.upper)
    }

    fn contains_range(&self, range: &Range) -> bool {
        (self.lower <= range.lower) && (self.upper >= range.upper)
    }

    fn num_valid_ids(&self) -> u64 {
        self.upper - self.lower + 1
    }

    fn snip(r: &Range, r_snip: &Range) -> Vec<Range> {
        // No overlap, do nothing
        if (r_snip.upper < r.lower) || (r_snip.lower > r.upper) {
            return vec![
                Range { lower: r.lower, upper: r.upper }
            ];
        }
        
        // Ranges are the same, or r_snip fully contains r
        if (r.lower >= r_snip.lower) && (r.upper <= r_snip.upper) {
            return vec![];
        }

        // r_snip overlaps r on the left
        if (r_snip.lower <= r.lower) && (r_snip.upper >= r.lower) && (r_snip.upper <= r.upper) {
            return vec![
                Range { lower: r_snip.upper + 1, upper: r.upper },
            ];
        }

        // r_snip overlaps r on the right
        if (r_snip.lower >= r.lower) && (r_snip.lower <= r.upper) && (r_snip.upper >= r.upper) {
            return vec![
                Range { lower: r.lower, upper: r_snip.lower - 1},
            ];
        }

        // r completely contains r_snip
        if (r_snip.lower >= r.lower) && (r_snip.upper <= r.upper) {
            return vec![
                Range { lower: r.lower, upper: r_snip.lower - 1 },
                Range { lower: r_snip.upper + 1, upper: r.upper },
            ]
        }

        Vec::new()
    }
}

#[derive(Debug)]
struct RangeCollection {
    ranges: Vec<Range>,
}

impl RangeCollection {
    fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    fn in_ranges(&self, val: u64) -> bool {
        for i in self.ranges.iter() {
            if i.in_bounds(val) {
                return true;
            }
        }
        false
    }

    fn get_lower(&self) -> u64 {
        let mut result: u64 = self.ranges[0].lower;
        for i in self.ranges.iter() {
            result = if i.lower < result { i.lower } else { result };
        }
        result
    }

    fn get_upper(&self) -> u64 {
        let mut result: u64 = self.ranges[0].upper;
        for i in self.ranges.iter() {
            result = if i.upper > result { i.upper } else { result };
        }
        result
    }

    fn num_valid_ids(&self) -> u64 {
        let mut total: u64 = 0;
        for i in self.ranges.iter() {
            total += i.num_valid_ids();
        }
        total
    }

    fn merge_ranges(r1: Range, r2: Range) -> Vec<Range> {
        // If one completely contains the other, that's our new range
        if r1.contains_range(&r2) {
            return vec![r1];
        }
        if r2.contains_range(&r1) {
            return vec![r2];
        }

        // If r2 is on the right of r1, merge where they overlap/are continuous
        if ((r1.lower-1)..=(r1.upper+1)).contains(&r2.lower) {
            return vec![Range { lower: r1.lower, upper: r2.upper }];
        }

        // If r2 is on the left of r1, merge where they overlap/are continuous
        if ((r1.lower-1)..=(r1.upper+1)).contains(&r2.upper) {
            return vec![Range { lower: r2.lower, upper: r1.upper }];
        }

        // No overlap, return the original ranges
        vec![r1, r2]
    }

    fn snip(&mut self, r_snip: &Range) {
        let mut new_ranges: Vec<Range> = Vec::new();
        for range in self.ranges.iter() {
            new_ranges.append(&mut Range::snip(range, &r_snip));
        }
        self.ranges = new_ranges;
    }
}

fn main() {
    // Load the input
    let data = fs::read_to_string("input.txt").unwrap();
    let data: Vec<String> = data.split("\n\n")
                                .map(|val| val.to_string())
                                .collect::<Vec<String>>();

    // Parse the ranges
    let mut ranges: RangeCollection = RangeCollection { ranges: Vec::new() };
    let range_strs: Vec<String> = data[0].split("\n")
                                         .map(|val| val.to_string())
                                         .collect::<Vec<String>>();
    for range_str in range_strs {
        let bounds: Vec<u64> = range_str.split("-")
                                        .map(|val| val.to_string().parse().unwrap())
                                        .collect();
        ranges.add_range(Range { upper: bounds[1], lower: bounds[0] });
    }

    // Parse the IDs
    let ids: Vec<u64> = data[1].split("\n")
                               .map(|val| val.to_string().parse().unwrap())
                               .collect::<Vec<u64>>();
    
    // Part 1
    let mut total: u32 = 0;
    for id in ids {
        total = if ranges.in_ranges(id) { total + 1 } else { total };
    }
    println!("Part 1: {}", total);

    // Part 2
    let mut initial_range = RangeCollection { ranges: vec![Range { lower: ranges.get_lower(), upper: ranges.get_upper() }] };
    let initial_num_ids = initial_range.num_valid_ids();
    for range in ranges.ranges.iter() {
        initial_range.snip(range);
    }
    let final_num_ids = initial_range.num_valid_ids();
    let num_valid_ids = initial_num_ids - final_num_ids;
    println!("Part 2: {}", num_valid_ids);
}
