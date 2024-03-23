use rand::Rng;

use crate::atsp::ATSP;
use crate::search::Initializer;
use crate::solution::Solution;
use crate::utils;

pub struct RandomInitializer {
    rng: rand::rngs::StdRng,
}

impl RandomInitializer {
    pub fn new(seed: u64) -> RandomInitializer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomInitializer { rng }
    }
}

impl Initializer for RandomInitializer {
    fn initialize(&mut self, instance: &ATSP) -> Solution {
        let cities: Vec<u32> = (0..(instance.dimension as u32)).collect();
        let mut initial_sol = Solution::new(&cities).unwrap();
        utils::randomize_by_swaps(&mut initial_sol, &mut self.rng);
        initial_sol
    }
}

pub struct NearestNeighborInitializer {
    rng: rand::rngs::StdRng,
}

impl NearestNeighborInitializer {
    pub fn new(seed: u64) -> NearestNeighborInitializer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        NearestNeighborInitializer { rng }
    }
}

impl Initializer for NearestNeighborInitializer {
    fn initialize(&mut self, instance: &ATSP) -> Solution {
        let mut visited = vec![false; instance.dimension];
        let mut order = vec![0; instance.dimension];

        let mut current = self.rng.gen_range(0..instance.dimension) as usize;
        visited[current] = true;
        order[0] = current as u32;
        for i in 1..instance.dimension {
            let mut next = 0;
            let mut min_cost = std::i32::MAX;
            for j in 0..instance.dimension {
                if !visited[j] && instance.matrix[current][j] < min_cost {
                    next = j;
                    min_cost = instance.matrix[current][j];
                }
            }
            order[i] = next as u32;
            visited[next] = true;
            current = next;
        }
        Solution::new(&order).unwrap()
    }
}
