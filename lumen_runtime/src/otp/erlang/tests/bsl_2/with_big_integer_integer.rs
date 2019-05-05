use super::*;

use num_traits::Num;

#[test]
fn with_atom_shift_errors_badarith() {
    with_shift_errors_badarith(|_| Term::str_to_atom("shift", DoNotCare).unwrap());
}

#[test]
fn with_local_reference_shift_errors_badarith() {
    with_shift_errors_badarith(|process| Term::next_local_reference(process));
}

#[test]
fn with_empty_list_shift_errors_badarith() {
    with_shift_errors_badarith(|_| Term::EMPTY_LIST);
}

#[test]
fn with_list_shift_errors_badarith() {
    with_shift_errors_badarith(|process| {
        Term::cons(0.into_process(&process), 1.into_process(&process), &process)
    });
}

#[test]
fn with_negative_without_big_integer_underflow_shifts_right_and_returns_big_integer() {
    with(|integer, process| {
        let shift = (-1).into_process(&process);

        let result = erlang::bsl_2(integer, shift, &process);

        assert!(result.is_ok());

        let shifted = result.unwrap();

        assert_eq!(shifted.tag(), Boxed);

        let unboxed_shifted: &Term = shifted.unbox_reference();

        assert_eq!(unboxed_shifted.tag(), BigInteger);

        assert_eq!(
            shifted,
            <BigInt as Num>::from_str_radix(
                "10110011100011110000111110000011111100000011111110000000111111110000000",
                2
            )
            .unwrap()
            .into_process(&process)
        );
    });
}

#[test]
fn with_negative_with_big_integer_underflow_without_small_integer_underflow_shifts_right_and_returns_small_integer(
) {
    with(|integer, process| {
        let shift = (-69).into_process(&process);

        let result = erlang::bsl_2(integer, shift, &process);

        assert!(result.is_ok());

        let shifted = result.unwrap();

        assert_eq!(shifted.tag(), SmallInteger);
        assert_eq!(shifted, 0b101.into_process(&process));
    });
}

#[test]
fn with_negative_with_underflow_returns_zero() {
    with(|integer, process| {
        let shift = (-74).into_process(&process);

        assert_eq!(
            erlang::bsl_2(integer, shift, &process),
            Ok(0b0.into_process(&process))
        );
    });
}

#[test]
fn with_zero_returns_same_big_integer() {
    with(|integer, process| {
        assert_eq!(
            erlang::bsl_2(integer, 0.into_process(&process), &process),
            Ok(integer)
        );
    });
}

#[test]
fn with_positive_returns_big_integer() {
    with(|integer, process| {
        let shift = 1.into_process(&process);

        let result = erlang::bsl_2(integer, shift, &process);

        assert!(result.is_ok());

        let shifted = result.unwrap();

        assert_eq!(shifted.tag(), Boxed);

        let unboxed_shifted: &Term = shifted.unbox_reference();

        assert_eq!(unboxed_shifted.tag(), BigInteger);
        assert_eq!(
            shifted,
            <BigInt as Num>::from_str_radix(
                "1011001110001111000011111000001111110000001111111000000011111111000000000",
                2
            )
            .unwrap()
            .into_process(&process)
        );
    });
}

#[test]
fn with_float_shift_errors_badarith() {
    with_shift_errors_badarith(|process| 1.0.into_process(&process));
}

#[test]
fn with_local_pid_shift_errors_badarith() {
    with_shift_errors_badarith(|_| Term::local_pid(0, 1).unwrap());
}

#[test]
fn with_external_pid_shift_errors_badarith() {
    with_shift_errors_badarith(|process| Term::external_pid(1, 2, 3, &process).unwrap());
}

#[test]
fn with_tuple_shift_errors_badarith() {
    with_shift_errors_badarith(|process| Term::slice_to_tuple(&[], &process));
}

#[test]
fn with_map_is_shift_errors_badarith() {
    with_shift_errors_badarith(|process| Term::slice_to_map(&[], &process));
}

#[test]
fn with_heap_binary_shift_errors_badarith() {
    with_shift_errors_badarith(|process| Term::slice_to_binary(&[], &process));
}

#[test]
fn with_subbinary_shift_errors_badarith() {
    with_shift_errors_badarith(|process| bitstring!(1 :: 1, &process));
}

fn with<F>(f: F)
where
    F: FnOnce(Term, &Process) -> (),
{
    with_process(|process| {
        let integer = <BigInt as Num>::from_str_radix(
            "101100111000111100001111100000111111000000111111100000001111111100000000",
            2,
        )
        .unwrap()
        .into_process(&process);

        assert_eq!(integer.tag(), Boxed);

        let integer_unboxed: &Term = integer.unbox_reference();

        assert_eq!(integer_unboxed.tag(), BigInteger);

        f(integer, &process)
    })
}

fn with_shift_errors_badarith<S>(shift: S)
where
    S: FnOnce(&Process) -> Term,
{
    super::errors_badarith(|process| {
        let integer = <BigInt as Num>::from_str_radix(
            "101100111000111100001111100000111111000000111111100000001111111100000000",
            2,
        )
        .unwrap()
        .into_process(&process);

        assert_eq!(integer.tag(), Boxed);

        let integer_unboxed: &Term = integer.unbox_reference();

        assert_eq!(integer_unboxed.tag(), BigInteger);

        let shift = shift(&process);

        erlang::bsl_2(integer, shift, &process)
    });
}
