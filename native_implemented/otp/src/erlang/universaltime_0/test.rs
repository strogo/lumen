use std::thread;
use std::time::Duration;

use crate::erlang::universaltime_0::result;
use crate::test::with_process;

#[test]
fn increases_after_2_seconds() {
    with_process(|process| {
        let first = result(process);

        thread::sleep(Duration::from_secs(2));

        let second = result(process);

        assert!(first < second);
    });
}
