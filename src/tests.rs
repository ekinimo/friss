//! # Test Module
//!
//! This module contains tests for the parser combinator library.

use crate::core::*;
use crate::parsers::{Indentation, Offset, Position, WithState};
use crate::state::{ StateCarrier, StatefulParser};
use crate::types::*;
use core::str;

use crate::sugar::ParserSugar;

#[test]
fn test_either_simple_fold() {
    let e1: Either<i32, i32> = Either::Left(42);
    let e2: Either<i32, i32> = Either::Right(42);

    assert_eq!(e1.fold(), 42);
    assert_eq!(e2.fold(), 42);
}

#[test]
fn test_either3_simple_fold() {
    let e1: Either3<i32, i32, i32> = Either3::Left(42);
    let e2: Either3<i32, i32, i32> = Either3::Middle(42);
    let e3: Either3<i32, i32, i32> = Either3::Right(42);

    assert_eq!(e1.fold(), 42);
    assert_eq!(e2.fold(), 42);
    assert_eq!(e3.fold(), 42);
}

/// Test recursively defined parsers
#[test]
fn test_recursive_parser() {
    // Parser for handling nested parentheses
    // Grammar: P -> (P) | empty
    let paren_parser: Box<dyn Parser<&str, i32, &str>> = recursive(move |parser| {
        // First explicitly create the parser for nested parentheses
        let nested = Box::new(
            '('.make_character_matcher("Expected opening paren")
                .seq(move |x| parser.parse(x))
                .map_err(|x| x.fold())
                .seq(')'.make_character_matcher("Expected closing paren"))
                .map_err(|x| x.fold())
                .map(|((_, inner), _)| inner + 1),
        );

        // Then separately create the empty parser
        let empty = "".make_literal_matcher("").map(|_| 0);

        // Now combine them, trying nested first, then empty if nested fails
        Box::new(
            nested
                .alt(empty)
                .map_err(|(a, _b)| a)
                .map(|either| match either {
                    Either::Left(depth) => depth,
                    Either::Right(empty_result) => empty_result,
                }),
        )
    });

    assert_eq!(paren_parser.parse(""), Ok(("", 0)));
    assert_eq!(paren_parser.parse("()"), Ok(("", 1)));
    assert_eq!(paren_parser.parse("(())"), Ok(("", 2)));
    assert_eq!(paren_parser.parse("((()))"), Ok(("", 3)));
    assert_eq!(paren_parser.parse("()extra"), Ok(("extra", 1)));

    assert_eq!(
        paren_parser.parse(")("),
        Ok((")(", 0)) // Matches empty case when no valid parens found
    );
}
/// Test reverse application
#[test]
fn test_rev_apply<'a>() {
    let parser = (
        [1].make_literal_matcher("1 failed"),
        [2].make_literal_matcher("2 failed"),
    )
        .seq()
        .rev_apply(move |input: &'a [i32]| Ok((input, move |a: &[i32], b: &[i32]| b[0] + a[0])));
    match parser.parse(&[1, 2]) {
        Ok((_, x)) => assert_eq!(x, 3),
        _ => assert!(false, "Parser failed"),
    }
}

/// Test chaining parsers with and_then
#[test]
fn test_and_then() {
    let first = "a".make_literal_matcher("No a");
    let parser = first.and_then(|_| "b".make_literal_matcher("No b"));

    assert_eq!(parser.parse("ab"), Ok(("", "b")));
    assert_eq!(parser.parse("a"), Err(("", "No b")));
}

/// Test exactly N parser
#[test]
fn test_exactly_n() {
    let parser = "a"
        .make_literal_matcher("No a")
        .exactly_n::<3>("Need exactly 3");

    assert_eq!(parser.parse("aaa"), Ok(("", Box::new(["a", "a", "a"]))));
    assert_eq!(parser.parse("aa"), Err(("", "Need exactly 3")));
    assert_eq!(parser.parse("aaaa"), Ok(("a", Box::new(["a", "a", "a"]))));
}

/// Test at least N parser
#[test]
fn test_at_least_n() {
    let parser = "a"
        .make_literal_matcher("No a")
        .at_least_n(2, "Need at least 2");

    assert_eq!(parser.parse("aa"), Ok(("", vec!["a", "a"])));
    assert_eq!(parser.parse("aaa"), Ok(("a", vec!["a", "a"])));
    assert_eq!(parser.parse("a"), Err(("", "Need at least 2")));
}

/// Test many parser with zero matches
#[test]
fn test_many_zero_matches() {
    let parser = "a".make_literal_matcher("No a").many();
    assert_eq!(parser.parse(""), Ok(("", vec![])));
    assert_eq!(parser.parse("b"), Ok(("b", vec![])));
}

/// Test alternation between multiple options
#[test]
fn test_alt_multiple_options() {
    let parser = (
        "a".make_literal_matcher("Not a"),
        "b".make_literal_matcher("Not b"),
        "c".make_literal_matcher("Not c"),
    )
        .alt();

    assert_eq!(parser.parse("a"), Ok(("", Either3::Left("a"))));
    assert_eq!(parser.parse("b"), Ok(("", Either3::Middle("b"))));
    assert_eq!(parser.parse("c"), Ok(("", Either3::Right("c"))));
    assert_eq!(parser.parse("d"), Err(("d", ("Not a", "Not b", "Not c"))));
}

/// Test maybe parser
#[test]
fn test_maybe() {
    let parser = "a".make_literal_matcher("No a").maybe();
    assert_eq!(parser.parse("a"), Ok(("", Some("a"))));
    assert_eq!(parser.parse("b"), Ok(("b", None)));
}

/// Test skip parser
#[test]
fn test_skip() {
    let parser = "a"
        .make_literal_matcher("No a")
        .skip("b".make_literal_matcher("No b"));

    assert_eq!(parser.parse("ab"), Ok(("", "a")));
    assert_eq!(parser.parse("a"), Err(("", "No b")));
}

/// Test validate parser
#[test]
fn test_validate() {
    let even_num = (
        "2".make_literal_matcher("Not 2"),
        "3".make_literal_matcher("Not 3"),
    )
        .alt()
        .map(|x| match x {
            Either::Left(x) | Either::Right(x) => x,
        })
        .map_err(|_| "Not 2 or 3")
        .validate(|s| s.parse::<i32>().unwrap() % 2 == 0, "Odd number");

    assert_eq!(even_num.parse("2"), Ok(("", "2")));
    assert_eq!(even_num.parse("3"), Err(("3", "Odd number")));
}

/// Test bind parser
#[test]
fn test_bind_output() {
    let num = "123".make_literal_matcher("Not number");
    let parser = num.bind_output(|n| {
        n[0..2]
            .make_literal_matcher("Wrong length")
            .map(|len| len.to_string())
    });

    assert_eq!(parser.parse("12312"), Ok(("", "12".to_string())));
    assert_eq!(parser.parse("12"), Err(("12", "Not number")));
}

#[test]
fn test_bind() {
    let parser = "hello".make_literal_matcher("Not hello").bind(
        |_hello| " parsed world".make_literal_matcher("Not world"),
        |_err| "goodbye".make_literal_matcher("Not goodbye"),
    );

    assert_eq!(
        parser.parse("hello parsed world"),
        Ok(("", " parsed world"))
    );
    assert_eq!(parser.parse("goodbye"), Ok(("", "goodbye")));
}

#[test]
fn test_bind_err() {
    let parser = "invalid".make_literal_matcher("Not valid");
    let parser = parser.bind_err(
        move |_err| "fallback".make_literal_matcher("Not fallback"),
        "Success not expected",
    );

    assert_eq!(parser.parse("invalid"), Err(("", "Success not expected")));
    assert_eq!(parser.parse("fallback"), Ok(("", "fallback")));
}

#[test]
fn test_bind_err2() {
    // Create a parser that always fails
    let failing_parser = "fail".make_literal_matcher("Failing parser");

    // Bind the error to a parser that returns "recovered"
    let recovery_parser = failing_parser.bind_err(
        |_err| "recovered".make_literal_matcher("Recovery failed"),
        "Unexpected success",
    );

    // Test that bind_err works when the first parser fails
    assert_eq!(recovery_parser.parse("recovered"), Ok(("", "recovered")));

    // Test that bind_err returns the expected error when the first parser succeeds
    assert_eq!(
        recovery_parser.parse("fail"),
        Err(("", "Unexpected success"))
    );
}

#[test]
fn test_bind2() {
    // Create parsers for success and error cases
    let success_parser = "success".make_literal_matcher("Not success");

    // Bind both success and error cases
    let combined_parser = success_parser.bind(
        |_s| "bound_success".make_literal_matcher("Bound success failed"),
        |_e| "bound_error".make_literal_matcher("Bound error failed"),
    );

    // Test success path
    assert_eq!(
        combined_parser.parse("successbound_success"),
        Ok(("", "bound_success"))
    );

    // Test error path
    assert_eq!(
        combined_parser.parse("bound_error"),
        Ok(("", "bound_error"))
    );
}
#[test]
fn test_general_bind() {
    let digit = "digit"
        .with_state(Position::new(1, 1))
        .make_literal_matcher("no digit");

    // Use general_bind to choose the next parser based on first result
    let p = digit.general_bind(
        |_state, _digit| {
            // create a new parser using state and output here
            "haha"
                .with_state(Position::new(1, 1))
                .make_literal_matcher("No haha")
        },
        |_state, _error| {
            // create a new parser using state and error here
            "hehe"
                .with_state(Position::new(1, 1))
                .make_literal_matcher("No hehe")
        },
    );
    let r1 = p.parse_with_state("digithaha", Position::new(0, 0));
    assert_eq!(
        r1,
        Ok((
            StateCarrier::new(Position { column: 9, line: 0 }, ""),
            StateCarrier::new(Position { column: 1, line: 1 }, "haha")
        ))
    )
}

#[test]
fn test_stateful_parser() {
    // Create a parser that tracks position through the input
    let char_parser = <&str as Parsable<&str>>::make_anything_matcher("Expected char");

    // Convert to a stateful parser that increments the offset
    let stateful = char_parser.with_state_transition(
        |state: Offset, _input, output, _orig| {
            let mut new_state = state;
            new_state.increment(1);
            (new_state, _input, output)
        },
        |state, input, error, _orig| (state, input, error),
    );

    // Parse with initial state
    let input = "abc".with_state(Offset(0));
    let result = stateful.parse(input);

    // Check that the state was properly updated
    match result {
        Ok((rest, output)) => {
            assert_eq!(output, 'a');
            assert_eq!(rest.state.value(), 1);
            assert_eq!(rest.input, "bc");
        }
        Err(_) => panic!("Parsing failed"),
    }
}

#[test]
fn test_state_capture() {
    // Test get_current_state
    {
        let digit_parser = <&str as Parsable<&str>>::make_anything_matcher("Expected digit")
            .validate(|c| c.is_ascii_digit(), "Not a digit")
            .with_state_transition(
                |mut state: Offset, _input, output, _orig| {
                    state.increment(1);
                    (state, _input, output)
                },
                |state, input, error, _orig| (state, input, error),
            );

        let current_state_parser = digit_parser.get_current_state();
        let result = current_state_parser.parse("1abc".with_state(Offset(0)));

        // Check that we get the updated state
        match result {
            Ok((rest, state)) => {
                assert_eq!(state.value(), 1);
                assert_eq!(rest.input, "abc");
            }
            Err(_) => panic!("Parsing failed"),
        }
    }
    {
        let digit_parser = <&str as Parsable<&str>>::make_anything_matcher("Expected digit")
            .validate(|c| c.is_ascii_digit(), "Not a digit")
            .with_state_transition(
                |mut state: Offset, _input, output, _orig| {
                    state.increment(1);
                    (state, _input, output)
                },
                |state, input, error, _orig| (state, input, error),
            );

        // Test get_last_state
        let last_state_parser = digit_parser.get_last_state();
        let result = last_state_parser.parse("1abc".with_state(Offset(0)));

        // Check that we get the original state
        match result {
            Ok((rest, state)) => {
                assert_eq!(state.value(), 0);
                assert_eq!(rest.input, "abc");
            }
            Err(_) => panic!("Parsing failed"),
        }
    }
    {
        let digit_parser = <&str as Parsable<&str>>::make_anything_matcher("Expected digit")
            .validate(|c| c.is_ascii_digit(), "Not a digit")
            .with_state_transition(
                |mut state: Offset, _input, output, _orig| {
                    state.increment(1);
                    (state, _input, output)
                },
                |state, input, error, _orig| (state, input, error),
            );

        // Test get_last_and_current_state
        let both_states_parser = digit_parser.get_last_and_current_state();
        let result = both_states_parser.parse("1abc".with_state(Offset(0)));

        // Check that we get both states
        match result {
            Ok((rest, (last, current))) => {
                assert_eq!(last.value(), 0);
                assert_eq!(current.value(), 1);
                assert_eq!(rest.input, "abc");
            }
            Err(_) => panic!("Parsing failed"),
        }
    }
}

#[test]
fn test_general_bind_2() {
    // Create a stateful parser for digits
    let digit_parser = <&str as Parsable<&str>>::make_anything_matcher("Expected char")
        .validate(|c| c.is_ascii_digit(), "Not a digit")
        .with_state_transition(
            |mut state: Offset, _input, output, _orig| {
                state.increment(1);
                (state, _input, output)
            },
            |state, input, error, _orig| (state, input, error),
        );

    // Test general_bind with success case
    let bind_parser = digit_parser.general_bind(
        |state, digit| {
            // On success, check if the digit is '1' and return a new parser
            if digit == '1' {
                "Y".with_state(Offset(0)).make_literal_matcher("No Y")
            } else {
                // This creates a parser that succeeds with a fixed result
                "X".with_state(Offset(0)).make_literal_matcher("No X")
            }
        },
        |_state, _error| "Z".with_state(Offset(0)).make_literal_matcher("No Z"),
    );

    // Test successful digit '1' followed by a letter
    let result = bind_parser.parse("1Y".with_state(Offset(0)));
    match result {
        Ok((rest, output)) => {
            assert_eq!(output, StateCarrier::new(Offset(0), "Y"));
            assert_eq!(rest.state.value(), 2); // Incremented twice (once for each parser)
            assert_eq!(rest.input, "");
        }
        Err(_) => panic!("Parsing failed"),
    }

    // Test successful digit '2' which returns 'X'
    let result = bind_parser.parse("2X".with_state(Offset(0)));
    match result {
        Ok((rest, output)) => {
            assert_eq!(output, StateCarrier::new(Offset(0), "X"));
            assert_eq!(rest.state.value(), 2);
            assert_eq!(rest.input, "");
        }
        Err(_) => panic!("Parsing failed"),
    }

    // Test error case which returns 'E'
    let result = bind_parser.parse("Z".with_state(Offset(0)));
    match result {
        Ok((rest, output)) => {
            assert_eq!(output, StateCarrier::new(Offset(0), "Z"));
            assert_eq!(rest.state.value(), 1); // No increment (error handler doesn't increment)
            assert_eq!(rest.input, "");
        }
        Err((sc, err)) => panic!("Parsing failed, {sc:?} {err:?}"),
    }
}

#[test]
fn test_state_carrier() {
    // Create a state carrier with an Offset state
    let carrier = "hello".with_state(Offset(5));

    // Check that state and input are stored correctly
    assert_eq!(carrier.state.value(), 5);
    assert_eq!(carrier.input, "hello");

    // Test mapping state
    let mapped = carrier.map_state(|offset| Offset(offset.value() + 10));
    assert_eq!(mapped.state.value(), 15);

    // Test mapping input
    let input_mapped = carrier.map_input(|s| &s[1..]);
    assert_eq!(input_mapped.input, "ello");
}

/// Test peek parser
#[test]
fn test_peek() {
    let parser = "a".make_literal_matcher("No a").peek();
    assert_eq!(parser.parse("abc"), Ok(("abc", "a"))); // Input remains "abc"
}

/// Test separator parser
#[test]
fn test_sep_by() {
    let num = "123".make_literal_matcher("Not number");
    let comma = ",".make_literal_matcher("No comma");
    let parser = num.sep_by(comma);

    assert_eq!(
        parser.parse("123,123,123"),
        Ok(("", vec!["123", "123", "123"]))
    );
    assert_eq!(parser.parse("123"), Ok(("", vec!["123"])));
}

/// Test chainl1 parser
#[test]
fn test_chainl1() {
    let num = "1".make_literal_matcher("Not one").map(|_| 1);
    let add = "+"
        .make_literal_matcher("No plus")
        .map(|_| Box::new(|a: i32, b: i32| a + b) as _);

    let parser = num.chainl1(add);
    assert_eq!(parser.parse("1+1+1"), Ok(("", 3)));
}

/// Test recover parser
#[test]
fn test_recover() {
    let bad = "invalid".make_literal_matcher("Still invalid");

    let parser = bad.recover_with(|_| "valid".make_literal_matcher("Invalid"));
    assert_eq!(parser.parse("valid"), Ok(("", "valid")));
}

/// Test literal sequence
#[test]
fn test_literal_sequence() {
    let result = "hello "
        .make_literal_matcher("hello failed")
        .seq("world".make_literal_matcher("world failed"))
        .seq('1'.make_character_matcher("1 failed"))
        .many()
        .parse("hello world1hello world1");

    let expected_output = (
        "",
        vec![(("hello ", "world"), '1'), (("hello ", "world"), '1')],
    );

    assert_eq!(result, Ok(expected_output));
}

/// Test function application
#[test]
fn test_fapply<'a>() {
    let p = "hello "
        .make_literal_matcher("hello failed")
        .seq("world ".make_literal_matcher("world failed"))
        .map(move |x: (&str, &str)| move |y: &'a str| (y, x))
        .map_err(|_| "err");
    let result = p
        .fapply("naha".make_literal_matcher("none"))
        .parse("hello world naha rlly");

    let expected_output = (" rlly", ("naha", ("hello ", "world ")));

    assert_eq!(result, Ok(expected_output));
}

/// Test at most N parser
#[test]
fn test_at_most_n_parser() {
    let parser = (
        'a'.make_character_matcher("char failed"),
        'b'.make_character_matcher("char failed"),
    )
        .seq()
        .map_err(|_| "fail")
        .at_most_n::<3>();

    let result = parser.parse("abab");

    if let Ok(("", arr)) = result {
        assert_eq!(arr[0], Some(('a', 'b')));
    } else {
        assert!(false, "Parser failed");
    }
}

/// JSON value definition for testing JSON parser
#[derive(PartialEq, Debug, Clone)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

/// JSON error definition for testing JSON parser
#[derive(Debug, Clone, PartialEq)]
enum JsonError {
    ExpectedValue,
    ExpectedBool,
    ExpectedNumber,
    ExpectedString,
    ExpectedArray,
    ExpectedObject,
    ExpectedColon,
    ExpectedComma,
    ExpectedEndOfInput,
    NoMatch,
}

/// A simple JSON parser as an example of a complete parser
fn json_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
    fn value_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
        |input| {
            (
                null_parser(),
                bool_parser(),
                number_parser(),
                string_parser(),
                array_parser(),
                object_parser(),
            )
                .alt()
                .map(|x| match x {
                    Either6::_1(x)
                    | Either6::_2(x)
                    | Either6::_3(x)
                    | Either6::_4(x)
                    | Either6::_5(x)
                    | Either6::_6(x) => x,
                })
                .map_err(|_| JsonError::NoMatch)
                .parse(input)
        }
    }

    fn null_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
        "null"
            .make_literal_matcher(JsonError::ExpectedValue)
            .map(|_| JsonValue::Null)
    }

    fn bool_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
        "true"
            .make_literal_matcher(JsonError::ExpectedBool)
            .alt("false".make_literal_matcher(JsonError::ExpectedBool))
            .map(|b| match b {
                Either::Left("true") => JsonValue::Bool(true),
                Either::Right("false") => JsonValue::Bool(false),
                _ => unreachable!(),
            })
            .map_err(|(e1, _)| e1)
    }

    fn number_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
        <&str as Parsable<JsonError>>::make_anything_matcher(JsonError::ExpectedNumber)
            .many()
            .map(|digits| {
                let s: String = digits.into_iter().collect();
                s
            })
            .validate(
                |s: &String| s.parse::<f32>().is_ok(),
                JsonError::ExpectedNumber,
            )
            .map(|s| JsonValue::Number(s.parse::<f32>().unwrap() as f64))
    }

    fn string_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
        (
            '"'.make_character_matcher(JsonError::ExpectedString),
            <&'a str as Parsable<JsonError>>::make_anything_matcher(JsonError::ExpectedString)
                .validate(|c| c != &'"', JsonError::ExpectedString)
                .many()
                .map(|x| JsonValue::String(x.into_iter().collect::<String>())),
            '"'.make_character_matcher(JsonError::ExpectedString),
        )
            .seq()
            .map(|(_, content, _)| content)
            .map_err(|x| match x {
                Either3::Left(e) | Either3::Middle(e) | Either3::Right(e) => e,
            })
    }

    fn bare_string_parser<'a>() -> impl Parser<&'a str, String, JsonError> {
        (
            '"'.make_character_matcher(JsonError::ExpectedString),
            <&'a str as Parsable<JsonError>>::make_anything_matcher(JsonError::ExpectedString)
                .validate(|c| c != &'"', JsonError::ExpectedString)
                .many()
                .map(|x| x.into_iter().collect::<String>()),
            '"'.make_character_matcher(JsonError::ExpectedString),
        )
            .seq()
            .map(|(_, content, _)| content)
            .map_err(|x| match x {
                Either3::Left(e) | Either3::Middle(e) | Either3::Right(e) => e,
            })
    }

    fn array_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
        (
            '['.make_character_matcher(JsonError::ExpectedArray),
            value_parser()
                .seq(
                    ','.make_character_matcher(JsonError::ExpectedComma)
                        .seq(value_parser())
                        .many(),
                )
                .maybe(),
            ']'.make_character_matcher(JsonError::ExpectedArray),
        )
            .seq()
            .map(|(_, content, _)| {
                let mut result = Vec::new();
                if let Some((first, rest)) = content {
                    result.push(first);
                    for (_, item) in rest {
                        result.push(item);
                    }
                }
                JsonValue::Array(result)
            })
            .map_err(|x| match x {
                Either3::Left(e) => e,
                Either3::Middle(Either::Left(e)) => e,
                Either3::Middle(Either::Right(Either::Left(e))) => e,
                Either3::Middle(Either::Right(Either::Right(e))) => e,
                Either3::Right(e) => e,
            })
    }

    fn key_value_pair_parser<'a>() -> impl Parser<&'a str, (String, JsonValue), JsonError> {
        (
            bare_string_parser(),
            ':'.make_character_matcher(JsonError::ExpectedColon),
            value_parser(),
        )
            .seq()
            .map(|(a, _, b)| (a, b))
            .map_err(|err| match err {
                Either3::Left(e) | Either3::Middle(e) | Either3::Right(e) => e,
            })
    }

    fn object_parser<'a>() -> impl Parser<&'a str, JsonValue, JsonError> {
        (
            '{'.make_character_matcher(JsonError::ExpectedObject),
            key_value_pair_parser().seq(
                ','.make_character_matcher(JsonError::ExpectedComma)
                    .seq(key_value_pair_parser())
                    .map(|(_, x)| x)
                    .map_err(|x| match x {
                        Either::Left(x) | Either::Right(x) => x,
                    })
                    .many()
                    .maybe(),
            ),
            '}'.make_character_matcher(JsonError::ExpectedObject),
        )
            .seq()
            .map(|(_, (content, maybe_rest), _)| {
                let mut result = vec![content];
                if let Some(x) = maybe_rest {
                    result.extend(x)
                }
                JsonValue::Object(result)
            })
            .map_err(|x| match x {
                Either3::Left(e)
                | Either3::Middle(Either::Left(e))
                | Either3::Middle(Either::Right(e))
                | Either3::Right(e) => e,
            })
    }

    value_parser()
}

/// Test JSON parser - null
#[test]
fn test_json_null() {
    let parser = json_parser();
    let ret = parser.parse("null");
    assert_eq!(ret, Ok(("", JsonValue::Null)));
}

/// Test JSON parser - true
#[test]
fn test_json_true() {
    let parser = json_parser();
    let ret = parser.parse("true");
    assert_eq!(ret, Ok(("", JsonValue::Bool(true))));
}

/// Test JSON parser - false
#[test]
fn test_json_false() {
    let parser = json_parser();
    let ret = parser.parse("false");
    assert_eq!(ret, Ok(("", JsonValue::Bool(false))));
}

/// Test JSON parser - number
#[test]
fn test_json_num() {
    let parser = json_parser();
    let ret = parser.parse("3.0");
    assert_eq!(ret, Ok(("", JsonValue::Number(3.0))));
}

/// Test JSON parser - string
#[test]
fn test_json_string() {
    let parser = json_parser();
    let ret = parser.parse("\"Hello\"");
    assert_eq!(ret, Ok(("", JsonValue::String("Hello".to_owned()))));
}

/// Test JSON parser - array
#[test]
fn test_json_array() {
    let parser = json_parser();
    let ret = parser.parse("[true,null]");
    assert_eq!(
        ret,
        Ok((
            "",
            JsonValue::Array(vec![JsonValue::Bool(true), JsonValue::Null])
        ))
    );
}

/// Test JSON parser - object
#[test]
fn test_json_object() {
    let parser = json_parser();
    let ret = parser.parse("{\"hmm\":null,\"true\":false,\"self\":{\"hmm\":null,\"true\":false}}");
    assert_eq!(
        ret,
        Ok((
            "",
            JsonValue::Object(vec![
                ("hmm".to_owned(), JsonValue::Null),
                ("true".to_owned(), JsonValue::Bool(false)),
                (
                    "self".to_owned(),
                    JsonValue::Object(vec![
                        ("hmm".to_owned(), JsonValue::Null),
                        ("true".to_owned(), JsonValue::Bool(false)),
                    ])
                )
            ])
        ))
    );
}

/// Test for an expression parser that can handle basic arithmetic expressions
mod expr_parser {
    use super::super::*;

    /// Expression tree node
    #[derive(Debug, PartialEq, Clone)]
    pub enum Expr {
        /// A numeric value
        Number(f64),
        /// A variable
        Variable(String),
        /// A binary operation
        BinaryOp(Box<Expr>, Op, Box<Expr>),
    }

    /// Operation type
    #[derive(Debug, PartialEq, Clone)]
    pub enum Op {
        /// Addition
        Add,
        /// Subtraction
        Sub,
        /// Multiplication
        Mul,
        /// Division
        Div,
    }

    /// Parse error
    #[derive(Debug, Clone, PartialEq)]
    pub enum ParseError {
        /// Expected a number
        ExpectedNumber,
        /// Expected a variable
        ExpectedVariable,
        /// Expected an operator
        ExpectedOperator,
        /// Expected a left parenthesis
        ExpectedLParen,
        /// Expected a right parenthesis
        ExpectedRParen,
        /// Unexpected input
        UnexpectedInput,
        /// Expected end of input
        EndOfInputExpected,
    }

    /// Checks if input is empty
    fn end_of_input<'a>() -> impl Parser<&'a str, (), ParseError> {
        |input: &'a str| {
            if input.is_empty() {
                Ok(("", ()))
            } else {
                Err((input, ParseError::EndOfInputExpected))
            }
        }
    }

    /// Parses a number
    fn number_parser<'a>() -> impl Parser<&'a str, Expr, ParseError> {
        (
            "1".make_literal_matcher(ParseError::ExpectedNumber),
            "2".make_literal_matcher(ParseError::ExpectedNumber),
            "3".make_literal_matcher(ParseError::ExpectedNumber),
            "4".make_literal_matcher(ParseError::ExpectedNumber),
            "5".make_literal_matcher(ParseError::ExpectedNumber),
            "6".make_literal_matcher(ParseError::ExpectedNumber),
            "7".make_literal_matcher(ParseError::ExpectedNumber),
            "8".make_literal_matcher(ParseError::ExpectedNumber),
            "9".make_literal_matcher(ParseError::ExpectedNumber),
            "0".make_literal_matcher(ParseError::ExpectedNumber),
        )
            .alt()
            .map(|x| match x {
                Either10::_1(x)
                | Either10::_2(x)
                | Either10::_3(x)
                | Either10::_4(x)
                | Either10::_5(x)
                | Either10::_6(x)
                | Either10::_7(x)
                | Either10::_8(x)
                | Either10::_9(x)
                | Either10::_10(x) => x,
            })
            .map_err(|_| ParseError::ExpectedNumber)
            .many()
            .map(|chars| chars.into_iter().collect::<String>())
            .validate(
                |s: &String| s.parse::<f64>().is_ok(),
                ParseError::ExpectedNumber,
            )
            .map(|s| Expr::Number(s.parse::<f64>().unwrap()))
    }

    /// Parses a variable
    fn variable_parser<'a>() -> impl Parser<&'a str, Expr, ParseError> {
        <&str as Parsable<ParseError>>::make_anything_matcher(ParseError::ExpectedVariable)
            .validate(|c: &char| c.is_alphabetic(), ParseError::ExpectedVariable)
            .map(|x| Expr::Variable(x.to_string()))
    }

    /// Parses an expression in parentheses
    fn paren_parser<'a>() -> impl Parser<&'a str, Expr, ParseError> {
        (
            '('.make_character_matcher(ParseError::ExpectedLParen),
            expr_parser_without_end(),
            ')'.make_character_matcher(ParseError::ExpectedRParen),
        )
            .seq()
            .map(|(_, expr, _)| expr)
            .map_err(|err| match err {
                Either3::Left(e) | Either3::Middle(e) | Either3::Right(e) => e,
            })
    }

    /// Parses an atomic expression (number, variable, or parenthesized expression)
    fn atom_parser<'a>() -> impl Parser<&'a str, Expr, ParseError> {
        |i| {
            paren_parser()
                .alt(variable_parser())
                .alt(number_parser())
                .map(|x| match x {
                    Either::Left(Either::Left(x)) => x,
                    Either::Left(Either::Right(x)) => x,
                    Either::Right(x) => x,
                })
                .map_err(|_| ParseError::ExpectedNumber)
                .parse(i)
        }
    }

    /// Parses a term (atoms and multiplication/division)
    fn term_parser<'a>() -> impl Parser<&'a str, Expr, ParseError> {
        let mul = "*"
            .make_literal_matcher(ParseError::ExpectedOperator)
            .map(|_| {
                Box::new(|a: Expr, b: Expr| Expr::BinaryOp(Box::new(a), Op::Mul, Box::new(b)))
                    as Box<dyn Fn(Expr, Expr) -> Expr>
            });

        let div = "/"
            .make_literal_matcher(ParseError::ExpectedOperator)
            .map(|_| {
                Box::new(|a: Expr, b: Expr| Expr::BinaryOp(Box::new(a), Op::Div, Box::new(b)))
                    as Box<dyn Fn(Expr, Expr) -> Expr>
            });

        atom_parser().chainl1(
            mul.alt(div)
                .map(|x| match x {
                    Either::Left(x) => x,
                    Either::Right(x) => x,
                })
                .map_err(|(x, _)| x),
        )
    }

    /// Parses an expression without checking for end of input
    fn expr_parser_without_end<'a>() -> impl Parser<&'a str, Expr, ParseError> {
        let add = "+"
            .make_literal_matcher(ParseError::ExpectedOperator)
            .map(|_| {
                Box::new(|a: Expr, b: Expr| Expr::BinaryOp(Box::new(a), Op::Add, Box::new(b)))
                    as Box<dyn Fn(Expr, Expr) -> Expr>
            });

        let sub = "-"
            .make_literal_matcher(ParseError::ExpectedOperator)
            .map(|_| {
                Box::new(|a: Expr, b: Expr| Expr::BinaryOp(Box::new(a), Op::Sub, Box::new(b)))
                    as Box<dyn Fn(Expr, Expr) -> Expr>
            });

        term_parser().chainl1(
            add.alt(sub)
                .map(|x| match x {
                    Either::Left(x) => x,
                    Either::Right(x) => x,
                })
                .map_err(|(x, _)| x),
        )
    }

    /// Parses a complete expression, checking for end of input
    fn expr_parser<'a>() -> impl Parser<&'a str, Expr, ParseError> {
        expr_parser_without_end()
            .seq(end_of_input())
            .map(|(expr, _)| expr)
            .map_err(|err| match err {
                Either::Left(e) | Either::Right(e) => e,
            })
    }

    /// Test for expression parser error handling
    #[test]
    fn test_error_handling() {
        let parser = expr_parser();

        assert_eq!(parser.parse("(3+4"), Err(("", ParseError::ExpectedNumber)));

        assert_eq!(
            parser.parse("3#4"),
            Err(("#4", ParseError::EndOfInputExpected))
        );

        assert_eq!(
            parser.parse("3+4abc"),
            Err(("abc", ParseError::EndOfInputExpected))
        );

        assert_eq!(
            parser.parse("123var"),
            Err(("var", ParseError::EndOfInputExpected))
        );
    }

    /// Test for valid expressions
    #[test]
    fn test_valid_expressions() {
        let parser = expr_parser();

        assert_eq!(
            parser.parse("3+4*2"),
            Ok((
                "",
                Expr::BinaryOp(
                    Box::new(Expr::Number(3.0)),
                    Op::Add,
                    Box::new(Expr::BinaryOp(
                        Box::new(Expr::Number(4.0)),
                        Op::Mul,
                        Box::new(Expr::Number(2.0))
                    ))
                )
            ))
        );

        assert_eq!(
            parser.parse("(x+y)*2"),
            Ok((
                "",
                Expr::BinaryOp(
                    Box::new(Expr::BinaryOp(
                        Box::new(Expr::Variable("x".to_string())),
                        Op::Add,
                        Box::new(Expr::Variable("y".to_string()))
                    )),
                    Op::Mul,
                    Box::new(Expr::Number(2.0))
                )
            ))
        );
    }
}

/// Tests positional tracking with line and column tracking
#[test]
fn test_position_tracking() {
    // Create a parser that tracks line and column position
    let char_parser = <&str as Parsable<&str>>::make_anything_matcher("Expected char")
        .with_state_transition(
            |mut state: Position, _input, output, _orig| {
                // Update position based on character
                if output == '\n' {
                    state.advance_line();
                } else {
                    state.advance_column(1);
                }
                (state, _input, output)
            },
            |state, input, error, _orig| (state, input, error),
        );

    // Parse a multi-line string
    let input = "abc\ndef\nghi".with_state(Position::new(1, 1));

    // Parse characters and track position
    let mut rest = input;

    // Parse first line
    for _ in 0..4 {
        // 'a', 'b', 'c', '\n'
        let result = char_parser.parse(rest);
        match result {
            Ok((new_rest, _)) => {
                rest = new_rest;
            }
            Err(_) => panic!("Parsing failed"),
        }
    }

    // Check position after first line
    assert_eq!(rest.state.line, 2);
    assert_eq!(rest.state.column, 0);

    // Parse second line
    for _ in 0..4 {
        // 'd', 'e', 'f', '\n'
        let result = char_parser.parse(rest);
        match result {
            Ok((new_rest, _)) => {
                rest = new_rest;
            }
            Err(_) => panic!("Parsing failed"),
        }
    }

    // Check position after second line
    assert_eq!(rest.state.line, 3);
    assert_eq!(rest.state.column, 0);
}

#[test]
fn test_indentation_aware_parsing<'a>() {
    // Create basic parsers for spaces and newlines
    let space = " ".make_literal_matcher("Expected space");
    let newline = "\n".make_literal_matcher("Expected newline");

    // Function that handles state updates based on indentation
    let parse_indentation = |input: StateCarrier<Indentation, &'a str>,
                             is_indent: bool|
     -> Result<
        (StateCarrier<Indentation, &str>, usize),
        (StateCarrier<Indentation, &str>, &str),
    > {
        // Function to count spaces followed by a newline
        let count_spaces = |input: &'a str| -> Result<(&'a str, usize), (&'a str, &'a str)> {
            let mut count = 0;
            let mut rest = input;

            // Count spaces
            loop {
                match space.parse(rest) {
                    Ok((new_rest, _)) => {
                        count += 1;
                        rest = new_rest;
                    }
                    Err(_) => break,
                }
            }

            // Expect a newline
            match newline.parse(rest) {
                Ok((final_rest, _)) => Ok((final_rest, count)),
                Err((rest, err)) => Err((rest, err)),
            }
        };

        match count_spaces(input.input) {
            Ok((rest, count)) => {
                let mut new_state = input.state.clone();

                if is_indent {
                    // For indent, push the level
                    new_state.push_level(count);
                } else {
                    // For dedent, check if we need to pop
                    if count < new_state.current_level() {
                        new_state.pop_level();
                    }
                }

                Ok((
                    StateCarrier {
                        state: new_state,
                        input: rest,
                    },
                    count,
                ))
            }
            Err((rest, err)) => Err((
                StateCarrier {
                    state: input.state,
                    input: rest,
                },
                err,
            )),
        }
    };

    // Create indent and dedent parsers
    let indent = |input| parse_indentation(input, true);
    let dedent = |input| parse_indentation(input, false);

    // Test with sample input
    let input = "  \n    \n  \n".with_state(Indentation::new());

    // Parse first indent
    let result = indent(input);
    match result {
        Ok((rest, _)) => {
            assert_eq!(rest.state.current_level(), 2);
            assert_eq!(rest.state.depth(), 1);

            // Parse second indent
            let result2 = indent(rest);
            match result2 {
                Ok((rest2, _)) => {
                    assert_eq!(rest2.state.current_level(), 4);
                    assert_eq!(rest2.state.depth(), 2);

                    // Parse dedent
                    let result3 = dedent(rest2);
                    match result3 {
                        Ok((rest3, _)) => {
                            assert_eq!(rest3.state.current_level(), 2);
                            assert_eq!(rest3.state.depth(), 1);
                        }
                        Err(_) => panic!("Dedent parsing failed"),
                    }
                }
                Err(_) => panic!("Second indent parsing failed"),
            }
        }
        Err(_) => panic!("First indent parsing failed"),
    }
}
