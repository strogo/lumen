use super::*;

#[test]
fn without_locked_adds_process_message_to_mailbox_and_returns_message() {
    run!(
        |arc_process| { (Just(arc_process.clone()), strategy::term(arc_process)) },
        |(arc_process, message)| {
            let destination = registered_name();
            let different_arc_process = test::process::child(&arc_process);

            prop_assert_eq!(
                erlang::register_2::result(
                    arc_process.clone(),
                    destination,
                    different_arc_process.pid_term(),
                ),
                Ok(true.into())
            );

            prop_assert_eq!(result(&arc_process, destination, message), Ok(message));

            prop_assert!(has_process_message(&different_arc_process, message));

            Ok(())
        },
    );
}

#[test]
fn with_locked_adds_heap_message_to_mailbox_and_returns_message() {
    run!(
        |arc_process| { (Just(arc_process.clone()), strategy::term(arc_process)) },
        |(arc_process, message)| {
            let destination = registered_name();
            let different_arc_process = test::process::child(&arc_process);

            prop_assert_eq!(
                erlang::register_2::result(
                    arc_process.clone(),
                    destination,
                    different_arc_process.pid_term(),
                ),
                Ok(true.into())
            );

            let _different_process_heap_lock = different_arc_process.acquire_heap();

            prop_assert_eq!(result(&arc_process, destination, message), Ok(message));

            prop_assert!(has_heap_message(&different_arc_process, message));

            Ok(())
        },
    );
}
