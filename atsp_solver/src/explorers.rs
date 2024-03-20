use rand::Rng;

use crate::atsp::ATSP;
use crate::operation::{self, NeighborhoodIterator};
use crate::search::{Context, Explorer};
use crate::solution::Solution;
use crate::utils;

pub struct PassThroughExplorer {}

impl Explorer for PassThroughExplorer {
    fn explore(&mut self, _: &ATSP, _: &mut Solution, _: &mut Context) {}

    fn stop_condition(&self, _: &Context) -> bool {
        true
    }
}

pub struct RandomExplorer {
    rng: rand::rngs::StdRng,
    max_time_ns: u64,
    time_start: std::time::Instant,
}

impl RandomExplorer {
    pub fn new(seed: u64, max_time_ns: u64) -> RandomExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomExplorer {
            rng,
            max_time_ns,
            time_start: std::time::Instant::now(),
        }
    }
}

impl Explorer for RandomExplorer {
    fn explore(&mut self, _: &ATSP, solution: &mut Solution, _: &mut Context) {
        utils::randomize_by_swaps(solution, &mut self.rng);
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        let elapsed = self.time_start.elapsed().as_nanos();
        elapsed >= self.max_time_ns as u128 || ctx.iterations >= std::u32::MAX
    }
}

pub struct RandomWalkExplorer {
    rng: rand::rngs::StdRng,
    max_time_ns: u64,
    time_start: std::time::Instant,
}

impl RandomWalkExplorer {
    pub fn new(seed: u64, max_time_ns: u64) -> RandomWalkExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomWalkExplorer {
            rng,
            max_time_ns,
            time_start: std::time::Instant::now(),
        }
    }
}

impl Explorer for RandomWalkExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, _: &mut Context) {
        let op = operation::random_operation(&mut self.rng, instance.dimension as u16);
        op.apply(solution);
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        let elapsed = self.time_start.elapsed().as_nanos();
        elapsed >= self.max_time_ns as u128 || ctx.iterations >= std::u32::MAX
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
        for op in self.ops.iter() {
            let op_deserialized = operation::Operation::from_int(op.to_owned());
            let op_delta = op_deserialized.evaluate(solution, instance);
            ctx.evaluations += 1;
            if op_delta < 0 {
                op_deserialized.apply(solution);
                ctx.steps += 1;
                return;
            }
        }
        self.stop = true;
    }

    fn stop_condition(&self, _: &Context) -> bool {
        return self.stop;
    }
}

pub struct SteepestSearchExplorer {
    rng: rand::rngs::StdRng,
    stop: bool,
}

impl SteepestSearchExplorer {
    pub fn new(seed: u64) -> SteepestSearchExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        SteepestSearchExplorer { rng, stop: false }
    }
}

impl Explorer for SteepestSearchExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        let mut best_ops: Vec<operation::Operation> = Vec::new();
        let mut best_delta = std::i32::MAX;
        for op in NeighborhoodIterator::new(instance.dimension as u16) {
            let op_deserialized = operation::Operation::from_int(op);
            let op_delta = op_deserialized.evaluate(solution, instance);
            ctx.evaluations += 1;
            if op_delta >= 0 || op_delta > best_delta {
                continue;
            }
            if op_delta < best_delta {
                best_delta = op_delta;
                best_ops.clear();
            }
            best_ops.push(op_deserialized);
        }
        self.stop = best_delta >= 0;
        if self.stop {
            return;
        }
        let sampled_idx = self.rng.gen_range(0..best_ops.len());
        best_ops[sampled_idx].apply(solution);
    }

    fn stop_condition(&self, _: &Context) -> bool {
        return self.stop;
    }
}
