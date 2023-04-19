//! Errors logger

extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;
use argmin::core::observers::Observe;
use argmin::core::State;
use argmin::core::KV;

/// Errors logger
#[allow(clippy::missing_docs_in_private_items)]
pub(super) struct ErrorsLogger {
    pub(super) writer: Rc<RefCell<BufWriter<File>>>,
}

impl<I> Observe<I> for ErrorsLogger
where
    I: State,
    <I as State>::Param: Display,
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
        // Writer the state
        writeln!(
            self.writer.borrow_mut(),
            "iter: {iter:>3}, cost: {cost:>18.15}, best_cost: {best_cost:>18.15}, param: {param:>18.15}, best_param: {best_param:>18.15}",
        )?;
        Ok(())
    }
}
