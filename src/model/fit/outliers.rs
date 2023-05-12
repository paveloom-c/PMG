//! Outliers

use super::compute_relative_discrepancy;
use super::Model;

use core::fmt::Debug;

use anyhow::{Context, Result};
use argmin::core::{ArgminFloat, CostFunction, Executor, State};
use argmin::solver::brent::BrentRoot;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use itertools::izip;
use num::Float;
use numeric_literals::replace_float_literals;

/// A problem for computing appropriate `k`
struct KProblem<F> {
    /// Number of the objects
    n: F,
    /// Significance level
    alpha: F,
}

impl<F> CostFunction for KProblem<F>
where
    F: Float + Debug + Default,
{
    type Param = F;
    type Output = F;

    // Find the reduced parallax
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        let p_f64 = p.to_f64().unwrap();
        let phi_f64 = libm::erf(p_f64 / 2.0.sqrt());
        let phi = F::from(phi_f64).unwrap();
        let cost = (F::one() - phi) * self.n - self.alpha;
        Ok(cost)
    }
}

impl<F> Model<F>
where
    F: Float
        + Debug
        + Default
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
{
    /// Check the estimates of the parameters for discrepancies
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::pattern_type_mismatch)]
    #[allow(clippy::type_complexity)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn find_outliers(&mut self, l_stroke: usize) -> Result<(Vec<(usize, usize, F)>, F, F)> {
        // Find the appropriate coefficient
        let kappa = {
            let problem = KProblem {
                n: F::from(self.count_non_outliers()).unwrap(),
                alpha: 1.,
            };
            let init_param = 2.8;
            let solver = BrentRoot::new(2., 5., 1e-15);
            let res = Executor::new(problem, solver)
                .configure(|state| state.param(init_param).max_iters(1000))
                .timer(false)
                .run()
                .with_context(|| "Couldn't solve the `k` problem")?;
            *res.state().get_best_param().unwrap()
        };

        let k_005 = {
            let problem = KProblem {
                n: F::from(self.count_non_outliers()).unwrap(),
                alpha: 0.05,
            };
            let init_param = 3.4;
            let solver = BrentRoot::new(2., 5., 1e-15);
            let res = Executor::new(problem, solver)
                .configure(|state| state.param(init_param).max_iters(1000))
                .timer(false)
                .run()
                .with_context(|| "Couldn't solve the `k` problem")?;
            *res.state().get_best_param().unwrap()
        };

        // Use this coefficient to check the discrepancies
        let n_nonblacklisted = self.count_non_outliers();
        let mut rel_discrepancies = Vec::with_capacity(n_nonblacklisted);
        let mut all_outliers = Vec::new();
        for m in 0..4 {
            // Compute the discrepancies
            for (i, (object, triples)) in
                izip!(self.objects.borrow().iter(), self.triples.borrow().iter()).enumerate()
            {
                if !object.outlier {
                    let triple = &triples[m];
                    let rel_discrepancy = compute_relative_discrepancy(triple);
                    rel_discrepancies.push((i, rel_discrepancy));
                }
            }

            // Find the outliers
            let mut outliers: Vec<&(usize, F)> = rel_discrepancies
                .iter()
                .filter(|(_, rel_discrepancy)| *rel_discrepancy > kappa)
                .collect();

            if !outliers.is_empty() {
                // Sort the outliers, the smallest discrepancy first
                outliers.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

                // Remove L' outliers if they're small enough
                let len = l_stroke.min(outliers.len());
                for j in (0..len).rev() {
                    let (_, rel_discrepancy) = outliers[j];
                    if *rel_discrepancy <= k_005 {
                        outliers.swap_remove(j);
                    }
                }

                // Mark the rest as outliers
                let mut objects = self.objects.borrow_mut();
                for (i, rel_discrepancy) in &outliers {
                    objects[*i].outlier = true;

                    // Save the outliers for logging
                    all_outliers.push((m, *i, *rel_discrepancy));
                }
            }

            rel_discrepancies.clear();
        }

        // Return all outliers for logging
        Ok(((all_outliers), kappa, k_005))
    }
}
