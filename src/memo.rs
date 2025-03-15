//! # Memoization for Parsers
//!
//! This module provides memoization functionality for parsers, enabling performance
//! optimization by caching parse results. This is particularly useful for recursive
//! parsers and complex grammars where the same input might be parsed multiple times.
//!
//! Memoization is implemented using the existing state system, allowing for efficient
//! caching of parse results while maintaining the functional design of the parser combinators.
//!
//! ## Example Usage
//!
//! ```rust
//! use friss::*;
//! use friss::memo::*;
//!
//! // Create a parser that might be expensive
//! let expensive_parser = "hello".make_literal_matcher("Expected hello");
//!
//! // Make it memoized
//! let memoized_parser = expensive_parser.memoize();
//!
//! // Now parsing the same input multiple times will use the cached result
//! let result1 = memoized_parser.parse("hello world");
//! let result2 = memoized_parser.parse("hello world"); // Uses cached result
//! ```

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::RefCell;

use crate::core::{Parser, ParserOutput};
use crate::state::{StateCarrier, StatefulParser};

/// A key for the memoization cache.
///
/// This represents a unique parsing position, which is used as a key for caching parse results.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MemoKey<I> {
    /// The input state at the point of parsing
    pub input: I,
    /// An optional parser identifier, used to distinguish between different parsers
    pub parser_id: Option<String>,
}

/// The result of a parse operation, stored in the memoization cache.
#[derive(Clone, Debug)]
pub enum MemoResult<I, O, E> {
    /// Successful parse result with remaining input and output
    Success(I, O),
    /// Failed parse result with remaining input and error
    Failure(I, E),
}

/// State type for memoization.
///
/// This stores the cache of parsing results for reuse.
#[derive(Debug, Clone, Default)]
pub struct MemoState<I, O, E> 
where 
    I: Clone + Hash + Eq,
    O: Clone,
    E: Clone,
{
    /// The cache of parse results
    pub cache: Rc<RefCell<HashMap<MemoKey<I>, MemoResult<I, O, E>>>>,
    /// An optional identifier for the parser, used for cache keys
    pub parser_id: Option<String>,
}

impl<I, O, E> MemoState<I, O, E>
where
    I: Clone + Hash + Eq,
    O: Clone,
    E: Clone,
{
    /// Creates a new memoization state.
    pub fn new() -> Self {
        MemoState {
            cache: Rc::new(RefCell::new(HashMap::new())),
            parser_id: None,
        }
    }

    /// Creates a new memoization state with a parser identifier.
    pub fn with_id(id: impl Into<String>) -> Self {
        MemoState {
            cache: Rc::new(RefCell::new(HashMap::new())),
            parser_id: Some(id.into()),
        }
    }
}

/// A memoized parser that caches its results.
///
/// This wraps a parser with memoization capabilities, storing results in a shared cache.
pub struct MemoizedParser<I, O, E, P>
where
    I: Clone + Hash + Eq + Parsable<E>,
    O: Clone,
    E: Clone,
    P: Parser<I, O, E>,
{
    /// The underlying parser
    parser: P,
    /// The state for caching results
    memo_state: MemoState<I, O, E>,
}

impl<I, O, E, P> MemoizedParser<I, O, E, P>
where
    I: Clone + Hash + Eq+ Parsable<E>,
    O: Clone,
    E: Clone,
    P: Parser<I, O, E>,
{
    /// Creates a new memoized parser.
    pub fn new(parser: P) -> Self {
        MemoizedParser {
            parser,
            memo_state: MemoState::new(),
        }
    }

    /// Creates a new memoized parser with a specific identifier.
    pub fn with_id(parser: P, id: impl Into<String>) -> Self {
        MemoizedParser {
            parser,
            memo_state: MemoState::with_id(id),
        }
    }
}

impl<I, O, E, P> Parser<I, O, E> for MemoizedParser<I, O, E, P>
where
    I: Clone + Hash + Eq + Parsable<E>,
    O: Clone + ParserOutput,
    E: Clone,
    P: Parser<I, O, E>,
{
    fn parse(&self, input: I) -> Result<(I, O), (I, E)> {
        let key = MemoKey {
            input: input.clone(),
            parser_id: self.memo_state.parser_id.clone(),
        };

        if let Some(result) = self.memo_state.cache.borrow().get(&key) {
            match result {
                MemoResult::Success(rest, output) => {
                    return Ok((rest.clone(), output.clone()));
                }
                MemoResult::Failure(rest, error) => {
                    return Err((rest.clone(), error.clone()));
                }
            }
        }

        let result = self.parser.parse(input.clone());

        match &result {
            Ok((rest, output)) => {
                self.memo_state.cache.borrow_mut().insert(
                    key,
                    MemoResult::Success(rest.clone(), output.clone()),
                );
            }
            Err((rest, error)) => {
                self.memo_state.cache.borrow_mut().insert(
                    key,
                    MemoResult::Failure(rest.clone(), error.clone()),
                );
            }
        }

        result
    }
}

/// A stateful memoized parser that uses a shared state for caching.
///
/// This combines the state transition capabilities of `StatefulParser` with memoization.
pub struct StatefulMemoizedParser<S, I, O, E, P, SuccessF, ErrorF>
where
    StateCarrier<S, I>: Parsable<E> + Eq + Hash + Clone,
    S: Default,
    I: Clone + Hash + Eq,
    O: Clone,
    E: Clone,
    P: Parser<StateCarrier<S, I>, O, E>,
    SuccessF: FnMut(MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, O, StateCarrier<S, I>) 
        -> (MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, O),
    ErrorF: FnMut(MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, E, StateCarrier<S, I>) 
        -> (MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, E),
{
    /// The underlying parser
    parser: P,
    /// Function to handle successful parse results and state transitions
    on_success: RefCell<SuccessF>,
    /// Function to handle parse failures and state transitions
    on_error: RefCell<ErrorF>,
    phantom:PhantomData<(S,I,O,E)>
}

impl<S, I, O, E, P, SuccessF, ErrorF> StatefulMemoizedParser<S, I, O, E, P, SuccessF, ErrorF>
where
    StateCarrier<S, I>: Parsable<E> + Eq + Hash + Clone,
    S: Default + Clone,
    I: Clone + Hash + Eq,
    O: Clone,
    E: Clone,
    P: Parser<StateCarrier<S, I>, O, E>,
    SuccessF: FnMut(MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, O, StateCarrier<S, I>) 
        -> (MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, O),
    ErrorF: FnMut(MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, E, StateCarrier<S, I>) 
        -> (MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, E),
{
    /// Creates a new stateful memoized parser.
    pub fn new(parser: P, success: SuccessF, error: ErrorF) -> Self {
        StatefulMemoizedParser {
            parser,
            on_success: RefCell::new(success),
            on_error: RefCell::new(error),
            phantom:PhantomData
        }
    }
}

impl<S, I, O, E, P, SuccessF, ErrorF> Parser<StateCarrier<MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>>, O, E> 
    for StatefulMemoizedParser<S, I, O, E, P, SuccessF, ErrorF>
where
    StateCarrier<MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>>: Parsable<E>,
    StateCarrier<S, I>: Parsable<E> + Eq + Hash + Clone,
    S: Default + Clone,
    I: Clone + Hash + Eq + Parsable<E>,
    O: Clone + ParserOutput,
    E: Clone,
    P: Parser<StateCarrier<S, I>, O, E>,
    SuccessF: FnMut(MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, O, StateCarrier<S, I>) 
        -> (MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, O),
    ErrorF: FnMut(MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, E, StateCarrier<S, I>) 
        -> (MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>, E),
{
    fn parse(
        &self, 
        input: StateCarrier<MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>>
    ) -> Result<
        (StateCarrier<MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>>, O),
        (StateCarrier<MemoState<StateCarrier<S, I>, O, E>, StateCarrier<S, I>>, E)
    > {
        let memo_state = input.state.clone();
        let inner_input = input.input.clone();
        let key = MemoKey {
            input: inner_input.clone(),
            parser_id: memo_state.parser_id.clone(),
        };
        if let Some(result) = memo_state.cache.borrow().get(&key) {
            match result {
                MemoResult::Success(rest, output) => {
                    let (new_memo_state, new_rest, new_output) = 
                        (self.on_success.borrow_mut())(
                            memo_state.clone(), 
                            rest.clone(), 
                            output.clone(), 
                            inner_input.clone()
                        );
                    return Ok((
                        StateCarrier {
                            state: new_memo_state,
                            input: new_rest,
                        },
                        new_output,
                    ));
                }
                MemoResult::Failure(rest, error) => {
                    let (new_memo_state, new_rest, new_error) = 
                        (self.on_error.borrow_mut())(
                            memo_state.clone(), 
                            rest.clone(), 
                            error.clone(), 
                            inner_input.clone()
                        );
                    return Err((
                        StateCarrier {
                            state: new_memo_state,
                            input: new_rest,
                        },
                        new_error,
                    ));
                }
            }
        }
        
        match self.parser.parse(inner_input.clone()) {
            Ok((rest, output)) => {
                memo_state.cache.borrow_mut().insert(
                    key,
                    MemoResult::Success(rest.clone(), output.clone()),
                );
                
                let (new_memo_state, new_rest, new_output) = 
                    (self.on_success.borrow_mut())(
                        memo_state, 
                        rest, 
                        output, 
                        inner_input
                    );
                
                Ok((
                    StateCarrier {
                        state: new_memo_state,
                        input: new_rest,
                    },
                    new_output,
                ))
            }
            Err((rest, error)) => {
                memo_state.cache.borrow_mut().insert(
                    key,
                    MemoResult::Failure(rest.clone(), error.clone()),
                );
                let (new_memo_state, new_rest, new_error) = 
                    (self.on_error.borrow_mut())(
                        memo_state, 
                        rest, 
                        error, 
                        inner_input
                    );
                
                Err((
                    StateCarrier {
                        state: new_memo_state,
                        input: new_rest,
                    },
                    new_error,
                ))
            }
        }
    }
}

/// Extension trait to add memoization capabilities to parsers.
pub trait MemoizableParser<I, O, E>: Parser<I, O, E> + Sized
where
    I: Clone + Hash + Eq + Parsable<E>,
    O: Clone + ParserOutput,
    E: Clone,
{
    /// Wraps the parser with memoization.
    ///
    /// The memoized parser will cache results based on input positions, avoiding redundant parsing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use friss::*;
    /// use friss::memo::*;
    ///
    /// let parser = "abc".make_literal_matcher("Expected abc").memoize();
    /// let result1 = parser.parse("abcdef");
    /// let result2 = parser.parse("abcdef"); // Uses cached result
    /// ```
    fn memoize(self) -> MemoizedParser<I, O, E, Self> {
        MemoizedParser::new(self)
    }
    
    /// Wraps the parser with memoization using a specific identifier.
    ///
    /// This allows different parsers to have separate caches even when parsing the same input.
    ///
    /// # Example
    ///
    /// ```rust
    /// use friss::*;
    /// use friss::memo::*;
    ///
    /// let parser = "abc".make_literal_matcher("Expected abc").memoize_with_id("abc_parser");
    /// ```
    fn memoize_with_id(self, id: impl Into<String>) -> MemoizedParser<I, O, E, Self> {
        MemoizedParser::with_id(self, id)
    }
}

impl<I, O, E, P> MemoizableParser<I, O, E> for P
where
    I: Clone + Hash + Eq + Parsable<E>,
    O: Clone + ParserOutput,
    E: Clone,
    P: Parser<I, O, E> + Sized,
{
}

use crate::core::Parsable;

impl<S, I, O, E, Error> Parsable<Error> for StateCarrier<MemoState<I, O, E>, S>
where

    S: Parsable<Error> + Clone,
    <S as Parsable<Error>>::Item : Clone,
    I: Clone + Hash + Eq,
    O: Clone,
    E: Clone,
    Error: Clone,
{
    type Item = S::Item;

    fn make_literal_matcher(self, err: Error) -> impl Parser<Self, Self, Error> {
        move |input: StateCarrier<MemoState<I, O, E>, S>| {
            let inner_result = input.input.clone().make_literal_matcher(err.clone()).parse(input.input);
            match inner_result {
                Ok((rest, ret)) => Ok((
                    StateCarrier {
                        state: input.state.clone(),
                        input: rest,
                    },
                    StateCarrier {
                        state: input.state,
                        input: ret,
                    },
                )),
                Err((rest, err)) => Err((
                    StateCarrier {
                        state: input.state,
                        input: rest,
                    },
                    err,
                )),
            }
        }
    }

    fn make_anything_matcher(err: Error) -> impl Parser<Self, Self::Item, Error> {
        move |input: StateCarrier<MemoState<I, O, E>, S>| {
            let inner_result = S::make_anything_matcher(err.clone()).parse(input.input);
            match inner_result {
                Ok((rest, ret)) => Ok((
                    StateCarrier {
                        state: input.state,
                        input: rest,
                    },
                    ret,
                )),
                Err((rest, err)) => Err((
                    StateCarrier {
                        state: input.state,
                        input: rest,
                    },
                    err,
                )),
            }
        }
    }

    fn make_item_matcher(character: Self::Item, err: Error) -> impl Parser<Self, Self::Item, Error> 
        
    {
        
        move |input: StateCarrier<MemoState<I, O, E>, S>| {
            let inner_result = S::make_item_matcher(character.clone(), err.clone()).parse(input.input);
            match inner_result {
                Ok((rest, ret)) => Ok((
                    StateCarrier {
                        state: input.state,
                        input: rest,
                    },
                    ret,
                )),
                Err((rest, err)) => Err((
                    StateCarrier {
                        state: input.state,
                        input: rest,
                    },
                    err,
                )),
            }
        }
    }

    fn make_empty_matcher(err: Error) -> impl Parser<Self, (), Error> {
        move |input: StateCarrier<MemoState<I, O, E>, S>| {
            let inner_result = S::make_empty_matcher(err.clone()).parse(input.input);
            match inner_result {
                Ok((rest, ret)) => Ok((
                    StateCarrier {
                        state: input.state,
                        input: rest,
                    },
                    ret,
                )),
                Err((rest, err)) => Err((
                    StateCarrier {
                        state: input.state,
                        input: rest,
                    },
                    err,
                )),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::recursive;
    use crate::*;

    // Test basic memoization
    #[test]
    fn test_basic_memoization<'a>() {
        // Create a counter to track how many times the parser is actually run
        let counter = Rc::new(RefCell::new(0));
        let counter_clone = counter.clone();
        
        // Create a parser that increments the counter when it runs
        let counting_parser = move |input: &'a str| {
            *counter_clone.borrow_mut() += 1;
            "hello".make_literal_matcher("Expected hello").parse(input)
        };
        
        // Memoize the parser
        let memoized = counting_parser.memoize();
        
        // Run the parser multiple times on the same input
        let _ = memoized.parse("hello world");
        let _ = memoized.parse("hello world");
        let _ = memoized.parse("hello world");
        
        // The counter should only be incremented once
        assert_eq!(*counter.borrow(), 1);
        
        // Run the parser on different input
        let _ = memoized.parse("hello there");
        
        // The counter should now be incremented again
        assert_eq!(*counter.borrow(), 2);
    }
    
    // Test memoization with recursive parsing
    #[test]
    fn test_recursive_memoization() {
        // Create a counter to track parser executions
        let counter = Rc::new(RefCell::new(0));
        let counter_clone = counter.clone();
        
        // Create a recursive parser for balanced parentheses
        let paren_parser: Box<dyn Parser<&'static str, i32, &'static str>> = recursive(move  |parser| {

            //let b = parser.rc();
            // Capture the counter
            let counter = counter_clone.clone();
            
            // Create a parser that counts executions
            let counting = move  |input: &'static str| {
                *counter.borrow_mut() += 1;
                
                // First explicitly create the parser for nested parentheses
                let nested = 
                    '('.make_character_matcher("Expected opening paren")
                        .seq( |x| parser.parse(x))
                        .map_err(|x| x.fold())
                        .seq(')'.make_character_matcher("Expected closing paren"))
                        .map_err(|x| x.fold())
                        .map(|((_, inner), _)| inner + 1);
                

                // Then separately create the empty parser
                let empty = "".make_literal_matcher("").map(|_| 0);

                // Now combine them
                nested
                    .alt(empty)
                    .map_err(|(a, _b)| a)
                    .map(|either| match either {
                        Either::Left(depth) => depth,
                        Either::Right(empty_result) => empty_result,
                    })
                    .parse(input)
            };
            
            // Memoize the parser
            Box::new(counting.memoize_with_id("paren"))
        });
        
        // Parse a complex nested structure
        let result = paren_parser.parse("(((())))");
        assert_eq!(result, Ok(("", 4)));
        
        // Without memoization, this would require many more parser executions
        // We're expecting one execution per unique input position, plus one for the initial call
        assert!(*counter.borrow() <= 10);
    }
}
