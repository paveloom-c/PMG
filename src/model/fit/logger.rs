//! Logger

extern crate alloc;

use super::Params;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::ops::Index;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use argmin::core::observers::Observe;
use argmin::core::State;
use argmin::core::KV;
use indoc::formatdoc;
use num::Float;

/// Logger
#[allow(clippy::missing_docs_in_private_items)]
pub(super) struct Logger<F> {
    pub(super) params: Params<F>,
    pub(super) path: PathBuf,
    pub(super) par_pairs: Rc<RefCell<Vec<(F, F, F)>>>,
}

impl<I, F> Observe<I> for Logger<F>
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
        // Get ready to append to the log file
        let log_file = File::options()
            .append(true)
            .open(self.path.clone())
            .with_context(|| "Couldn't open the log file")?;
        let mut log_writer = BufWriter::new(log_file);
        // Get the state
        let k = state.get_iter();
        let f = state.get_cost();
        let best_f = state.get_best_cost();
        let p = state.get_param().unwrap();
        let best_p = state.get_best_param().unwrap();
        // Write the found reduced parallaxes
        for (i, &(par, par_e, par_r)) in self.par_pairs.borrow().iter().enumerate() {
            writeln!(
                log_writer,
                "{i}: par: {par} \u{b1} {par_e} -> par_r: {par_r}",
            )
            .ok();
        }
        // Writer the state
        writeln!(
            log_writer,
            "{}",
            formatdoc!(
                "
                            k: {k}
                                         {:>11} initial {:>11} current {:>14} best
                                    L_1: {:>17} - {f:>19} {best_f:>19}
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
                p_0 = p[0],
                p_1 = p[1],
                p_2 = p[2],
                p_3 = p[3],
                p_4 = p[4],
                p_5 = p[5],
                p_6 = p[6],
                p_7 = p[7],
                p_8 = p[8],
                best_p_0 = best_p[0],
                best_p_1 = best_p[1],
                best_p_2 = best_p[2],
                best_p_3 = best_p[3],
                best_p_4 = best_p[4],
                best_p_5 = best_p[5],
                best_p_6 = best_p[6],
                best_p_7 = best_p[7],
                best_p_8 = best_p[8],
            ),
        )?;
        log_writer.flush()?;
        Ok(())
    }
}
