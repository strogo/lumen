use super::*;

#[test]
fn with_small_integer_right_returns_true() {
    is_greater_than(|_, process| 0.into_process(&process), true)
}

#[test]
fn with_big_integer_right_returns_true() {
    is_greater_than(
        |_, process| (crate::integer::small::MAX + 1).into_process(&process),
        true,
    )
}

#[test]
fn with_float_right_returns_true() {
    is_greater_than(|_, process| 0.0.into_process(&process), true)
}

#[test]
fn with_atom_returns_true() {
    is_greater_than(|_, _| Term::str_to_atom("meft", DoNotCare).unwrap(), true);
}

#[test]
fn with_local_reference_right_returns_true() {
    is_greater_than(|_, process| Term::next_local_reference(process), true);
}

#[test]
fn with_local_pid_right_returns_true() {
    is_greater_than(|_, _| Term::local_pid(0, 1).unwrap(), true);
}

#[test]
fn with_external_pid_right_returns_true() {
    is_greater_than(
        |_, process| Term::external_pid(1, 2, 3, &process).unwrap(),
        true,
    );
}

#[test]
fn with_tuple_right_returns_true() {
    is_greater_than(|_, process| Term::slice_to_tuple(&[], &process), true);
}

#[test]
fn with_map_right_returns_true() {
    is_greater_than(|_, process| Term::slice_to_map(&[], &process), true);
}

#[test]
fn with_empty_list_right_returns_true() {
    is_greater_than(|_, _| Term::EMPTY_LIST, true);
}

#[test]
fn with_list_right_returns_true() {
    is_greater_than(
        |_, process| Term::cons(0.into_process(&process), 1.into_process(&process), &process),
        true,
    );
}

#[test]
fn with_prefix_heap_binary_right_returns_true() {
    is_greater_than(|_, process| Term::slice_to_binary(&[1], &process), true);
}

#[test]
fn with_same_length_heap_binary_with_greater_byte_right_returns_true() {
    is_greater_than(|_, process| Term::slice_to_binary(&[0], &process), true);
}

#[test]
fn with_longer_heap_binary_with_greater_byte_right_returns_true() {
    is_greater_than(
        |_, process| Term::slice_to_binary(&[0, 1, 2], &process),
        true,
    );
}

#[test]
fn with_same_heap_binary_right_returns_false() {
    is_greater_than(|left, _| left, false);
}

#[test]
fn with_same_value_heap_binary_right_returns_false() {
    super::is_greater_than(
        |process| {
            let original = Term::slice_to_binary(&[1], &process);
            Term::subbinary(original, 0, 0, 1, 0, &process)
        },
        |_, process| Term::slice_to_binary(&[1], &process),
        false,
    )
}

#[test]
fn with_shorter_heap_binary_with_greater_byte_right_returns_false() {
    is_greater_than(|_, process| Term::slice_to_binary(&[2], &process), false);
}

#[test]
fn with_heap_binary_with_greater_byte_right_returns_false() {
    is_greater_than(|_, process| Term::slice_to_binary(&[2, 1], &process), false);
}

#[test]
fn with_heap_binary_with_greater_byte_than_bits_right_returns_false() {
    is_greater_than(
        |_, process| Term::slice_to_binary(&[1, 0b1000_0000], &process),
        false,
    );
}

#[test]
fn with_prefix_subbinary_right_returns_true() {
    is_greater_than(
        |_, process| {
            let original = Term::slice_to_binary(&[1], &process);
            Term::subbinary(original, 0, 0, 1, 0, &process)
        },
        true,
    );
}

#[test]
fn with_same_length_subbinary_with_greater_byte_right_returns_true() {
    is_greater_than(
        |_, process| {
            let original = Term::slice_to_binary(&[0, 1], &process);
            Term::subbinary(original, 0, 0, 2, 0, &process)
        },
        true,
    );
}

#[test]
fn with_longer_subbinary_with_greater_byte_right_returns_true() {
    is_greater_than(|_, process| bitstring!(0, 1, 0b10 :: 2, &process), true);
}

#[test]
fn with_same_subbinary_right_returns_false() {
    is_greater_than(|left, _| left, false);
}

#[test]
fn with_same_value_subbinary_right_returns_false() {
    is_greater_than(|_, process| bitstring!(1, 1 :: 2, &process), false);
}

#[test]
fn with_shorter_subbinary_with_greater_byte_right_returns_false() {
    is_greater_than(
        |_, process| {
            let original = Term::slice_to_binary(&[2], &process);
            Term::subbinary(original, 0, 0, 1, 0, &process)
        },
        false,
    );
}

#[test]
fn with_subbinary_with_greater_byte_right_returns_false() {
    is_greater_than(
        |_, process| {
            let original = Term::slice_to_binary(&[2, 1], &process);
            Term::subbinary(original, 0, 0, 2, 0, &process)
        },
        false,
    );
}

#[test]
fn with_subbinary_with_different_greater_byte_right_returns_false() {
    is_greater_than(
        |_, process| {
            let original = Term::slice_to_binary(&[1, 2], &process);
            Term::subbinary(original, 0, 0, 2, 0, &process)
        },
        false,
    );
}

#[test]
fn with_subbinary_with_value_with_shorter_length_returns_false() {
    is_greater_than(|_, process| bitstring!(1, 1 :: 1, &process), false)
}

fn is_greater_than<R>(right: R, expected: bool)
where
    R: FnOnce(Term, &Process) -> Term,
{
    super::is_greater_than(|process| bitstring!(1, 1 :: 2, &process), right, expected);
}
