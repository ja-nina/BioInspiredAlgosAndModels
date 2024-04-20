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
}

impl RandomExplorer {
    pub fn new(seed: u64) -> RandomExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomExplorer { rng }
    }
}

impl Explorer for RandomExplorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, ctx: &mut Context) {
        utils::randomize_by_swaps(solution, &mut self.rng);
        let new_cost = instance.cost_of_solution(solution);
        ctx.evaluations += 1;
        ctx.current_cost = new_cost;
    }

    fn stop_condition(&self, _: &Context) -> bool {
        false
    }
}

pub struct RandomWalkExplorer {
    rng: rand::rngs::StdRng,
    op_flags: u32,
}

impl RandomWalkExplorer {
    pub fn new(seed: u64, op_flags: u32) -> RandomWalkExplorer {
        let rng = rand::SeedableRng::seed_from_u64(seed);
        RandomWalkExplorer { rng, op_flags }
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

    fn stop_condition(&self, _: &Context) -> bool {
        false
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
    rng: rand::rngs::StdRng,
    op_flags: u32,
    tabu_list: VecDeque<u32>,
    patience: u32,
    tabu_tenure: u32,
    elite_percentage: f64,
}

impl TabuSearchExplorer {
    pub fn new(
        seed: u64,
        op_flags: u32,
        patience: u32,
        elite_percentage: f64,
        tabu_tenure: u32,
    ) -> TabuSearchExplorer {
        TabuSearchExplorer {
            rng: rand::SeedableRng::seed_from_u64(seed),
            op_flags,
            tabu_list: VecDeque::new(),
            patience,
            elite_percentage,
            tabu_tenure,
        }
    }

    fn build_top_moves(
        &mut self,
        instance: &ATSP,
        solution: &Solution,
        ctx: &mut Context,
    ) -> Vec<operation::Operation> {
        let mut n_it =
            operation::NeighborhoodIterator::new(instance.dimension as u16, self.op_flags)
                .collect();
        utils::shuffle(&mut n_it, &mut self.rng);

        let subset_size = (self.elite_percentage * n_it.len() as f64).round();
        let elite_size = (self.elite_percentage * subset_size).round();
        let n_it_subset = &n_it[..subset_size as usize];
        let mut top_operations_deltas: VecDeque<(i32, operation::Operation)> = VecDeque::new();

        for op in n_it_subset {
            let op_deserialized = operation::Operation::from_int(op.to_owned());
            let delta = op_deserialized.evaluate(solution, instance);
            ctx.evaluations += 1;

            if (!top_operations_deltas.len() == elite_size as usize)
                && delta >= top_operations_deltas.back().unwrap().0
            {
                continue;
            }

            let insert_index = top_operations_deltas
                .binary_search_by(|&(d, _)| d.cmp(&delta))
                .unwrap_or_else(|x| x);
            top_operations_deltas.insert(
                insert_index,
                (
                    delta,
                    operation::Operation::from_int(op_deserialized.to_int()),
                ),
            );
            if top_operations_deltas.len() > elite_size as usize {
                top_operations_deltas.pop_back();
            }
        }
        top_operations_deltas.into_iter().map(|x| x.1).collect()
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
        let top_moves = self.build_top_moves(instance, solution, ctx);
        let mut selected_op: Option<operation::Operation> = None;
        let mut selected_delta: Option<i32> = None;
        for op in top_moves {
            let delta = op.evaluate(solution, instance);
            let improves_best = delta + ctx.current_cost < ctx.best_cost;
            if self.tabu_list.contains(&op.to_int()) && !improves_best {
                continue;
            }
            selected_op = Some(op);
            selected_delta = Some(delta);
            break;
        }
        let concrete_op = match selected_op {
            Some(op) => {
                self.update_tabu_list(&op);
                op
            }
            None => {
                let op = operation::Operation::from_int(self.tabu_list.back().unwrap().to_owned());
                selected_delta = Some(op.evaluate(solution, instance));
                op
            }
        };
        ctx.evaluations += 1;
        ctx.current_cost += selected_delta.expect("Delta not found");
        concrete_op.apply(solution);
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        ctx.iterations_without_improvement >= self.patience
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
            tolerance_iterations: 15,
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
        let length_condition =
            self.no_improvement_counter >= self.tolerance_iterations * self.markov_chain_length;
        let temperature_condition = self.temperature < 0.01;
        length_condition || temperature_condition
    }
}
