#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use anyhow::*;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

#[native_implemented::function(erlang:length/1)]
pub fn result(process: &Process, list: Term) -> exception::Result<Term> {
    match list.decode()? {
        TypedTerm::Nil => Ok(0.into()),
        TypedTerm::List(cons) => match cons.count() {
            Some(count) => Ok(process.integer(count)),
            None => Err(ImproperListError).context(format!("list ({}) is improper", list)),
        },
        _ => Err(TypeError).context(format!("list ({}) is not a list", list)),
    }
    .map_err(From::from)
}
