use crate::atsp::ATSP;
use crate::operation::{self, NeighborhoodIterator};
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

pub struct GreedySearchExplorer {
    rng: rand::rngs::StdRng,
    stop: bool,
    ops: Vec<u32>,
}

impl GreedySearchExplorer {
    pub fn new(seed: u64, num_nodes: u16) -> GreedySearchExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        GreedySearchExplorer {
            rng,
            stop: false,
            ops: NeighborhoodIterator::new(num_nodes).collect(),
        }
    }
}

impl Explorer for GreedySearchExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        utils::shuffle(&mut self.ops, &mut self.rng);
        let mut best_op: Option<operation::Operation> = None;
        let mut best_cost = 0;
        // TODO: fix infinite loop
        for op in self.ops.iter() {
            let op_deserialized = operation::Operation::from_int(op.to_owned());
            let op_cost = op_deserialized.evaluate(solution, instance);
            if op_cost < best_cost {
                best_op = Some(op_deserialized);
                best_cost = op_cost;
            }
            ctx.evaluations += 1;
        }
        println!("Best cost: {}", best_cost);
        match best_op {
            Some(op) => {
                op.apply(solution);
                ctx.iterations += 1;
            }
            None => self.stop = true,
        }
    }

    fn stop_condition(&self, _: &Context) -> bool {
        return self.stop;
    }
}
