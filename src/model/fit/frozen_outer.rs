//! Outer optimization problem with a frozen parameter

extern crate alloc;

use super::outer::{Output, Param};
use super::{Objects, OuterOptimizationProblem, Params};
use super::{SigmaOuterOptimizationProblem, Triples};
use crate::utils::FiniteDiff;
use alloc::rc::Rc;
use core::cell::RefCell;

use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::path::PathBuf;

use anyhow::Result;
use argmin::core::{ArgminFloat, CostFunction, Gradient};
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use num::Float;
use numeric_literals::replace_float_literals;

/// A problem for the outer optimization, but with a frozen parameter
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub struct FrozenOuterOptimizationProblem<'a, F, FN>
where
    FN: Fn(F, &[F]) -> F,
{
    pub disable_inner: bool,
    pub l_stroke: usize,
    pub index: usize,
    pub param: F,
    /// Allow to compute the frozen parameter
    /// from the free (N - 1) parameters
    pub compute_param: FN,
    pub objects: &'a Objects<F>,
    pub params: &'a Params<F>,
    pub fit_params: &'a Params<F>,
    pub triples: &'a Rc<RefCell<Vec<Triples<F>>>>,
    pub output_dir: &'a PathBuf,
}

impl<'a, F, FN> CostFunction for FrozenOuterOptimizationProblem<'a, F, FN>
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
    Vec<F>: FiniteDiff<F>,
    FN: Fn(F, &[F]) -> F,
{
    type Param = Param<F>;
    type Output = Output<F>;

    #[allow(clippy::as_conversions)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::print_stderr)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        if self.l_stroke == 3 {
            // Create an outer problem
            let outer_problem = OuterOptimizationProblem {
                disable_inner: self.disable_inner,
                objects: self.objects,
                params: self.params,
                triples: self.triples,
                output_dir: self.output_dir,
            };
            // Prepare the parameter vector
            let mut new_p = p.clone();
            new_p.insert(self.index, self.param);
            // Compute the cost
            outer_problem.cost(&new_p)
        } else {
            // Create an outer problem
            let outer_problem = SigmaOuterOptimizationProblem {
                disable_inner: self.disable_inner,
                objects: self.objects,
                fit_params: self.fit_params,
                triples: self.triples,
                output_dir: self.output_dir,
            };
            // Prepare the parameter vector
            let mut new_p = p.clone();
            new_p.insert(self.index, (self.compute_param)(self.param, &new_p));
            // Compute the cost
            outer_problem.cost(&new_p)
        }
    }
}

impl<'a, F, FN> Gradient for FrozenOuterOptimizationProblem<'a, F, FN>
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
    Vec<F>: FiniteDiff<F>,
    FN: Fn(F, &[F]) -> F,
{
    type Param = Vec<F>;
    type Gradient = Vec<F>;

    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient> {
        Ok((*p).central_diff(&|x| self.cost(x).unwrap(), F::sqrt(F::epsilon())))
    }
}
