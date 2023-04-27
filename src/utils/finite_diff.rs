//! Finite differences

use core::fmt::Debug;

use num::Float;
use numeric_literals::replace_float_literals;

/// A subset of the `finitediff`s `FiniteDiff` trait made generic
pub trait FiniteDiff<F> {
    /// Compute the forward difference
    fn forward_diff(&self, f: &dyn Fn(&Self) -> F, epsilon: F) -> Self;
    /// Compute the central difference
    fn central_diff(&self, f: &dyn Fn(&Self) -> F, epsilon: F) -> Self;
}

impl<F> FiniteDiff<F> for Vec<F>
where
    F: Float + Debug,
{
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn forward_diff(&self, f: &dyn Fn(&Self) -> F, epsilon: F) -> Self {
        let fx = (f)(self);
        let mut xt = self.clone();
        (0..self.len())
            .map(|i| {
                let fx1 = mod_and_calc(&mut xt, f, i, epsilon);
                (fx1 - fx) / epsilon
            })
            .collect()
    }
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn central_diff(&self, f: &dyn Fn(&Self) -> F, epsilon: F) -> Self {
        let mut xt = self.clone();
        (0..self.len())
            .map(|i| {
                let fx1 = mod_and_calc(&mut xt, f, i, epsilon);
                let fx2 = mod_and_calc(&mut xt, f, i, -epsilon);
                (fx1 - fx2) / (2.0 * epsilon)
            })
            .collect()
    }
}

/// Change the parameter and compute the function
#[allow(clippy::indexing_slicing)]
pub fn mod_and_calc<F, T>(x: &mut Vec<F>, f: &dyn Fn(&Vec<F>) -> T, idx: usize, y: F) -> T
where
    F: Float,
{
    let xtmp = x[idx];
    x[idx] = xtmp + y;
    let fx1 = (f)(x);
    x[idx] = xtmp;
    fx1
}
