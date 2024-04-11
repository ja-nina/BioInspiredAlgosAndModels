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
        }
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
}

impl<'a, T: Initializer, U: Explorer> SearchAlgorithm<'a, T, U> {
    pub fn new(instance: &'a ATSP, initializer: &'a mut T, explorer: &'a mut U) -> Self {
        SearchAlgorithm {
            instance,
            initializer,
            explorer,
            best_solution: None,
        }
    }

    pub fn run(&mut self) -> (Solution, Context) {
        let mut solution = self.initializer.initialize(self.instance);
        let initial_cost = self.instance.cost_of_solution(&solution);

        self.best_solution = Some(solution.clone());

        let mut ctx = Context::new(initial_cost);
        let mut stop_alg = false;

        while !stop_alg {
            self.explorer
                .explore(self.instance, &mut solution, &mut ctx);

            if ctx.current_cost < ctx.best_cost {
                ctx.iterations_without_improvement = 0;
                self.best_solution = Some(solution.clone());
                ctx.best_cost = ctx.current_cost;
                ctx.steps += 1;
            }

            ctx.iterations += 1;
            ctx.iterations_without_improvement += 1;
            stop_alg = self.explorer.stop_condition(&ctx);
        }

        (self.best_solution.clone().unwrap(), ctx)
    }
}
