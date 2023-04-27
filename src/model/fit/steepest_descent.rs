//! A copy of the `argmin`'s `SteepestDescent`
//! with a custom termination condition

#![allow(dead_code)]

use core::fmt::Debug;

use argmin::{
    argmin_error_closure,
    core::{
        ArgminFloat, CostFunction, DeserializeOwnedAlias, Error, Executor, Gradient, IterState,
        LineSearch, OptimizationResult, Problem, SerializeAlias, Solver, TerminationReason,
        TerminationStatus, KV,
    },
    float,
};
use argmin_math::{ArgminAdd, ArgminMul};

/// Steepest descent
#[derive(Clone)]
pub struct SteepestDescent<L, F> {
    /// Line search
    linesearch: L,
    /// Tolerance
    tol_cost: F,
}

impl<L, F> SteepestDescent<L, F> {
    /// Initialize the struct
    pub fn new(linesearch: L, tol_cost: F) -> Self {
        SteepestDescent {
            linesearch,
            tol_cost,
        }
    }
}

impl<O, L, P, G, F> Solver<O, IterState<P, G, (), (), F>> for SteepestDescent<L, F>
where
    O: CostFunction<Param = P, Output = F> + Gradient<Param = P, Gradient = G>,
    P: Clone + SerializeAlias + DeserializeOwnedAlias + ArgminAdd<F, P> + Debug,
    G: Clone + SerializeAlias + DeserializeOwnedAlias + ArgminMul<F, P> + Debug,
    L: Clone + LineSearch<P, F> + Solver<O, IterState<P, G, (), (), F>>,
    F: ArgminFloat,
{
    const NAME: &'static str = "Steepest Descent";

    #[allow(clippy::str_to_string)]
    fn next_iter(
        &mut self,
        problem: &mut Problem<O>,
        mut state: IterState<P, G, (), (), F>,
    ) -> Result<(IterState<P, G, (), (), F>, Option<KV>), Error> {
        let param_new = state.take_param().ok_or_else(argmin_error_closure!(
            NotInitialized,
            concat!(
                "`SteepestDescent` requires an initial parameter vector. ",
                "Please provide an initial guess via `Executor`s `configure` method."
            )
        ))?;
        let new_cost = problem.cost(&param_new)?;
        let new_grad = problem.gradient(&param_new)?;

        let direction = new_grad.mul(&(float!(-1.0)));
        self.linesearch.search_direction(direction);

        // Run line search
        let OptimizationResult {
            problem: line_problem,
            state: mut linesearch_state,
            ..
        } = Executor::new(
            problem.take_problem().ok_or_else(argmin_error_closure!(
                PotentialBug,
                "`SteepestDescent`: Failed to take `problem` for line search"
            ))?,
            self.linesearch.clone(),
        )
        .configure(|config| config.param(param_new).gradient(new_grad).cost(new_cost))
        .ctrlc(false)
        .run()?;

        // Get back problem and function evaluation counts
        problem.consume_problem(line_problem);

        Ok((
            state
                .param(
                    linesearch_state
                        .take_param()
                        .ok_or_else(argmin_error_closure!(
                            PotentialBug,
                            "`GradientDescent`: No `param` returned by line search"
                        ))?,
                )
                .cost(linesearch_state.get_cost()),
            None,
        ))
    }

    fn terminate(&mut self, state: &IterState<P, G, (), (), F>) -> TerminationStatus {
        if (state.get_prev_cost() - state.get_cost()).abs() < self.tol_cost {
            return TerminationStatus::Terminated(TerminationReason::SolverConverged);
        }
        TerminationStatus::NotTerminated
    }
}
