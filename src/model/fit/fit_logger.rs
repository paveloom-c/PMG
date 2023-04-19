//! Fit logger

extern crate alloc;

use super::Params;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::ops::Index;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;
use argmin::core::observers::Observe;
use argmin::core::State;
use argmin::core::KV;
use indoc::formatdoc;
use num::Float;

/// Fit logger
#[allow(clippy::missing_docs_in_private_items)]
pub(super) struct FitLogger<F> {
    pub(super) params: Params<F>,
    pub(super) par_pairs: Rc<RefCell<Vec<(F, F, F)>>>,
    pub(super) writer: Rc<RefCell<BufWriter<File>>>,
}

impl<I, F> Observe<I> for FitLogger<F>
where
    I: State,
    <I as State>::Param: Index<usize>,
    <<I as State>::Param as Index<usize>>::Output: Display + Sized,
    F: Float + Debug + Display,
{
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    fn observe_iter(&mut self, state: &I, _kv: &KV) -> Result<()> {
        // Get the state
        let iter = state.get_iter();
        let cost = state.get_cost();
        let best_cost = state.get_best_cost();
        let param = state.get_param().unwrap();
        let best_param = state.get_best_param().unwrap();
        // Write the found reduced parallaxes
        for (i, &(par, par_e, par_r)) in self.par_pairs.borrow().iter().enumerate() {
            writeln!(
                self.writer.borrow_mut(),
                "{i}: par: {par} \u{b1} {par_e} -> par_r: {par_r}",
            )
            .ok();
        }
        // Writer the state
        writeln!(
            self.writer.borrow_mut(),
            "{}",
            formatdoc!(
                "
                iter: {iter}
                             {:>11} initial {:>11} current {:>14} best
                        L_1: {:>17} - {cost:>19} {best_cost:>19}
                        r_0: {i_0:>19.15} {p_0:>19.15} {best_p_0:>19.15}
                    omega_0: {i_1:>19.15} {p_1:>19.15} {best_p_1:>19.15}
                          a: {i_2:>19.15} {p_2:>19.15} {best_p_2:>19.15}
                      u_sun: {i_3:>19.15} {p_3:>19.15} {best_p_3:>19.15}
                      v_sun: {i_4:>19.15} {p_4:>19.15} {best_p_4:>19.15}
                      w_sun: {i_5:>19.15} {p_5:>19.15} {best_p_5:>19.15}
                    sigma_r: {i_6:>19.15} {p_6:>19.15} {best_p_6:>19.15}
                sigma_theta: {i_7:>19.15} {p_7:>19.15} {best_p_7:>19.15}
                    sigma_z: {i_8:>19.15} {p_8:>19.15} {best_p_8:>19.15}
                ",
                "",
                "",
                "",
                "",
                i_0 = self.params.r_0,
                i_1 = self.params.omega_0,
                i_2 = self.params.a,
                i_3 = self.params.u_sun,
                i_4 = self.params.v_sun,
                i_5 = self.params.w_sun,
                i_6 = self.params.sigma_r,
                i_7 = self.params.sigma_theta,
                i_8 = self.params.sigma_z,
                p_0 = param[0],
                p_1 = param[1],
                p_2 = param[2],
                p_3 = param[3],
                p_4 = param[4],
                p_5 = param[5],
                p_6 = param[6],
                p_7 = param[7],
                p_8 = param[8],
                best_p_0 = best_param[0],
                best_p_1 = best_param[1],
                best_p_2 = best_param[2],
                best_p_3 = best_param[3],
                best_p_4 = best_param[4],
                best_p_5 = best_param[5],
                best_p_6 = best_param[6],
                best_p_7 = best_param[7],
                best_p_8 = best_param[8],
            ),
        )?;
        Ok(())
    }
}
