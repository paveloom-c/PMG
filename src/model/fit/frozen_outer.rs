//! Outer optimization problem with a frozen parameter

extern crate alloc;

use super::outer::{Output, Param};
use super::OuterOptimizationProblem;
use super::{Objects, Params};
use alloc::rc::Rc;
use core::cell::RefCell;

use core::fmt::{Debug, Display};
use core::iter::Sum;

use anyhow::Result;
use argmin::core::{ArgminFloat, CostFunction, Gradient};
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use finitediff::FiniteDiff;
use num::Float;
use numeric_literals::replace_float_literals;

/// A problem for the outer optimization, but with a frozen parameter
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub(super) struct FrozenOuterOptimizationProblem<'a, F> {
    pub(super) index: usize,
    pub(super) param: F,
    pub(super) objects: &'a Objects<F>,
    pub(super) params: &'a Params<F>,
    pub(super) par_pairs: &'a Rc<RefCell<Vec<(F, F, F)>>>,
}

impl<'a, F> CostFunction for FrozenOuterOptimizationProblem<'a, F>
where
    F: Float
        + Debug
        + Default
        + Display
        + Sync
        + Send
        + Sum
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
    type Param = Param<F>;
    type Output = Output<F>;

    #[allow(clippy::as_conversions)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        // Create an outer problem
        let outer_problem = OuterOptimizationProblem {
            objects: self.objects,
            params: self.params,
            par_pairs: self.par_pairs,
        };
        // Prepare the parameter vector
        let mut new_p = p.clone();
        new_p.insert(self.index, self.param);
        // Compute the cost
        outer_problem.inner_cost(&new_p, false)
    }
}

impl<'a, F> Gradient for FrozenOuterOptimizationProblem<'a, F>
where
    F: Float
        + Debug
        + Default
        + Display
        + Sync
        + Send
        + Sum
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
    type Param = Vec<F>;
    type Gradient = Vec<F>;

    #[allow(clippy::unwrap_used)]
    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient> {
        Ok((*p).central_diff(&|x| self.cost(x).unwrap().to_f64().unwrap()))
    }
}