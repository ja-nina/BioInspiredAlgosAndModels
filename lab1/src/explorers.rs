use crate::atsp::ATSP;
use crate::operation;
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
    fn explore(&mut self, _: &ATSP, solution: &mut Solution, _: &mut Context) {
        utils::randomize_by_swaps(solution, &mut self.rng);
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        ctx.iterations >= self.max_iterations
    }
}

pub struct RandomWalkExplorer {
    rng: rand::rngs::StdRng,
    max_iterations: u32,
}

impl RandomWalkExplorer {
    pub fn new(seed: u64, max_iterations: u32) -> RandomWalkExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomWalkExplorer {
            rng,
            max_iterations,
        }
    }
}

impl Explorer for RandomWalkExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, _: &mut Context) {
        let op = operation::random_operation(&mut self.rng, instance.dimension as u16);
        op.apply(solution);
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        ctx.iterations >= self.max_iterations
    }
}
