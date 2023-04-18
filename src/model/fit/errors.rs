//! Confidence intervals (standard errors)

extern crate alloc;

use super::FrozenOuterOptimizationProblem;
use super::{Objects, Params};

use alloc::rc::Rc;
use argmin::solver::linesearch::condition::ArmijoCondition;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;

use anyhow::{Context, Result};
use argmin::core::{ArgminFloat, CostFunction, Executor};
use argmin::solver::linesearch::BacktrackingLineSearch;
use argmin::solver::quasinewton::LBFGS;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use finitediff::FiniteDiff;
use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub(super) struct ConfidenceIntervalProblem<'a, F> {
    pub(super) index: usize,
    pub(super) best_outer_cost: F,
    pub(super) objects: &'a Objects<F>,
    pub(super) params: &'a Params<F>,
    pub(super) par_pairs: &'a Rc<RefCell<Vec<(F, F, F)>>>,
}

impl<'a, F> CostFunction for ConfidenceIntervalProblem<'a, F>
where
    F: Float
        + Debug
        + Default
        + Display
        + Sum
        + Sync
        + Send
        + ArgminFloat
        + ArgminL2Norm<F>
        + ArgminSub<F, F>
        + ArgminAdd<F, F>
        + ArgminDot<F, F>
        + ArgminMul<F, F>
        + ArgminZeroLike
        + ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<F, Vec<F>>,
    Vec<F>: ArgminAdd<Vec<F>, Vec<F>>,
    Vec<F>: ArgminAdd<F, Vec<F>>,
    Vec<F>: ArgminMul<F, Vec<F>>,
    Vec<F>: ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminL1Norm<F>,
    Vec<F>: ArgminSignum,
    Vec<F>: ArgminMinMax,
    Vec<F>: ArgminDot<Vec<F>, F>,
    Vec<F>: ArgminL2Norm<F>,
    Vec<F>: FiniteDiff,
{
    type Param = F;
    type Output = F;

    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, param: &Self::Param) -> Result<Self::Output> {
        // Define the problem of the outer optimization with a frozen parameter
        let problem = FrozenOuterOptimizationProblem {
            index: self.index,
            param: *param,
            objects: self.objects,
            params: self.params,
            par_pairs: self.par_pairs,
        };
        let mut init_param = self.params.to_point();
        // Remove the frozen parameter
        init_param.remove(self.index);
        let cond = ArmijoCondition::new(0.5)?;
        let linesearch = BacktrackingLineSearch::new(cond).rho(0.9)?;
        let solver = LBFGS::new(linesearch, 7).with_tolerance_cost(1e-12)?;
        // Find the local minimum in the outer optimization
        let res = Executor::new(problem, solver)
            .configure(|state| state.param(init_param))
            .run()
            .with_context(|| {
                "Couldn't solve the outer optimization problem with a frozen parameter"
            })?;
        let best_inner_cost = res.state().get_best_cost();
        Ok(best_inner_cost - self.best_outer_cost - 0.5)
    }
}
