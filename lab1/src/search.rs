use crate::atsp::ATSP;
use crate::solution::Solution;

#[derive(Debug, Clone)]
pub struct Context {
    pub iterations: u32,
    pub evaluations: u32,
}

impl Context {
    fn new() -> Self {
        Context {
            iterations: 0,
            evaluations: 0,
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
    fn explore(&mut self, instance: &ATSP, solution: &Solution, context: &mut Context) -> Solution;

    fn stop_condition(&self, ctx: &Context) -> bool;
}

impl Explorer for Box<dyn Explorer> {
    fn explore(&mut self, instance: &ATSP, solution: &Solution, context: &mut Context) -> Solution {
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
}

impl<'a, T: Initializer, U: Explorer> SearchAlgorithm<'a, T, U> {
    pub fn new(instance: &'a ATSP, initializer: &'a mut T, explorer: &'a mut U) -> Self {
        SearchAlgorithm {
            instance,
            initializer,
            explorer,
        }
    }

    pub fn run(&mut self) -> (Solution, Context) {
        let mut solution = self.initializer.initialize(self.instance);
        let mut ctx = Context::new();
        let mut stop_alg = false;
        while !stop_alg {
            solution = self.explorer.explore(self.instance, &solution, &mut ctx);
            stop_alg = self.explorer.stop_condition(&ctx);
            ctx.iterations += 1;
        }

        (solution, ctx)
    }
}
