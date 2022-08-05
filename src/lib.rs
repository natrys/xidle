extern crate emacs;
extern crate x11;

use std::cell::RefCell;

use emacs::{defun, Env, Result};
emacs::plugin_is_GPL_compatible!();

mod xidle;

thread_local!(
    static INFO: RefCell<xidle::XIdleService> = 
        RefCell::new(xidle::XIdleService::new());
);

#[emacs::module(name(fn))]
fn x11idle(_env: &Env) -> Result<()> { Ok(()) }

#[defun]
fn get() -> Result<u64> {
    let idle = INFO.with(|i| {
        let idle = i.borrow().idle();
        idle.as_secs()
    });

    Ok(idle)
}
