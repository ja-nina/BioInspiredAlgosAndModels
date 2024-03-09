use crate::atsp::ATSP;
use crate::search::{Context, Explorer};
use crate::solution::Solution;
use crate::utils;

pub struct RandomExplorer {
    rng: rand::rngs::StdRng,
    max_iterations: u32,
}

impl RandomExplorer {
    pub fn new(seed: u64, max_iterations: u32) -> RandomExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomExplorer {
            rng,
            max_iterations,
        }
    }
}

impl Explorer for RandomExplorer {
    fn explore(&mut self, _: &ATSP, solution: &Solution, _: &mut Context) -> Solution {
        let mut new_solution = solution.clone();
        utils::randomize_by_swaps(&mut new_solution, &mut self.rng);
        new_solution
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        ctx.iterations >= self.max_iterations
    }
}
