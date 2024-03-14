use crate::errors::MyError;
use crate::solution::Solution;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ATSP {
    pub name: String,
    pub comment: String,
    pub dimension: usize,
    pub edge_weight_type: String,
    pub edge_weight_format: String,
    pub matrix: Vec<Vec<i32>>,
}

impl ATSP {
    pub fn read_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut name = String::new();
        let mut comment = String::new();
        let mut dimension = 0;
        let mut edge_weight_type = String::new();
        let mut edge_weight_format = String::new();
        let mut matrix = Vec::new();
        let mut read_matrix = false;

        let mut current_row = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("NAME:") {
                name = line.split(":").nth(1).unwrap().trim().to_string();
            } else if line.starts_with("COMMENT:") {
                comment = line.split(":").nth(1).unwrap().trim().to_string();
            } else if line.starts_with("DIMENSION:") {
                dimension = line.split(":").nth(1).unwrap().trim().parse()?;
            } else if line.starts_with("EDGE_WEIGHT_TYPE:") {
                edge_weight_type = line.split(":").nth(1).unwrap().trim().to_string();
            } else if line.starts_with("EDGE_WEIGHT_FORMAT:") {
                edge_weight_format = line.split(":").nth(1).unwrap().trim().to_string();
            } else if line.starts_with("EDGE_WEIGHT_SECTION") {
                read_matrix = true;
            } else if line == "EOF" {
                break;
            } else if read_matrix {
                let values: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap_or(9999))
                    .collect();
                current_row.extend(values);
                if current_row.len() == dimension {
                    matrix.push(current_row);
                    current_row = Vec::new();
                }
            }
        }

        if !current_row.is_empty() {
            matrix.push(current_row);
        }

        Ok(Self {
            name,
            comment,
            dimension,
            edge_weight_type,
            edge_weight_format,
            matrix,
        })
    }

    pub fn display(&self, with_matrix: bool) {
        println!("Name: {}", self.name);
        println!("Comment: {}", self.comment);
        println!("Dimension: {}", self.dimension);
        println!("Edge Weight Type: {}", self.edge_weight_type);
        println!("Edge Weight Format: {}", self.edge_weight_format);

        // Added ommit as large matrix is too big to meaningfully display
        if !with_matrix {
            return;
        }
        println!("\nMatrix:");
        for row in &self.matrix {
            print!(
                "{}\n",
                row.iter()
                    .map(|&val| format!("{:4}", val))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }

    pub fn is_solution_valid(&self, solution: &Solution) -> Result<(), MyError> {
        if solution.dimension != self.dimension {
            return Err(MyError::DimensionMismatch);
        }
        if solution.order.iter().cloned().collect::<HashSet<_>>().len() != self.dimension {
            return Err(MyError::LengthMismatch);
        }
        if !solution
            .order
            .iter()
            .all(|&x| x >= 0 && x < self.dimension.try_into().unwrap())
        {
            return Err(MyError::OutOfRange);
        }
        Ok(())
    }

    pub fn cost_of_solution(&self, solution: &Solution) -> i32 {
        let mut cost = 0;
        for i in 0..self.dimension {
            cost += self.matrix[solution.order[i] as usize]
                [solution.order[(i + 1) % self.dimension] as usize];
        }
        cost
    }

    // TODO: Implement this as the Initializer (to be paired up with empty explorer to get pure heuristic solution)
    pub fn nearest_neigbor(&self) -> Solution {
        let mut visited = vec![false; self.dimension];
        let mut order = vec![0; self.dimension];
        let mut current = 0;
        visited[current] = true;
        for i in 1..self.dimension {
            let mut next = 0;
            let mut min_cost = 9999;
            for j in 0..self.dimension {
                if !visited[j] && self.matrix[current][j] < min_cost {
                    next = j;
                    min_cost = self.matrix[current][j];
                }
            }
            order[i] = next as u32;
            visited[next] = true;
            current = next;
        }
        Solution::new(&order).unwrap()
    }
}
