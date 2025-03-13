//! # Test Module
//!
//! This module contains tests for the parser combinator library.

use crate::core::*;
use crate::types::*;
use core::str;

use crate::sugar::ParserSugar;


// Usage example with nested Either types
#[test]
fn test_eithers() {
    // Either<A, A> - base case
    let simple: Either<i32, i32> = Either::Left(5);
    let result1 = simple.deep_fold(); // Type: DeepFoldable<Zero, Zero>
    assert_eq!(result1, 5);
    
    // Either<Either<A, A>, A> - one level of nesting on left
    let nested_left: Either<Either<i32, i32>, i32> = Either::Left(Either::Right(10));
    let result2 = nested_left.deep_fold(); // Type: DeepFoldable<Succ<Zero>, Zero>
    assert_eq!(result2, 10);
    
    // Either<A, Either<A, A>> - one level of nesting on right
    let nested_right: Either<i32, Either<i32, i32>> = Either::Right(Either::Left(15));
    let result3 = nested_right.deep_fold(); // Type: DeepFoldable<Zero, Succ<Zero>>
    assert_eq!(result3, 15);
}


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
    
    #[test]
    fn test_either_zero_zero_deep_fold() {
        let e1: Either<i32, i32> = Either::Left(42);
        let e2: Either<i32, i32> = Either::Right(42);
        
        let result1 = DeepFoldable::<(Zero, Zero)>::deep_fold(e1);
        let result2 = DeepFoldable::<(Zero, Zero)>::deep_fold(e2);
        
        assert_eq!(result1, 42);
        assert_eq!(result2, 42);
    }
    
    #[test]
    fn test_either_succ_zero_deep_fold() {
        // We need to explicitly specify all the types for the compiler
        type EitherInt = Either<i32, i32>;
        
        // Either<Either<i32, i32>, i32>
        let nested1: Either<EitherInt, i32> = Either::Left(Either::Left(42));
        let nested2: Either<EitherInt, i32> = Either::Left(Either::Right(42));
        let nested3: Either<EitherInt, i32> = Either::Right(42);
        
        let result1 = <Either<EitherInt, i32> as DeepFoldable<(Succ<Zero>, Zero)>>::deep_fold(nested1);
        let result2 = <Either<EitherInt, i32> as DeepFoldable<(Succ<Zero>, Zero)>>::deep_fold(nested2);
        let result3 = <Either<EitherInt, i32> as DeepFoldable<(Succ<Zero>, Zero)>>::deep_fold(nested3);
        
        assert_eq!(result1, 42);
        assert_eq!(result2, 42);
        assert_eq!(result3, 42);
    }
    
    #[test]
    fn test_either_zero_succ_deep_fold() {
        // We need to explicitly specify all the types for the compiler
        type EitherInt = Either<i32, i32>;
        
        // Either<i32, Either<i32, i32>>
        let nested1: Either<i32, EitherInt> = Either::Left(42);
        let nested2: Either<i32, EitherInt> = Either::Right(Either::Left(42));
        let nested3: Either<i32, EitherInt> = Either::Right(Either::Right(42));
        
        let result1 = <Either<i32, EitherInt> as DeepFoldable<(Zero, Succ<Zero>)>>::deep_fold(nested1);
        let result2 = <Either<i32, EitherInt> as DeepFoldable<(Zero, Succ<Zero>)>>::deep_fold(nested2);
        let result3 = <Either<i32, EitherInt> as DeepFoldable<(Zero, Succ<Zero>)>>::deep_fold(nested3);
        
        assert_eq!(result1, 42);
        assert_eq!(result2, 42);
        assert_eq!(result3, 42);

        let nested1: Either<i32, EitherInt> = Either::Left(42);
        let nested2: Either<i32, EitherInt> = Either::Right(Either::Left(42));
        let nested3: Either<i32, EitherInt> = Either::Right(Either::Right(42));

        let result1 = nested1.deep_fold();
        let result2 = nested2.deep_fold();
        let result3 = nested3.deep_fold();
        
        assert_eq!(result1, 42);
        assert_eq!(result2, 42);
        assert_eq!(result3, 42);


    }

    #[test]
    fn test_either_succ_succ_deep_fold() {
        // We need to explicitly specify all the types for the compiler
        type EitherInt = Either<i32, i32>;
        
        // Either<Either<i32, i32>, Either<i32, i32>>
        let nested1: Either<EitherInt, EitherInt> = Either::Left(Either::Left(42));
        let nested2: Either<EitherInt, EitherInt> = Either::Left(Either::Right(42));
        let nested3: Either<EitherInt, EitherInt> = Either::Right(Either::Left(42));
        let nested4: Either<EitherInt, EitherInt> = Either::Right(Either::Right(42));
        
        let result1 = <Either<EitherInt, EitherInt> as DeepFoldable<(Succ<Zero>, Succ<Zero>)>>::deep_fold(nested1);
        let result2 = <Either<EitherInt, EitherInt> as DeepFoldable<(Succ<Zero>, Succ<Zero>)>>::deep_fold(nested2);
        let result3 = <Either<EitherInt, EitherInt> as DeepFoldable<(Succ<Zero>, Succ<Zero>)>>::deep_fold(nested3);
        let result4 = <Either<EitherInt, EitherInt> as DeepFoldable<(Succ<Zero>, Succ<Zero>)>>::deep_fold(nested4);
        
        assert_eq!(result1, 42);
        assert_eq!(result2, 42);
        assert_eq!(result3, 42);
        assert_eq!(result4, 42);

    }

#[test]
fn test_either3_deep_fold_basics() {
    // Test basic deep_fold for Either3
    let e1: Either3<i32, i32, i32> = Either3::Left(42);
    let e2: Either3<i32, i32, i32> = Either3::Middle(42);
    let e3: Either3<i32, i32, i32> = Either3::Right(42);
    
    let result1 = DeepFoldable::<(Zero, Zero, Zero)>::deep_fold(e1);
    let result2 = DeepFoldable::<(Zero, Zero, Zero)>::deep_fold(e2);
    let result3 = DeepFoldable::<(Zero, Zero, Zero)>::deep_fold(e3);
    
    assert_eq!(result1, 42);
    assert_eq!(result2, 42);
    assert_eq!(result3, 42);
}

#[test]
fn test_either3_nested_fold() {
    // Test deep_fold with nested Either3
    type E3Int = Either3<i32, i32, i32>;
    
    // Either3<Either3<i32, i32, i32>, i32, i32>
    let nested1: Either3<E3Int, i32, i32> = Either3::Left(Either3::Left(42));
    let nested2: Either3<E3Int, i32, i32> = Either3::Left(Either3::Middle(42));
    let nested3: Either3<E3Int, i32, i32> = Either3::Left(Either3::Right(42));
    let nested4: Either3<E3Int, i32, i32> = Either3::Middle(42);
    let nested5: Either3<E3Int, i32, i32> = Either3::Right(42);
    
    let result1 = <Either3<E3Int, i32, i32> as DeepFoldable<(Succ<Zero>, Zero, Zero)>>::deep_fold(nested1);
    let result2 = <Either3<E3Int, i32, i32> as DeepFoldable<(Succ<Zero>, Zero, Zero)>>::deep_fold(nested2);
    let result3 = <Either3<E3Int, i32, i32> as DeepFoldable<(Succ<Zero>, Zero, Zero)>>::deep_fold(nested3);
    let result4 = <Either3<E3Int, i32, i32> as DeepFoldable<(Succ<Zero>, Zero, Zero)>>::deep_fold(nested4);
    let result5 = <Either3<E3Int, i32, i32> as DeepFoldable<(Succ<Zero>, Zero, Zero)>>::deep_fold(nested5);
    
    assert_eq!(result1, 42);
    assert_eq!(result2, 42);
    assert_eq!(result3, 42);
    assert_eq!(result4, 42);
    assert_eq!(result5, 42);
}

#[test]
fn test_either3_middle_nested_fold() {
    // Test deep_fold with nested Either3 in middle position
    type E3Int = Either3<i32, i32, i32>;
    
    // Either3<i32, Either3<i32, i32, i32>, i32>
    let nested1: Either3<i32, E3Int, i32> = Either3::Left(42);
    let nested2: Either3<i32, E3Int, i32> = Either3::Middle(Either3::Left(42));
    let nested3: Either3<i32, E3Int, i32> = Either3::Middle(Either3::Middle(42));
    let nested4: Either3<i32, E3Int, i32> = Either3::Middle(Either3::Right(42));
    let nested5: Either3<i32, E3Int, i32> = Either3::Right(42);
    
    let result1 = <Either3<i32, E3Int, i32> as DeepFoldable<(Zero, Succ<Zero>, Zero)>>::deep_fold(nested1);
    let result2 = <Either3<i32, E3Int, i32> as DeepFoldable<(Zero, Succ<Zero>, Zero)>>::deep_fold(nested2);
    let result3 = <Either3<i32, E3Int, i32> as DeepFoldable<(Zero, Succ<Zero>, Zero)>>::deep_fold(nested3);
    let result4 = <Either3<i32, E3Int, i32> as DeepFoldable<(Zero, Succ<Zero>, Zero)>>::deep_fold(nested4);
    let result5 = <Either3<i32, E3Int, i32> as DeepFoldable<(Zero, Succ<Zero>, Zero)>>::deep_fold(nested5);
    
    assert_eq!(result1, 42);
    assert_eq!(result2, 42);
    assert_eq!(result3, 42);
    assert_eq!(result4, 42);
    assert_eq!(result5, 42);
}

#[test]
fn test_either3_right_nested_fold() {
    // Test deep_fold with nested Either3 in right position
    type E3Int = Either3<i32, i32, i32>;
    
    // Either3<i32, i32, Either3<i32, i32, i32>>
    let nested1: Either3<i32, i32, E3Int> = Either3::Left(42);
    let nested2: Either3<i32, i32, E3Int> = Either3::Middle(42);
    let nested3: Either3<i32, i32, E3Int> = Either3::Right(Either3::Left(42));
    let nested4: Either3<i32, i32, E3Int> = Either3::Right(Either3::Middle(42));
    let nested5: Either3<i32, i32, E3Int> = Either3::Right(Either3::Right(42));
    
    let result1 = <Either3<i32, i32, E3Int> as DeepFoldable<(Zero, Zero, Succ<Zero>)>>::deep_fold(nested1);
    let result2 = <Either3<i32, i32, E3Int> as DeepFoldable<(Zero, Zero, Succ<Zero>)>>::deep_fold(nested2);
    let result3 = <Either3<i32, i32, E3Int> as DeepFoldable<(Zero, Zero, Succ<Zero>)>>::deep_fold(nested3);
    let result4 = <Either3<i32, i32, E3Int> as DeepFoldable<(Zero, Zero, Succ<Zero>)>>::deep_fold(nested4);
    let result5 = <Either3<i32, i32, E3Int> as DeepFoldable<(Zero, Zero, Succ<Zero>)>>::deep_fold(nested5);
    
    assert_eq!(result1, 42);
    assert_eq!(result2, 42);
    assert_eq!(result3, 42);
    assert_eq!(result4, 42);
    assert_eq!(result5, 42);
}

#[test]
fn test_either3_all_nested_fold() {
    // Test deep_fold with nested Either3 in all positions
    type E3Int = Either3<i32, i32, i32>;
    
    // Either3<Either3<i32, i32, i32>, Either3<i32, i32, i32>, Either3<i32, i32, i32>>
    let nested1: Either3<E3Int, E3Int, E3Int> = Either3::Left(Either3::Left(42));
    let nested2: Either3<E3Int, E3Int, E3Int> = Either3::Left(Either3::Middle(42));
    let nested3: Either3<E3Int, E3Int, E3Int> = Either3::Left(Either3::Right(42));
    let nested4: Either3<E3Int, E3Int, E3Int> = Either3::Middle(Either3::Left(42));
    let nested5: Either3<E3Int, E3Int, E3Int> = Either3::Middle(Either3::Middle(42));
    let nested6: Either3<E3Int, E3Int, E3Int> = Either3::Middle(Either3::Right(42));
    let nested7: Either3<E3Int, E3Int, E3Int> = Either3::Right(Either3::Left(42));
    let nested8: Either3<E3Int, E3Int, E3Int> = Either3::Right(Either3::Middle(42));
    let nested9: Either3<E3Int, E3Int, E3Int> = Either3::Right(Either3::Right(42));
    
    let result1 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested1);
    let result2 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested2);
    let result3 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested3);
    let result4 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested4);
    let result5 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested5);
    let result6 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested6);
    let result7 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested7);
    let result8 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested8);
    let result9 = <Either3<E3Int, E3Int, E3Int> as DeepFoldable<(Succ<Zero>, Succ<Zero>, Succ<Zero>)>>::deep_fold(nested9);
    
    assert_eq!(result1, 42);
    assert_eq!(result2, 42);
    assert_eq!(result3, 42);
    assert_eq!(result4, 42);
    assert_eq!(result5, 42);
    assert_eq!(result6, 42);
    assert_eq!(result7, 42);
    assert_eq!(result8, 42);
    assert_eq!(result9, 42);
}



#[test]
fn test_either_with_string_values() {
    // Test deep_fold with string values
    let e1: Either<String, String> = Either::Left("hello".to_string());
    let e2: Either<String, String> = Either::Right("world".to_string());
    
    let result1 = DeepFoldable::<(Zero, Zero)>::deep_fold(e1);
    let result2 = DeepFoldable::<(Zero, Zero)>::deep_fold(e2);
    
    assert_eq!(result1, "hello");
    assert_eq!(result2, "world");
    
    // Test nested Either with strings
    let nested1: Either<Either<String, String>, String> = 
        Either::Left(Either::Left("nested".to_string()));
    let nested2: Either<Either<String, String>, String> = 
        Either::Left(Either::Right("hello".to_string()));
    let nested3: Either<Either<String, String>, String> = 
        Either::Right("world".to_string());
    
    let result1 = <Either<Either<String, String>, String> as DeepFoldable<(Succ<Zero>, Zero)>>::deep_fold(nested1);
    let result2 = <Either<Either<String, String>, String> as DeepFoldable<(Succ<Zero>, Zero)>>::deep_fold(nested2);
    let result3 = <Either<Either<String, String>, String> as DeepFoldable<(Succ<Zero>, Zero)>>::deep_fold(nested3);
    
    assert_eq!(result1, "nested");
    assert_eq!(result2, "hello");
    assert_eq!(result3, "world");
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
        .exactly_n::<2>("Need exactly 2");

    assert_eq!(parser.parse("aa"), Ok(("", Box::new(["a", "a"]))));
    assert_eq!(parser.parse("a"), Err(("a", "Need exactly 2")));
    assert_eq!(parser.parse("aaa"), Ok(("a", Box::new(["a", "a"]))));
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
fn test_bind() {
    let num = "123".make_literal_matcher("Not number");
    let parser = num.bind(|n| {
        n[0..2]
            .make_literal_matcher("Wrong length")
            .map(|len| len.to_string())
    });

    assert_eq!(parser.parse("12312"), Ok(("", "12".to_string())));
    assert_eq!(parser.parse("12"), Err(("12", "Not number")));
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
                if let Some(x) = maybe_rest { result.extend(x) }
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

