use std::collections::VecDeque;

use rand::Rng;

use crate::atsp::ATSP;
use crate::operation;
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
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        utils::randomize_by_swaps(solution, &mut self.rng);
        let new_cost = instance.cost_of_solution(solution);
        ctx.evaluations += 1;
        ctx.current_cost = new_cost;
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
    op_flags: u32,
}

impl RandomWalkExplorer {
    pub fn new(seed: u64, max_time_ns: u64, op_flags: u32) -> RandomWalkExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomWalkExplorer {
            rng,
            max_time_ns,
            time_start: std::time::Instant::now(),
            op_flags,
        }
    }
}

impl Explorer for RandomWalkExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        let op =
            operation::random_operation(&mut self.rng, instance.dimension as u16, self.op_flags);
        let cost_change = op.evaluate(solution, instance);
        ctx.current_cost += cost_change;
        op.apply(solution);
        ctx.evaluations += 1;
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
    pub fn new(seed: u64, num_nodes: u16, op_flags: u32) -> GreedySearchExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        GreedySearchExplorer {
            rng,
            stop: false,
            ops: operation::NeighborhoodIterator::new(num_nodes, op_flags).collect(),
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
                ctx.current_cost += op_delta;
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
    op_flags: u32,
}

impl SteepestSearchExplorer {
    pub fn new(seed: u64, op_flags: u32) -> SteepestSearchExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        SteepestSearchExplorer {
            rng,
            stop: false,
            op_flags,
        }
    }
}

impl Explorer for SteepestSearchExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        let mut best_ops: Vec<operation::Operation> = Vec::new();
        let mut best_delta = std::i32::MAX;
        for op in operation::NeighborhoodIterator::new(instance.dimension as u16, self.op_flags) {
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
        ctx.current_cost += best_delta;
    }

    fn stop_condition(&self, _: &Context) -> bool {
        return self.stop;
    }
}

pub struct TabuSearchExplorer {
    op_flags: u32,
    tabu_list: VecDeque<u32>,
    tabu_tenure: u32,
    top_k: u32,
    top_moves: Vec<operation::Operation>,
    stop_flag: bool,
}

impl TabuSearchExplorer {
    pub fn new(op_flags: u32, top_k: u32, tabu_tenure: u32) -> TabuSearchExplorer {
        TabuSearchExplorer {
            op_flags,
            tabu_list: VecDeque::new(),
            top_k,
            tabu_tenure,
            top_moves: Vec::new(),
            stop_flag: false,
        }
    }

    fn build_top_moves(&mut self, instance: &ATSP, solution: &Solution, ctx: &mut Context) {
        let n_it = operation::NeighborhoodIterator::new(solution.order.len() as u16, self.op_flags);
        let mut top_operations_deltas: VecDeque<(i32, operation::Operation)> = VecDeque::new();
        for op in n_it {
            let op_deserialized = operation::Operation::from_int(op);
            let delta = op_deserialized.evaluate(solution, instance);
            ctx.evaluations += 1;
            if (delta >= 0)
                || (!top_operations_deltas.is_empty() && delta >= top_operations_deltas[0].0)
            {
                continue;
            }
            if self.tabu_list.contains(&op) {
                continue;
            }
            let mut inserted = false;
            for i in 0..top_operations_deltas.len() {
                if delta < top_operations_deltas[i].0 {
                    top_operations_deltas.insert(
                        i,
                        (
                            delta,
                            operation::Operation::from_int(op_deserialized.to_int()),
                        ),
                    );
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                top_operations_deltas.push_back((delta, op_deserialized));
            }
            if top_operations_deltas.len() > self.top_k as usize {
                top_operations_deltas.pop_front();
            }
        }
        self.top_moves = top_operations_deltas.into_iter().map(|x| x.1).collect();
        self.stop_flag = self.top_moves.is_empty();
    }

    fn update_tabu_list(&mut self, op: &operation::Operation) {
        self.tabu_list.push_front(op.to_int());
        if self.tabu_list.len() > self.tabu_tenure as usize {
            self.tabu_list.pop_back();
        }
    }
}

impl Explorer for TabuSearchExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        if self.top_moves.is_empty() {
            self.build_top_moves(instance, solution, ctx);
            return;
        }
        let best_op = self.top_moves.pop().unwrap();
        let delta = best_op.evaluate(solution, instance);
        if delta > 0 {
            self.build_top_moves(instance, solution, ctx);
            return;
        }
        self.update_tabu_list(&best_op);
        best_op.apply(solution);
        ctx.current_cost += delta;
    }

    fn stop_condition(&self, _: &Context) -> bool {
        self.stop_flag
    }
}

pub struct SimulatedAnnealingExplorer {
    rng: rand::rngs::StdRng,
    op_flags: u32,
    temperature: f64,
    alpha: f64,
    markov_chain_length: u32,
    tolerance_iterations: u32,
    no_improvement_counter: u32,
    cooldown_counter: u32,
}

impl SimulatedAnnealingExplorer {
    pub fn new(
        seed: u64,
        op_flags: u32,
        temperature: f64,
        alpha: f64,
        markov_chain_length: u32,
    ) -> SimulatedAnnealingExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        SimulatedAnnealingExplorer {
            rng,
            op_flags,
            temperature,
            alpha,
            markov_chain_length,
            tolerance_iterations: 10,
            no_improvement_counter: 0,
            cooldown_counter: 0,
        }
    }
}

impl Explorer for SimulatedAnnealingExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        let op =
            operation::random_operation(&mut self.rng, instance.dimension as u16, self.op_flags);
        let cost_change = op.evaluate(solution, instance);
        ctx.evaluations += 1;
        let accept_probability = if cost_change < 0 {
            1.0
        } else {
            (-(cost_change as f64) / self.temperature).exp()
        };
        let accept = utils::generate_decision(accept_probability, &mut self.rng);
        self.no_improvement_counter += 1;
        if accept {
            self.no_improvement_counter = 0;
            ctx.current_cost += cost_change;
            op.apply(solution);
        }
        if self.cooldown_counter >= self.markov_chain_length {
            self.cooldown_counter = 0;
            self.temperature *= self.alpha;
        }
    }

    fn stop_condition(&self, _: &Context) -> bool {
        return self.no_improvement_counter >= self.tolerance_iterations * self.markov_chain_length;
    }
}
