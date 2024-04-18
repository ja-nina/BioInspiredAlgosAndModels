use crate::atsp::ATSP;
use crate::solution::Solution;

#[derive(Debug, Clone)]
pub struct Context {
    pub iterations: u32,
    pub evaluations: u32,
    pub steps: u32,
    pub initial_cost: i32,
    pub current_cost: i32,
    pub best_cost: i32,
    pub iterations_without_improvement: u32,
    pub evaluations_history: Vec<u32>,
    pub cost_history: Vec<i32>,
}

impl Context {
    fn new(initial_cost: i32) -> Self {
        Context {
            iterations: 0,
            evaluations: 0,
            steps: 0,
            initial_cost,
            current_cost: initial_cost,
            best_cost: initial_cost,
            iterations_without_improvement: 0,
            evaluations_history: vec![0],
            cost_history: vec![initial_cost],
        }
    }

    pub fn on_change_best(&mut self) {
        self.steps += 1;
        self.iterations_without_improvement = 0;
        self.best_cost = self.current_cost;
        self.cost_history.push(self.best_cost);
        self.evaluations_history.push(self.evaluations);
    }

    pub fn on_iteration_end(&mut self) {
        self.iterations += 1;
        self.iterations_without_improvement += 1;
    }
}

pub trait Initializer {
    fn initialize(&mut self, instance: &ATSP) -> Solution;
}

impl Initializer for Box<dyn Initializer> {
    fn initialize(&mut self, instance: &ATSP) -> Solution {
        (**self).initialize(instance)
    }
}

pub trait Explorer {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, context: &mut Context);

    fn stop_condition(&self, ctx: &Context) -> bool;
}

impl Explorer for Box<dyn Explorer> {
    fn explore(&mut self, instance: &ATSP, solution: &mut Solution, context: &mut Context) {
        (**self).explore(instance, solution, context)
    }

    fn stop_condition(&self, ctx: &Context) -> bool {
        (**self).stop_condition(ctx)
    }
}

pub struct SearchAlgorithm<'a, T: Initializer, U: Explorer> {
    instance: &'a ATSP,
    initializer: &'a mut T,
    explorer: &'a mut U,
    best_solution: Option<Solution>,
    max_time: i64,
}

impl<'a, T: Initializer, U: Explorer> SearchAlgorithm<'a, T, U> {
    pub fn new(
        instance: &'a ATSP,
        initializer: &'a mut T,
        explorer: &'a mut U,
        max_time: i64,
    ) -> Self {
        SearchAlgorithm {
            instance,
            initializer,
            explorer,
            best_solution: None,
            max_time,
        }
    }

    pub fn run(&mut self) -> (Solution, Context) {
        let time_start = std::time::Instant::now();
        let mut solution = self.initializer.initialize(self.instance);
        let initial_cost = self.instance.cost_of_solution(&solution);

        self.best_solution = Some(solution.clone());

        let mut ctx = Context::new(initial_cost);
        let mut stop_alg = false;

        while !stop_alg {
            self.explorer
                .explore(self.instance, &mut solution, &mut ctx);

            if ctx.current_cost < ctx.best_cost {
                self.best_solution = Some(solution.clone());
                ctx.on_change_best();
            }

            let time_break =
                (self.max_time >= 0) && (time_start.elapsed().as_nanos() >= self.max_time as u128);
            stop_alg = self.explorer.stop_condition(&ctx) || time_break;
            ctx.on_iteration_end();
        }

        (self.best_solution.clone().unwrap(), ctx)
    }
}
