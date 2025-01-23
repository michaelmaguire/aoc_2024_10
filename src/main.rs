use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Matrix {
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(data: Vec<Vec<i32>>) -> Self {
        Matrix { data }
    }

    fn from_file(file_path: &str) -> io::Result<Self> {
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut result = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let row: Vec<i32> = line.chars()
                        .filter_map(|c| c.to_digit(10))
                        .map(|d| d as i32)
                        .collect();
            result.push(row);
        }
        Ok(Matrix::new(result))
    }

    fn width(&self) -> usize {
        self.data.first().map_or(0, |row| row.len())
    }
    fn height(&self) -> usize {
        self.data.len()
    }
    fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
    fn print(&self) {
        for row in &self.data {
            for &digit in row {
                print!("{}", digit);
            }
            println!();
        }
    }

    fn trailhead_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for (y, row) in self.data.iter().enumerate() {
            for (x, &digit) in row.iter().enumerate() {
                if digit == 0 {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

}


fn recursively_visit_all_paths_from_trailhead_part1(matrix: &Matrix, trailhead: (usize, usize), trail_score_for_trailhead: &mut u64) {
    let mut stack = Vec::new();
    let mut peak_9_visited: HashSet<(usize,usize)>  = HashSet::new();
    stack.push(trailhead);
    while let Some((x, y)) = stack.pop() {
        let current_value = matrix.data[y][x];
        // Visit all neighbors.
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if matrix.check_bounds(new_x as usize, new_y as usize) {
                let new_value = matrix.data[new_y as usize][new_x as usize];
                if new_value == current_value + 1 {
                    if new_value == 9 {
                        // We found the end of the trail for this trailhead which we have not visited before.
                        if ! peak_9_visited.contains(&(new_x as usize, new_y as usize)) {
                            *trail_score_for_trailhead += 1;
                            peak_9_visited.insert((new_x as usize, new_y as usize));
                        }
                    } else {
                        // We found a possible trail to continue to investigate.
                        stack.push((new_x as usize, new_y as usize));
                    }
                }
            }
        }
    }
}

fn recursively_visit_all_paths_from_trailhead_part2(matrix: &Matrix, trailhead: (usize, usize), trail_rating_for_trailhead: &mut u64) {
    let mut stack = Vec::new();
    stack.push(trailhead);
    while let Some((x, y)) = stack.pop() {
        let current_value = matrix.data[y][x];
        // Visit all neighbors.
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if matrix.check_bounds(new_x as usize, new_y as usize) {
                let new_value = matrix.data[new_y as usize][new_x as usize];
                if new_value == current_value + 1 {
                    if new_value == 9 {
                        *trail_rating_for_trailhead += 1;
                    } else {
                        // We found a possible trail to continue to investigate.
                        stack.push((new_x as usize, new_y as usize));
                    }
                }
            }
        }
    }
}


fn main() {
    println!("Hello, aoc_2024_10!");
    match Matrix::from_file("./src/input.txt") {
        Ok(matrix) => {
            matrix.print();
            let trail_heads = matrix.trailhead_positions();
            //for (ch, pos) in &positions {
            //    println!("{}: {:?}", ch, pos);
            //}

            let mut total_trail_score: u64 = 0;
            let mut total_trail_rating: u64 = 0;
            for trail_head in &trail_heads {
                let mut trail_score_for_trailhead = 0;

                // Examine possible trails from each trailhead.
                recursively_visit_all_paths_from_trailhead_part1(&matrix, *trail_head, &mut trail_score_for_trailhead );
                println!("Trailhead at position: {:?} part 1 trail_score_for_trailhead {}", trail_head, trail_score_for_trailhead);
                total_trail_score += trail_score_for_trailhead;

                let mut trail_rating_for_trailhead = 0;

                recursively_visit_all_paths_from_trailhead_part2(&matrix, *trail_head, &mut trail_rating_for_trailhead );
                println!("Trailhead at position: {:?} part 2 trail_raiting_for_trailhead {}", trail_head, trail_rating_for_trailhead);
                total_trail_rating += trail_rating_for_trailhead;

            }

            println!("total_trail_score: {}", total_trail_score);
            println!("total_trail_rating: {}", total_trail_rating);

        },
        Err(e) => eprintln!("Error reading input file: {}", e),
    }
}