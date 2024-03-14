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
