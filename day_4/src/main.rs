use std::fs;

struct Space {
    grid: Vec<Vec<char>>,
}

impl Space {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn pos(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn remove(&mut self, x: usize, y: usize) {
        self.grid[y][x] = '.';
    }

    fn adjacent_coords(&self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
        // Generate and filter the indices to check
        let mut coords: Vec<(usize, usize)> = Vec::new();

        if self.pos(x, y) != '@' {
            return None;
        }

        for y_adj in -1..=1 {
            for x_adj in -1..=1 {
                let xpos = x as i32 + x_adj;
                let ypos = y as i32 + y_adj;
                if xpos < 0 || xpos >= self.width() as i32 {
                    continue;
                }
                if ypos < 0 || ypos >= self.height() as i32 {
                    continue;
                }
                if xpos as usize == x && ypos as usize == y {
                    continue;
                }
                if self.pos(xpos as usize, ypos as usize) == '@' {
                    coords.push((xpos as usize, ypos as usize));
                }
            }
        }
        Some(coords)
    }

    fn num_adjacent(&self, x: usize, y: usize) -> Option<usize> {
        match self.adjacent_coords(x, y) {
            Some(val) => Some(val.len()),
            None => None,
        }
    }
}

fn main() {
    // Read the input
    let data = fs::read_to_string("input.txt").unwrap();
    let grid_lines: Vec<String> = data.split("\n")
                                      .collect::<Vec<&str>>()
                                      .iter()
                                      .map(|&val| val.to_string())
                                      .collect();
    let mut grid_data: Vec<Vec<char>> = Vec::new();
    for i in grid_lines {
        let tmp: Vec<char> = i.chars().collect();
        grid_data.push(tmp);
    }
    let mut space = Space { grid: grid_data };

    // Part 1
    let mut total = 0;
    for y in 0..space.height() {
        for x in 0..space.width() {
            if let Some(val) = space.num_adjacent(x, y) {
                total = if val <= 3 {total + 1} else {total};
            }
        }
    }
    println!("Part 1: {}", total);

    // Part 2
    let mut removed: usize = 0;
    loop {
        // Get a list of all removable rolls
        let mut removable: Vec<(usize, usize)> = Vec::new();
        for y in 0..space.height() {
            for x in 0..space.width() {
                if let Some(val) = space.num_adjacent(x, y) {
                    if val <= 3 {
                        removable.push((x, y));
                    }
                }
            }
        }

        // Count and remove all removable rolls
        removed += removable.len();
        for roll in removable.iter() {
            space.remove(roll.0, roll.1);
        }

        // If we didn't remove any rolls, we're done
        if removable.len() == 0 {
            break;
        }
    }
    println!("Part 2: {}", removed)

}
