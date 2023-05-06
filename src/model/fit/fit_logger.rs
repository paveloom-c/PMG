//! Fit logger

extern crate alloc;

use super::{Objects, Params, Triple, Triples, PARAMS_N};

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;
use argmin::core::observers::Observe;
use argmin::core::State;
use argmin::core::KV;
use indoc::formatdoc;
use itertools::izip;
use num::Float;

/// Fit logger
#[allow(clippy::missing_docs_in_private_items)]
pub struct FitLogger<F> {
    pub sample_iteration: usize,
    pub objects: Objects<F>,
    pub params: Params<F>,
    pub triples: Rc<RefCell<Vec<Triples<F>>>>,
    pub writer: Rc<RefCell<BufWriter<File>>>,
}

impl<I, F> Observe<I> for FitLogger<F>
where
    I: State<Param = Vec<F>>,
    F: Float + Debug + Display,
{
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    fn observe_iter(&mut self, state: &I, _kv: &KV) -> Result<()> {
        // Get the state
        let iter = state.get_iter();
        let cost = state.get_cost();
        let best_cost = state.get_best_cost();
        let param = state.get_param().unwrap();
        let best_param = state.get_best_param().unwrap();

        let len = param.len();
        let mut p = [F::zero(); PARAMS_N];
        let mut best_p = [F::zero(); PARAMS_N];
        p[0..len].copy_from_slice(&param[0..len]);
        best_p[0..len].copy_from_slice(&best_param[0..len]);

        // Write the sample iteration
        writeln!(
            self.writer.borrow_mut(),
            "sample_iteration: {}\nfit_iteration: {iter}\n",
            self.sample_iteration
        )
        .ok();
        // Log the found reduced parallaxes
        for (i, (object, triples)) in
            izip!(self.objects.borrow().iter(), self.triples.borrow().iter()).enumerate()
        {
            // Unpack the par triple
            let Triple {
                observed: par,
                model: par_r,
                error: par_e,
            } = triples[3];
            // Log the values
            if object.outlier {
                writeln!(
                    self.writer.borrow_mut(),
                    "{}: par: {par} \u{b1} {par_e} -> OUTLIER",
                    i + 1
                )
                .ok();
            } else {
                writeln!(
                    self.writer.borrow_mut(),
                    "{}: par: {par} \u{b1} {par_e} -> par_r: {par_r}",
                    i + 1
                )
                .ok();
            }
        }
        // Log the state
        writeln!(
            self.writer.borrow_mut(),
            "{}",
            formatdoc!(
                "

                             {empty:11} initial {empty:11} current {empty:14} best
                        L_1: {empty:19} - {cost:>21.15} {best_cost:>21.15}
                          R: {i_0:>21.15} {p_0:>21.15} {best_p_0:>21.15}
                    omega_0: {i_1:>21.15} {p_1:>21.15} {best_p_1:>21.15}
                          A: {i_2:>21.15} {p_2:>21.15} {best_p_2:>21.15}
                      U_sun: {i_3:>21.15} {p_3:>21.15} {best_p_3:>21.15}
                      V_sun: {i_4:>21.15} {p_4:>21.15} {best_p_4:>21.15}
                      W_sun: {i_5:>21.15} {p_5:>21.15} {best_p_5:>21.15}
                    sigma_R: {i_6:>21.15} {p_6:>21.15} {best_p_6:>21.15}
                sigma_theta: {i_7:>21.15} {p_7:>21.15} {best_p_7:>21.15}
                    sigma_Z: {i_8:>21.15} {p_8:>21.15} {best_p_8:>21.15}
                    theta_2: {i_9:>21.15} {p_9:>21.15} {best_p_9:>21.15}
                    theta_3: {i_10:>21.15} {p_10:>21.15} {best_p_10:>21.15}
                    theta_4: {i_11:>21.15} {p_11:>21.15} {best_p_11:>21.15}
                    theta_5: {i_12:>21.15} {p_12:>21.15} {best_p_12:>21.15}
                    theta_6: {i_13:>21.15} {p_13:>21.15} {best_p_13:>21.15}
                    theta_7: {i_14:>21.15} {p_14:>21.15} {best_p_14:>21.15}
                    theta_8: {i_15:>21.15} {p_15:>21.15} {best_p_15:>21.15}
                ",
                empty = "",
                i_0 = self.params.r_0,
                i_1 = self.params.omega_0,
                i_2 = self.params.a,
                i_3 = self.params.u_sun,
                i_4 = self.params.v_sun,
                i_5 = self.params.w_sun,
                i_6 = self.params.sigma_r_g,
                i_7 = self.params.sigma_theta,
                i_8 = self.params.sigma_z,
                i_9 = self.params.theta_2,
                i_10 = self.params.theta_3,
                i_11 = self.params.theta_4,
                i_12 = self.params.theta_5,
                i_13 = self.params.theta_6,
                i_14 = self.params.theta_7,
                i_15 = self.params.theta_8,
                p_0 = p[0],
                p_1 = p[1],
                p_2 = p[2],
                p_3 = p[3],
                p_4 = p[4],
                p_5 = p[5],
                p_6 = p[6],
                p_7 = p[7],
                p_8 = p[8],
                p_9 = p[9],
                p_10 = p[10],
                p_11 = p[11],
                p_12 = p[12],
                p_13 = p[13],
                p_14 = p[14],
                p_15 = p[15],
                best_p_0 = best_p[0],
                best_p_1 = best_p[1],
                best_p_2 = best_p[2],
                best_p_3 = best_p[3],
                best_p_4 = best_p[4],
                best_p_5 = best_p[5],
                best_p_6 = best_p[6],
                best_p_7 = best_p[7],
                best_p_8 = best_p[8],
                best_p_9 = best_p[9],
                best_p_10 = best_p[10],
                best_p_11 = best_p[11],
                best_p_12 = best_p[12],
                best_p_13 = best_p[13],
                best_p_14 = best_p[14],
                best_p_15 = best_p[15],
            ),
        )
        .ok();
        Ok(())
    }
}
