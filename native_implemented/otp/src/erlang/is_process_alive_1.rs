#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::runtime::registry::pid_to_process;

#[native_implemented::function(erlang:is_process_alive/1)]
pub fn result(process: &Process, pid: Term) -> exception::Result<Term> {
    if pid == process.pid_term() {
        Ok((!process.is_exiting()).into())
    } else {
        let pid_pid = term_try_into_local_pid!(pid)?;

        match pid_to_process(&pid_pid) {
            Some(arc_process) => Ok((!arc_process.is_exiting()).into()),
            None => Ok(false.into()),
        }
    }
}
