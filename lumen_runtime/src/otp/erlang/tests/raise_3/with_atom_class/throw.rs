use super::*;

mod with_list_stacktrace;

#[test]
fn with_local_reference_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| Term::next_local_reference(process));
}

#[test]
fn with_empty_list_stacktrace_errors() {
    let class = Term::str_to_atom("error", DoNotCare).unwrap();
    let stacktrace = Term::EMPTY_LIST;
    let reason = Term::str_to_atom("reason", DoNotCare).unwrap();

    assert_raises!(
        erlang::raise_3(class, reason, stacktrace),
        Error { arguments: None },
        reason,
        Some(stacktrace)
    );
}

#[test]
fn with_small_integer_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| 0usize.into_process(&process));
}

#[test]
fn with_big_integer_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| {
        <BigInt as Num>::from_str_radix("576460752303423489", 10)
            .unwrap()
            .into_process(&process)
    });
}

#[test]
fn with_float_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| 1.0.into_process(&process));
}

#[test]
fn with_local_pid_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|_| Term::local_pid(0, 0).unwrap());
}

#[test]
fn with_external_pid_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| Term::external_pid(1, 0, 0, &process).unwrap());
}

#[test]
fn with_tuple_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| Term::slice_to_tuple(&[], &process));
}

#[test]
fn with_map_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| Term::slice_to_map(&[], &process));
}

#[test]
fn with_heap_binary_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| {
        // :erlang.term_to_binary(:atom)
        Term::slice_to_binary(&[131, 100, 0, 4, 97, 116, 111, 109], &process)
    });
}

#[test]
fn with_subbinary_stacktrace_errors_badarg() {
    with_stacktrace_errors_badarg(|process| {
        // <<1::1, :erlang.term_to_binary(:atom) :: binary>>
        let original_term =
            Term::slice_to_binary(&[193, 178, 0, 2, 48, 186, 55, 182, 0b1000_0000], &process);
        Term::subbinary(original_term, 0, 1, 8, 0, &process)
    });
}

fn with<F>(f: F)
where
    F: FnOnce(Term, Term, &Process) -> (),
{
    with_process(|process| {
        let class = Term::str_to_atom("throw", DoNotCare).unwrap();
        let reason = Term::str_to_atom("reason", DoNotCare).unwrap();

        f(class, reason, &process)
    })
}

fn with_stacktrace_errors_badarg<S>(stacktrace: S)
where
    S: FnOnce(&Process) -> Term,
{
    with(|class, reason, process| {
        let stacktrace = stacktrace(&process);

        assert_badarg!(erlang::raise_3(class, reason, stacktrace))
    })
}
