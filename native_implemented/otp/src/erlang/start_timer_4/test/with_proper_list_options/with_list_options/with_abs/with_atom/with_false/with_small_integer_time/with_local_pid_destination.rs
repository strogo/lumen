use super::*;
use crate::test::freeze_timeout;

#[test]
fn with_different_process_sends_message_when_timer_expires() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &(milliseconds(), strategy::process()).prop_flat_map(|(milliseconds, arc_process)| {
                (
                    Just(milliseconds),
                    Just(arc_process.clone()),
                    strategy::term(arc_process),
                )
            }),
            |(milliseconds, arc_process, message)| {
                let time = arc_process.integer(milliseconds);

                let destination_arc_process = test::process::child(&arc_process);
                let destination = destination_arc_process.pid_term();

                let options = options(&arc_process);

                let start_monotonic = freeze_timeout();

                let result = result(arc_process.clone(), time, destination, message, options);

                prop_assert!(
                    result.is_ok(),
                    "Timer reference not returned.  Got {:?}",
                    result
                );

                let timer_reference = result.unwrap();

                prop_assert!(timer_reference.is_boxed_local_reference());

                let timeout_message = arc_process.tuple_from_slice(&[
                    Atom::str_to_term("timeout"),
                    timer_reference,
                    message,
                ]);

                prop_assert!(!has_message(&destination_arc_process, timeout_message));

                freeze_at_timeout(start_monotonic + milliseconds + Milliseconds(1));

                prop_assert!(has_message(&destination_arc_process, timeout_message));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn with_same_process_sends_message_when_timer_expires() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &(milliseconds(), strategy::process()).prop_flat_map(|(milliseconds, arc_process)| {
                (
                    Just(milliseconds),
                    Just(arc_process.clone()),
                    strategy::term(arc_process),
                )
            }),
            |(milliseconds, arc_process, message)| {
                let time = arc_process.integer(milliseconds);

                let destination = arc_process.pid_term();
                let options = options(&arc_process);

                let start_monotonic = freeze_timeout();

                let result = result(arc_process.clone(), time, destination, message, options);

                prop_assert!(
                    result.is_ok(),
                    "Timer reference not returned.  Got {:?}",
                    result
                );

                let timer_reference = result.unwrap();

                prop_assert!(timer_reference.is_boxed_local_reference());

                let timeout_message = arc_process.tuple_from_slice(&[
                    Atom::str_to_term("timeout"),
                    timer_reference,
                    message,
                ]);

                prop_assert!(!has_message(&arc_process, timeout_message));

                freeze_at_timeout(start_monotonic + milliseconds + Milliseconds(1));

                prop_assert!(has_message(&arc_process, timeout_message));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn without_process_sends_nothing_when_timer_expires() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &(milliseconds(), strategy::process()).prop_flat_map(|(milliseconds, arc_process)| {
                (
                    Just(milliseconds),
                    Just(arc_process.clone()),
                    strategy::term(arc_process),
                )
            }),
            |(milliseconds, arc_process, message)| {
                let time = arc_process.integer(milliseconds);
                let destination = Pid::next_term();
                let options = options(&arc_process);

                let start_monotonic = freeze_timeout();

                let result = result(arc_process.clone(), time, destination, message, options);

                prop_assert!(
                    result.is_ok(),
                    "Timer reference not returned.  Got {:?}",
                    result
                );

                let timer_reference = result.unwrap();

                prop_assert!(timer_reference.is_boxed_local_reference());

                let timeout_message = arc_process.tuple_from_slice(&[
                    Atom::str_to_term("timeout"),
                    timer_reference,
                    message,
                ]);

                prop_assert!(!has_message(&arc_process, timeout_message));

                freeze_at_timeout(start_monotonic + milliseconds + Milliseconds(1));

                prop_assert!(!has_message(&arc_process, timeout_message));

                Ok(())
            },
        )
        .unwrap();
}
