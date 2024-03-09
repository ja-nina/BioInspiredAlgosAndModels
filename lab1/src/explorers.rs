use crate::atsp::ATSP;
use crate::search::{Context, Explorer};
use crate::solution::Solution;
use crate::utils;

pub struct RandomExplorer {
    rng: rand::rngs::StdRng,
    max_iterations: u32,
    best_solution: Option<Solution>,
    best_cost: i32,
}

impl RandomExplorer {
    pub fn new(seed: u64, max_iterations: u32) -> RandomExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomExplorer {
            rng,
            max_iterations,
            best_solution: None,
            best_cost: i32::MAX,
        }
    }
}

impl Explorer for RandomExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) -> Solution {
        let new_solution = utils::randomize_by_swaps(&solution, &mut self.rng);
        let new_cost: i32 = instance.cost_of_solution(&new_solution);
        if new_cost < self.best_cost {
            ctx.steps += 1;
            self.best_solution = Some(new_solution.clone());
            self.best_cost = new_cost;
            return new_solution;
        };
        // TODO: Optimize Random explorer not to clone everything
        self.best_solution.as_ref().unwrap().clone()
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        ctx.iterations >= self.max_iterations
    }
}
