#[cfg(test)]
mod test;

use crate::runtime::time::datetime;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

#[native_implemented::function(erlang:localtime/0)]
pub fn result(process: &Process) -> Term {
    let now: [usize; 6] = datetime::local_now();

    let date_tuple = process.tuple_from_slice(&[
        process.integer(now[0]),
        process.integer(now[1]),
        process.integer(now[2]),
    ]);
    let time_tuple = process.tuple_from_slice(&[
        process.integer(now[3]),
        process.integer(now[4]),
        process.integer(now[5]),
    ]);

    process.tuple_from_slice(&[date_tuple, time_tuple])
}
