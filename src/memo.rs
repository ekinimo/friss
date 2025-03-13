
//TODO this needs a serious recheck

//! # Memoization for Parser Combinators
//!
//! Implements packrat parsing (memoized recursive descent parsing) 
//! using the state machine infrastructure from the library.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::core::{Parsable, Parser};
use crate::state::StateMachine;

/// Memoization cache for parsing results
#[derive(Clone, Debug)]
pub struct MemoCache<Input, Output, Error>
where
    Input: Clone + Eq + Hash,
    Error: Clone,
    Output: Clone,
{
    /// Shared internal cache mapping input positions to parsing results
    cache: Rc<RefCell<HashMap<(Input, usize), Result<(Input, Output), (Input, Error)>>>>,
}

impl<Input, Output, Error> MemoCache<Input, Output, Error>
where
    Input: Clone + Eq + Hash,
    Error: Clone,
    Output: Clone,
{
    /// Create a new, shared memoization cache
    pub fn new() -> Self {
        MemoCache {
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Check if a result is already memoized for a given input
    pub fn get(
        &self,
        input: &Input,
        rule_id: usize,
    ) -> Option<Result<(Input, Output), (Input, Error)>> {
        self.cache.borrow().get(&(input.clone(), rule_id)).cloned()
    }

    /// Memoize a parsing result
    pub fn insert(
        &self,
        input: Input,
        rule_id: usize,
        result: Result<(Input, Output), (Input, Error)>,
    ) {
        self.cache.borrow_mut().insert((input, rule_id), result);
    }
}

/// State machine for memoization tracking
pub struct MemoStateMachine<Input, Output, Error>
where
    Input: Clone + Eq + Hash,
    Error: Clone,
    Output: Clone,
{
    /// Unique identifier for the parsing rule
    rule_id: usize,
    /// Memoization cache
    cache: MemoCache<Input, Output, Error>,
    _phantom: PhantomData<(Input, Output, Error)>,
}

impl<Input, Output, Error> MemoStateMachine<Input, Output, Error>
where
    Input: Clone + Eq + Hash,
    Error: Clone,
    Output: Clone,
{
    /// Create a new memoization state machine
    pub fn new(rule_id: usize) -> Self {
        MemoStateMachine {
            rule_id,
            cache: MemoCache::new(),
            _phantom: PhantomData,
        }
    }
}

impl<Input, Output, Error> StateMachine<Input, MemoCache<Input, Output, Error>>
    for MemoStateMachine<Input, Output, Error>
where
    Input: Clone + Eq + Hash,
    Error: Clone,
    Output: Clone,
{
    fn update(
        &self,
        _state: &mut MemoCache<Input, Output, Error>,
        _consumed: &Input,
        _remaining: &Input,
    ) {
        // No state update needed for pure memoization
    }

    fn initial_state(&self) -> MemoCache<Input, Output, Error> {
        self.cache.clone()
    }
}

/// Extension trait for memoized parsing
pub trait MemoizedParserExt<Input, Output, Error>: Parser<Input, Output, Error>
where
    Input: Clone + Eq + Hash + Parsable<Error>,
    Error: Clone,
    Output: Clone,
    Self: Sized,
{
    /// Add memoization to the parser
    fn memoize(self) -> impl Parser<Input, Output, Error> {
        // Generate a unique rule ID based on the parser's memory address
        // Not sure if this is kosher

        let rule_id = std::ptr::addr_of!(self) as usize;
        
        // Create a shared cache
        let cache = MemoCache::new();

        move |input: Input| {
            // Check if result is already memoized
            if let Some(result) = cache.get(&input, rule_id) {
                return result;
            }

            // Parse the input and memoize the result
            let result = self.parse(input.clone());
            cache.insert(input, rule_id, result.clone());

            result
        }
    }
}

impl<P, Input, Output, Error> MemoizedParserExt<Input, Output, Error> for P
where
    P: Parser<Input, Output, Error>,
    Input: Clone + Eq + Hash + Parsable<Error>,
    Error: Clone,
    Output: Clone,
{
}

/// Recursive parser with built-in memoization
pub fn memoized_recursive<Input, Output, Error, F>(f: F) -> Box<dyn Parser<Input, Output, Error>>
where
    Input: Clone + Eq + Hash + Parsable<Error> + 'static,
    Output: Clone + 'static,
    Error: Clone + 'static,
    F: Fn(Box<dyn Parser<Input, Output, Error>>) -> Box<dyn Parser<Input, Output, Error>> + 'static,
{
    // Generate a unique rule ID
    let rule_id = std::ptr::addr_of!(f) as usize;

    // Create a shared memoization cache
    let cache = MemoCache::new();
    let new_c = cache.clone();
    // Create a recursive parser with memoization
    let cell: Rc<RefCell<Option<Box<dyn Parser<Input, Output, Error>>>>> =
        Rc::new(RefCell::new(None));

    let cell_for_placeholder = cell.clone();

    let placeholder: Box<dyn Parser<Input, Output, Error>> = Box::new(move |input: Input| {
        
        // Check if result is already memoized
        if let Some(result) = cache.get(&input, rule_id) {
            return result;
        }


        // Borrow the inner parser and delegate to it
        let borrowed = cell_for_placeholder.as_ref().borrow();
        let result = match &*borrowed {
            Some(parser) => parser.parse(input.clone()),
            None => panic!("Recursive parser used before being initialized"),
        };

        // Memoize the result
        cache.insert(input, rule_id, result.clone());

        result
    });

    
    let actual = f(placeholder);

    
    *cell.as_ref().borrow_mut() = Some(actual);
    
    Box::new(move |input: Input| {
        // Check if result is already memoized
        if let Some(result) = new_c.get(&input, rule_id) {
            return result;
        }

        let borrowed = cell.as_ref().borrow();
        let result = match &*borrowed {
            Some(parser) => parser.parse(input.clone()),
            None => panic!("Recursive parser not initialized"),
        };

        // Memoize the result
        new_c.insert(input, rule_id, result.clone());

        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::Either;
    use crate::types::Foldable;

    /// Test memoization of a simple parser
    #[test]
    fn test_basic_memoization<'a>() {
        // Keep track of how many times the parser is actually called
        let call_count = Rc::new(RefCell::new(0));
        let call_count_1 = call_count.clone();
        // Create a parser that increments a call counter
        let counter_parser = move |input: &'a str| {
            let call_count = call_count_1.clone();
            *call_count.borrow_mut() += 1;
            if input.starts_with("hello") {
                Ok((&input[5..], "hello"))
            } else {
                Err((input, "Not hello"))
            }
        };

        // Memoize the parser
        let memoized = counter_parser.memoize();

        // First parse
        let result1 = memoized.parse("hello world");
        assert_eq!(result1, Ok((" world", "hello")));
        assert_eq!(*call_count.borrow(), 1);

        // Second parse with same input - should use memoized result
        let result2 = memoized.parse("hello world");
        assert_eq!(result2, Ok((" world", "hello")));
        // Call count should not increase if memoization works
        assert_eq!(*call_count.borrow(), 1);
    }

    /// Test memoization in recursive parsers
    #[test]
    fn test_recursive_memoization() {
        // Simulate a computationally expensive recursive parser for nested parentheses
        let call_count = Rc::new(RefCell::new(0));
        let call_count_1 = call_count.clone();
        let paren_parser: Box<dyn Parser<&str, i32, &str>> = memoized_recursive(move |parser| {
            let call_count_1 = call_count_1.clone();
            let nested = Box::new(
                '('.make_character_matcher("Expected opening paren")
                    .seq(move |x| parser.parse(x))
                    .map_err(|x| x.fold())
                    .seq(')'.make_character_matcher("Expected closing paren"))
                    .map_err(|x| x.fold())
                    .map(move |((_, inner), _)| {
                        *call_count_1.borrow_mut() += 1;
                        inner + 1
                    }),
            );

            let empty = "".make_literal_matcher("").map(|_| 0);

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

        // Nested parentheses parsing - test memoization
        let test_cases = vec![
            "((()))",
            "((()))",  // Repeat to check memoization
            "(((())))",
            "(((())))",  // Repeat to check memoization
        ];

        let expected_results = vec![3, 3, 4, 4];
        let expected_calls = vec![3, 0, 4, 0];

        for (i, input) in test_cases.into_iter().enumerate() {
            *call_count.borrow_mut() = 0;
            let result = paren_parser.parse(input);
            
            assert_eq!(result, Ok(("", expected_results[i])), "Failed for input: {} at {i}", input);
            assert_eq!(*call_count.borrow(), expected_calls[i], "Unexpected call count for input: {} at {i}", input);
        }
    }
}
