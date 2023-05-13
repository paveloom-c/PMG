//! Outliers

#![allow(clippy::missing_docs_in_private_items)]

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

struct OneDimensionalProblem<F> {
    /// Number of the objects
    n: usize,
    /// Significance level
    alpha: F,
    /// Raise the probability function to the power of N?
    raise: bool,
}

impl<F> CostFunction for OneDimensionalProblem<F>
where
    F: Float + Debug + Default,
{
    type Param = F;
    type Output = F;

    // Find the reduced parallax
    #[allow(clippy::as_conversions)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        let p_f64 = p.to_f64().unwrap();
        let n = F::from(self.n).unwrap();

        let prob_f64 = libm::erf(p_f64 / 2.0_f64.sqrt());
        let prob = F::from(prob_f64).unwrap();
        let cost = if self.raise {
            F::one() - prob.powi(self.n as i32) - self.alpha
        } else {
            (F::one() - prob) * n - self.alpha
        };
        Ok(cost)
    }
}

/// Four-dimensional problem
struct FourDimensionalProblem<F> {
    /// Number of the objects
    n: usize,
    /// Significance level
    alpha: F,
    /// Raise the probability function to the power of N?
    raise: bool,
}

impl<F> CostFunction for FourDimensionalProblem<F>
where
    F: Float + Debug + Default,
{
    type Param = F;
    type Output = F;

    // Find the reduced parallax
    #[allow(clippy::as_conversions)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        let z = *p;
        let n = F::from(self.n).unwrap();

        let prob = 1. - F::exp(-z / 2.) * (z / 2. + 1.);
        let cost = if self.raise {
            1. - prob.powi(self.n as i32) - self.alpha
        } else {
            (1. - prob) * n - self.alpha
        };
        Ok(cost)
    }
}

pub struct OneDimensionalOutliers<F> {
    /// (m, i, rel_discrepancy)
    pub vec: Vec<(usize, usize, F)>,
    pub kappa: F,
    pub k_005: F,
}

pub struct FourDimensionalOutliers<F> {
    /// (i, summed_rel_discrepancy)
    pub vec: Vec<(usize, F)>,
    pub kappa: F,
    pub k_005: F,
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
    #[allow(clippy::too_many_lines)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn find_outliers(
        &mut self,
        l_stroke: usize,
    ) -> Result<(OneDimensionalOutliers<F>, FourDimensionalOutliers<F>)> {
        let n_nonblacklisted = self.count_non_outliers();

        // Check for outliers using one-dimensional algorithm
        let one_dimensional_outliers = {
            let kappa = {
                let problem = OneDimensionalProblem {
                    n: n_nonblacklisted,
                    alpha: 1.,
                    raise: false,
                };
                let init_param = 2.8;
                let solver = BrentRoot::new(2., 5., 1e-15);
                let res = Executor::new(problem, solver)
                    .configure(|state| state.param(init_param).max_iters(1000))
                    .timer(false)
                    .run()
                    .with_context(|| {
                        "Couldn't find an appropriate `kappa` in the one-dimensional algorithm"
                    })?;
                *res.state().get_best_param().unwrap()
            };

            let k_005 = {
                let problem = OneDimensionalProblem {
                    n: n_nonblacklisted,
                    alpha: 0.05,
                    raise: true,
                };
                let init_param = 3.4;
                let solver = BrentRoot::new(2., 5., 1e-15);
                let res = Executor::new(problem, solver)
                    .configure(|state| state.param(init_param).max_iters(1000))
                    .timer(false)
                    .run()
                    .with_context(|| {
                        "Couldn't find an appropriate `k_005` in the one-dimensional algorithm"
                    })?;
                *res.state().get_best_param().unwrap()
            };

            let mut rel_discrepancies = Vec::with_capacity(n_nonblacklisted);
            let mut all_outliers = Vec::new();

            // For each type of discrepancy
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

                // Find the outliers (for the current type of discrepancy)
                let mut m_outliers: Vec<&(usize, F)> = rel_discrepancies
                    .iter()
                    .filter(|(_, rel_discrepancy)| *rel_discrepancy > kappa)
                    .collect();

                if !m_outliers.is_empty() {
                    // Sort the outliers, the smallest discrepancy first
                    m_outliers.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

                    // Remove L' outliers if they're small enough
                    let len = l_stroke.min(m_outliers.len());
                    for j in (0..len).rev() {
                        let (_, rel_discrepancy) = m_outliers[j];
                        if *rel_discrepancy <= k_005 {
                            m_outliers.swap_remove(j);
                        }
                    }

                    // Mark the rest as outliers
                    let mut objects = self.objects.borrow_mut();
                    for (i, rel_discrepancy) in &m_outliers {
                        objects[*i].outlier = true;

                        // Save the outliers
                        all_outliers.push((m, *i, *rel_discrepancy));
                    }
                }

                rel_discrepancies.clear();
            }

            OneDimensionalOutliers {
                vec: all_outliers,
                kappa,
                k_005,
            }
        };

        // Check for outliers using four-dimensional algorithm
        let four_dimensional_outliers = {
            let kappa = {
                let problem = FourDimensionalProblem {
                    n: n_nonblacklisted,
                    alpha: 1.,
                    raise: false,
                };
                let init_param = 14.86;
                let solver = BrentRoot::new(9., 18., 1e-15);
                let res = Executor::new(problem, solver)
                    .configure(|state| state.param(init_param).max_iters(1000))
                    .timer(false)
                    .run()
                    .with_context(|| {
                        "Couldn't find an appropriate `kappa` in the four-dimensional algorithm"
                    })?;
                *res.state().get_best_param().unwrap()
            };

            let k_005 = {
                let problem = FourDimensionalProblem {
                    n: n_nonblacklisted,
                    alpha: 0.05,
                    raise: true,
                };
                let init_param = 21.46;
                let solver = BrentRoot::new(15., 25., 1e-15);
                let res = Executor::new(problem, solver)
                    .configure(|state| state.param(init_param).max_iters(1000))
                    .timer(false)
                    .run()
                    .with_context(|| {
                        "Couldn't find an appropriate `k_005` in the four-dimensional algorithm"
                    })?;
                *res.state().get_best_param().unwrap()
            };

            let mut summed_rel_discrepancies = Vec::with_capacity(n_nonblacklisted);

            for (i, (object, triples)) in
                izip!(self.objects.borrow().iter(), self.triples.borrow().iter()).enumerate()
            {
                if !object.outlier {
                    let mut summed_rel_discrepancy = F::zero();
                    for triple in triples {
                        // We don't use the function here because there is a slight difference in the
                        // squared values, which sometimes leads to huge difference in the results
                        summed_rel_discrepancy = summed_rel_discrepancy
                            + (triple.observed - triple.model).powi(2) / triple.error.powi(2);
                    }
                    summed_rel_discrepancies.push((i, summed_rel_discrepancy));
                }
            }

            // Find the outliers
            let mut outliers: Vec<(usize, F)> = summed_rel_discrepancies
                .iter()
                .copied()
                .filter(|(_, summed_rel_discrepancy)| *summed_rel_discrepancy > kappa)
                .collect();

            if !outliers.is_empty() {
                // Sort the outliers, the smallest discrepancy first
                outliers.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

                // Remove L' outliers if they're small enough
                let len = l_stroke.min(outliers.len());
                for j in (0..len).rev() {
                    let (_, summed_rel_discrepancy) = &outliers[j];
                    if *summed_rel_discrepancy <= k_005 {
                        outliers.swap_remove(j);
                    }
                }

                // Mark the rest as outliers
                let mut objects = self.objects.borrow_mut();
                for (i, _) in &outliers {
                    objects[*i].outlier = true;
                }
            }

            FourDimensionalOutliers {
                vec: outliers,
                kappa,
                k_005,
            }
        };

        Ok((one_dimensional_outliers, four_dimensional_outliers))
    }
}
