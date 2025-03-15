//! # Packrat Parser Implementation
//!
//! This module provides a complete implementation of the Packrat parsing algorithm
//! for handling left-recursive grammars, based on the algorithm described by
//! Warth, Douglass, and Millstein in their paper "Packrat Parsers Can Support Left Recursion".

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::core::{Parsable, Parser, ParserOutput};

/// Represents the growth status of a left-recursive parser.
#[derive(Debug, Clone, PartialEq)]
enum GrowthStatus {
    /// The parser has matched more input.
    Growing,
    /// The parser has reached a fixed point and is no longer growing.
    Stable,
    /// The parser is not involved in left recursion.
    NotInvolved,
}

/// A key for the packrat cache.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PackratKey<I: 'static + Clone + Hash + Eq> {
    /// The input position
    pub input: I,
    /// The parser identifier
    pub parser_id: String,
}

/// The result of a parse operation, stored in the packrat cache.
#[derive(Clone, Debug)]
pub enum PackratResult<I, O, E> {
    /// Successful parse result with remaining input and output
    Success(I, O),
    /// Failed parse result with remaining input and error
    Failure(I, E),
    /// Parser is still evaluating (prevents infinite recursion)
    Evaluating,
}

/// State for tracking left recursion for a specific rule.
#[derive(Clone, Debug)]
pub struct RecursionState<I: 'static + Clone + Hash + Eq> {
    /// Whether this rule involves left recursion
    pub involved: bool,
    /// The input position where left recursion was detected
    pub input_pos: I,
    /// The set of rules involved in this left recursion
    pub involved_set: HashSet<String>,
    /// The current growth status for this rule
    pub status: GrowthStatus,
}

/// State for packrat parsing with left recursion support.
#[derive(Clone)]
pub struct PackratState<I, O, E>
where
    I: 'static + Clone + Hash + Eq,
    O: Clone,
    E: Clone + 'static,
{
    /// The parse result cache
    pub memo_table: Rc<RefCell<HashMap<PackratKey<I>, PackratResult<I, O, E>>>>,
    /// The left recursion state for each rule
    pub recursion_state: Rc<RefCell<HashMap<String, RecursionState<I>>>>,
    /// The call stack for detecting recursive rules
    pub call_stack: Rc<RefCell<Vec<String>>>,
}

impl<I, O, E> PackratState<I, O, E>
where
    I: 'static + Clone + Hash + Eq,
    O: Clone,
    E: Clone + 'static,
{
    /// Creates a new packrat state.
    pub fn new() -> Self {
        PackratState {
            memo_table: Rc::new(RefCell::new(HashMap::new())),
            recursion_state: Rc::new(RefCell::new(HashMap::new())),
            call_stack: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Records that we're entering a parser rule.
    pub fn enter_rule(&self, rule_id: &str) {
        self.call_stack.borrow_mut().push(rule_id.to_string());
    }

    /// Records that we're exiting a parser rule.
    pub fn exit_rule(&self) {
        self.call_stack.borrow_mut().pop();
    }

    /// Checks if we're currently in a left-recursive call.
    pub fn is_left_recursive(&self, rule_id: &str) -> bool {
        self.call_stack.borrow().contains(&rule_id.to_string())
    }

    /// Gets the current call stack.
    pub fn get_call_stack(&self) -> Vec<String> {
        self.call_stack.borrow().clone()
    }
}

impl<I, O, E> Default for PackratState<I, O, E>
where
    I: 'static + Clone + Hash + Eq,
    O: Clone,
    E: Clone + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A packrat parser that can handle left recursion.
///
/// This parser implementation uses the algorithm described by Warth, Douglass, and Millstein
/// to support left-recursive grammars without infinite recursion.
///
/// # Type Parameters
///
/// * `I` - The input type
/// * `O` - The output type
/// * `E` - The error type
/// * `P` - The underlying parser type
///
/// # Example
///
/// ```rust
/// use friss::*;
/// use friss::packrat::*;
///
/// ```
pub struct PackratParserImpl<I, O, E, P>
where
    I: 'static + Clone + Hash + Eq + Parsable<E>,
    O: Clone,
    E: Clone + 'static,
    P: Parser<I, O, E>,
{
    /// The underlying parser
    parser: P,
    /// The packrat state for the parser
    state: PackratState<I, O, E>,
    /// Unique identifier for this parser rule
    rule_id: String,
}

impl<I, O, E, P> PackratParserImpl<I, O, E, P>
where
    I: Clone + Hash + Eq + Parsable<E> + 'static,
    O: Clone,
    E: Clone + 'static,
    P: Parser<I, O, E>,
{
    /// Creates a new packrat parser.
    pub fn new(parser: P, rule_id: impl Into<String>) -> Self {
        PackratParserImpl {
            parser,
            state: PackratState::new(),
            rule_id: rule_id.into(),
        }
    }

    /// Creates a new packrat parser with a shared state.
    pub fn with_state(parser: P, rule_id: impl Into<String>, state: PackratState<I, O, E>) -> Self {
        PackratParserImpl {
            parser,
            state,
            rule_id: rule_id.into(),
        }
    }

    /// Implementation of the packrat algorithm with left recursion.
    fn packrat_parse(&self, input: I) -> Result<(I, O), (I, E)> {
        let rule_id = self.rule_id.clone();
        let key = PackratKey {
            input: input.clone(),
            parser_id: rule_id.clone(),
        };

        // Check if we have a result in the memo table
        let memo_result = self.state.memo_table.borrow().get(&key).cloned();

        if let Some(result) = memo_result {
            match result {
                PackratResult::Success(rest, output) => return Ok((rest, output)),
                PackratResult::Failure(rest, error) => return Err((rest, error)),
                PackratResult::Evaluating => {
                    // Left recursion detected

                    // Create initial recursion state if needed
                    if !self.state.recursion_state.borrow().contains_key(&rule_id) {
                        let mut involved_set = HashSet::new();
                        involved_set.insert(rule_id.clone());

                        let recursion_state = RecursionState {
                            involved: true,
                            input_pos: input.clone(),
                            involved_set,
                            status: GrowthStatus::Growing,
                        };

                        self.state
                            .recursion_state
                            .borrow_mut()
                            .insert(rule_id.clone(), recursion_state);

                        // Initial failure to bootstrap recursive parsing
                        return Err((input, self.create_error("Left-recursive rule encountered")));
                    } else {
                        // Get the current result for left recursion
                        let recursion_state = self
                            .state
                            .recursion_state
                            .borrow()
                            .get(&rule_id)
                            .cloned()
                            .expect("Recursion state should exist at this point");

                        // If we're in a growing state, apply the current best result
                        if recursion_state.status == GrowthStatus::Growing {
                            // Try to find a successful result in the memo table
                            for (memo_key, memo_result) in self.state.memo_table.borrow().iter() {
                                if memo_key.parser_id == rule_id
                                    && matches!(memo_result, PackratResult::Success(..))
                                {
                                    match memo_result {
                                        PackratResult::Success(rest, output) => {
                                            return Ok((rest.clone(), output.clone()));
                                        }
                                        _ => {
                                            unreachable!("Already checked this is a success result")
                                        }
                                    }
                                }
                            }

                            // No successful result yet, return failure
                            return Err((
                                input,
                                self.create_error("Left-recursive rule not yet resolved"),
                            ));
                        } else {
                            // We've reached a fixed point, this is a failure
                            return Err((
                                input,
                                self.create_error("Left-recursive rule reached fixed point"),
                            ));
                        }
                    }
                }
            }
        }

        // Check for left recursion
        if self.state.is_left_recursive(&rule_id) {
            // Mark that we're evaluating this rule
            self.state
                .memo_table
                .borrow_mut()
                .insert(key, PackratResult::Evaluating);

            return Err((input, self.create_error("Left-recursive rule detected")));
        }

        // Enter this rule
        self.state.enter_rule(&rule_id);

        // Mark that we're evaluating this rule
        self.state
            .memo_table
            .borrow_mut()
            .insert(key.clone(), PackratResult::Evaluating);

        // Attempt to parse
        let result = self.parser.parse(input.clone());

        // Process the result
        let final_result = match &result {
            Ok((rest, output)) => {
                // Success: store in memo table
                self.state.memo_table.borrow_mut().insert(
                    key.clone(),
                    PackratResult::Success(rest.clone(), output.clone()),
                );

                // If this is part of a left-recursive rule, try to grow the result
                if let Some(mut recursion_state) =
                    self.state.recursion_state.borrow_mut().get_mut(&rule_id)
                {
                    if recursion_state.involved && recursion_state.status == GrowthStatus::Growing {
                        // Enter growth stage - repeatedly apply the rule until it stops growing
                        let mut previous_rest = rest.clone();
                        let mut previous_output = output.clone();

                        loop {
                            // Update memo table with current best result
                            self.state.memo_table.borrow_mut().insert(
                                key.clone(),
                                PackratResult::Success(
                                    previous_rest.clone(),
                                    previous_output.clone(),
                                ),
                            );

                            // Try parsing again
                            match self.parser.parse(input.clone()) {
                                Ok((new_rest, new_output)) => {
                                    // If we matched more input, continue growing
                                    if is_more_matched(&previous_rest, &new_rest) {
                                        previous_rest = new_rest;
                                        previous_output = new_output;
                                    } else {
                                        // Reached a fixed point
                                        recursion_state.status = GrowthStatus::Stable;
                                        break;
                                    }
                                }
                                Err(_) => {
                                    // Failed to grow further
                                    recursion_state.status = GrowthStatus::Stable;
                                    break;
                                }
                            }
                        }

                        // Return the best result
                        Ok((previous_rest, previous_output))
                    } else {
                        result.clone()
                    }
                } else {
                    result.clone()
                }
            }
            Err((rest, error)) => {
                // Failure: store in memo table
                self.state
                    .memo_table
                    .borrow_mut()
                    .insert(key, PackratResult::Failure(rest.clone(), error.clone()));

                result.clone()
            }
        };

        // Exit this rule
        self.state.exit_rule();

        final_result
    }

    /// Creates an error value for packrat-specific errors.
    fn create_error(&self, message: &str) -> E {
        // For string error types, we can simply use the message
        // For other error types, we need to convert the string to the appropriate error type
        self.string_to_error(message)
    }

    /// Converts a string to an error value.
    /// This method should be specialized based on the error type E.
    fn string_to_error(&self, message: &str) -> E {
        // This implementation handles common error types
        use std::any::TypeId;

        let error_type_id = TypeId::of::<E>();

        if error_type_id == TypeId::of::<&str>() {
            // String slice errors
            unsafe { std::mem::transmute_copy(&message) }
        } else if error_type_id == TypeId::of::<String>() {
            // String errors
            unsafe { std::mem::transmute_copy(&message.to_string()) }
        } else {
            // For custom error types, we need to implement a conversion
            // This panic will help catch cases where this hasn't been properly implemented
            panic!("Cannot convert string to error type. Implement a custom error conversion for your error type.")
        }
    }
}

impl<I, O, E, P> Parser<I, O, E> for PackratParserImpl<I, O, E, P>
where
    I: 'static + Clone + Hash + Eq + Parsable<E>,
    O: Clone + ParserOutput,
    E: Clone + 'static,
    P: Parser<I, O, E>,
{
    fn parse(&self, input: I) -> Result<(I, O), (I, E)> {
        self.packrat_parse(input)
    }
}

/// Extension trait to add packrat parsing capabilities to parsers.
pub trait PackratParser<I, O, E>: Parser<I, O, E> + Sized
where
    I: 'static + Clone + Hash + Eq + Parsable<E>,
    O: Clone + ParserOutput,
    E: Clone + 'static,
{
    /// Wraps the parser with packrat parsing capabilities for handling left recursion.
    ///
    /// # Arguments
    ///
    /// * `rule_id` - A unique identifier for this parser rule
    ///
    /// # Returns
    ///
    /// A packrat parser that can handle left recursion
    ///
    /// # Example
    ///
    /// ```rust
    /// use friss::*;
    /// use friss::packrat::*;
    ///
    /// #[derive(Clone)]
    /// enum Op { Add}
    ///
    /// #[derive(Clone)]
    /// enum Expr { Number(f64),BinaryOp(Box<Expr>, Op, Box<Expr>), }
    ///
    /// // Create a left-recursive expression parser
    /// let expr: Box<dyn Parser<&'static str, Expr, &'static str>> = recursive(|expr| {
    ///     let term = move |i : &'static str| {"num".make_literal_matcher("Expected number")
    ///         .map(|_| Expr::Number(1.0)).parse(i)};
    ///    let term2 = term.clone();
    ///     // Left recursion: expr -> expr "+" term | term
    ///     
    ///     Box::new(
    ///
    ///         (move | i :&'static str| expr.parse(i))
    ///             .seq("+".make_literal_matcher("Expected +").seq( move |i :&'static str| term.parse(i)))
    ///             .map(|(e, (_, t))| Expr::BinaryOp(Box::new(e), Op::Add, Box::new(t)))
    ///             .alt(term2)
    ///             .map(|x| x.fold())
    ///             .map_err(|_| "term failed")
    ///             .packrat("expr")
    ///     )
    /// });
    /// ```
    fn packrat(self, rule_id: impl Into<String>) -> PackratParserImpl<I, O, E, Self> {
        PackratParserImpl::new(self, rule_id)
    }

    /// Wraps the parser with packrat parsing capabilities, using a shared state.
    ///
    /// # Arguments
    ///
    /// * `rule_id` - A unique identifier for this parser rule
    /// * `state` - The shared packrat state
    ///
    /// # Returns
    ///
    /// A packrat parser that can handle left recursion using the shared state
    fn packrat_with_state(
        self,
        rule_id: impl Into<String>,
        state: PackratState<I, O, E>,
    ) -> PackratParserImpl<I, O, E, Self> {
        PackratParserImpl::with_state(self, rule_id, state)
    }
}

// Implement PackratParser for all parsers
impl<I, O, E, P> PackratParser<I, O, E> for P
where
    I: 'static + Clone + Hash + Eq + Parsable<E>,
    O: Clone + ParserOutput,
    E: Clone + 'static,
    P: Parser<I, O, E> + Sized,
{
}

/// Helper function to determine if a parse result has matched more input.
///
/// We consider that more input has been matched if the length of the
/// remaining input is shorter.
fn is_more_matched<I: 'static>(prev_rest: &I, new_rest: &I) -> bool {
    // Use specialized implementations for common input types
    if let Some(prev) = input_len(prev_rest) {
        if let Some(new) = input_len(new_rest) {
            return new < prev;
        }
    }

    // Default to false if we can't determine lengths
    false
}

/// Gets the length of an input if possible.
/// This handles common input types and can be extended for custom types.
fn input_len<I: 'static>(input: &I) -> Option<usize> {
    use std::any::TypeId;

    let type_id = TypeId::of::<I>();

    if type_id == TypeId::of::<&str>() {
        // String slice
        let s = unsafe { &*(input as *const I as *const &str) };
        Some(s.len())
    } else if type_id == TypeId::of::<String>() {
        // String
        let s = unsafe { &*(input as *const I as *const String) };
        Some(s.len())
    } else if type_id == TypeId::of::<&[u8]>() {
        // Byte slice
        let s = unsafe { &*(input as *const I as *const &[u8]) };
        Some(s.len())
    } else if type_id == TypeId::of::<Vec<u8>>() {
        // Byte vector
        let s = unsafe { &*(input as *const I as *const Vec<u8>) };
        Some(s.len())
    } else {
        // Try to get length through reflection (limited to types that match this pattern)
        let ptr = input as *const I;
        let ptr_usize = ptr as usize;

        // Check if this is a slice or vector-like type with a length field
        // This is unsafe and relies on common Rust memory layouts
        if ptr_usize != 0 {
            // Try to read length at common offset (this works for many Rust collection types)
            let length_ptr = (ptr_usize + std::mem::size_of::<usize>()) as *const usize;
            if length_ptr as usize != 0 {
                let length = unsafe { *length_ptr };
                if length < 1_000_000_000 {
                    // Sanity check to avoid garbage values
                    return Some(length);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::recursive;
    use crate::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Define a simple expression AST for testing
    #[derive(Debug, Clone, PartialEq)]
    enum Expr {
        Number(f64),
        Variable(String),
        BinaryOp(Box<Expr>, Op, Box<Expr>),
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Op {
        Add,
        Mul,
    }

    // Helper function to create a number parser
    fn number_parser<'a>() -> impl Parser<&'a str, Expr, &'static str> {
        (
            "0".make_literal_matcher("Expected digit"),
            "1".make_literal_matcher("Expected digit"),
            "2".make_literal_matcher("Expected digit"),
            "3".make_literal_matcher("Expected digit"),
            "4".make_literal_matcher("Expected digit"),
            "5".make_literal_matcher("Expected digit"),
            "6".make_literal_matcher("Expected digit"),
            "7".make_literal_matcher("Expected digit"),
            "8".make_literal_matcher("Expected digit"),
            "9".make_literal_matcher("Expected digit"),
        )
            .alt()
            .map_err(|_| "Expected number")
            .map(|s| s.fold())
            .map(|s| Expr::Number(s.parse::<f64>().unwrap()))
    }

    // Helper function to create a variable parser
    fn variable_parser<'a>() -> impl Parser<&'a str, Expr, &'static str> {
        (
            "x".make_literal_matcher("Expected variable"),
            "y".make_literal_matcher("Expected variable"),
            "z".make_literal_matcher("Expected variable"),
        )
            .alt()
            .map_err(|_| "Expected variable")
            .map(|s| s.fold())
            .map(|s| Expr::Variable(s.to_string()))
    }

    // Test basic packrat parsing (non-recursive)
    #[test]
    fn test_packrat_basic() {
        // Create a simple parser for a number or variable
        let term = number_parser()
            .alt(variable_parser())
            .map_err(|_| "Expected term");

        // Wrap it with packrat parsing
        let packrat_term = term.map(|x| x.fold()).packrat("term");

        // Test parsing a number
        let result = packrat_term.parse("5");
        assert_eq!(result, Ok(("", Expr::Number(5.0))));

        // Test parsing a variable
        let result = packrat_term.parse("x");
        assert_eq!(result, Ok(("", Expr::Variable("x".to_string()))));

        // Test parsing an invalid input
        let result = packrat_term.parse("a");
        assert!(result.is_err());
    }

    // Test counting parser executions with memoization
    #[test]
    fn test_packrat_memoization() {
        // Create a counter to track parser executions
        let counter = Rc::new(RefCell::new(0));
        let counter_clone = counter.clone();

        // Create a parser that increments the counter when executed
        let counting_parser = move |input: &'static str| {
            *counter_clone.borrow_mut() += 1;
            "x".make_literal_matcher("Expected x").parse(input)
        };

        // Wrap it with packrat parsing
        let packrat_parser = counting_parser.packrat("counter");

        // Parse the same input multiple times
        let _ = packrat_parser.parse("x");
        let _ = packrat_parser.parse("x");
        let _ = packrat_parser.parse("x");

        // The counter should only be incremented once due to memoization
        assert_eq!(*counter.borrow(), 1);
    }


    /*
    #[test]
    fn test_left_recursive_expression() {
        // Create a left-recursive expression parser
        let expr_parser: Box<dyn Parser<&'static str, Expr, &'static str>> =
            recursive(|expr_parser| {
                // Term parser for primary expressions (numbers or variables)
                let term = |input: &'static str| {
                    number_parser()
                        .alt(variable_parser())
                        .map_err(|_| "Expected term").parse(input)
                };

                let term2 = |input: &'static str| {
                    number_parser()
                        .alt(variable_parser())
                        .map_err(|_| "Expected term").parse(input)
                };

                //let expr = crate::core::FnOnceWrapper::new( move  |input: &'static str| expr_parser.parse(input));
                
                // Create left-recursive parsers for addition and multiplication

                // Addition: expr + term
                let add = (
                    move  |input: &'static str| expr_parser.parse(input),
                    "+".make_literal_matcher("Expected +"),
                    move |input: &'static str| term.parse(input),
                )
                    .seq()
                    .map(|(left, _, right)| {
                        Expr::BinaryOp(Box::new(left), Op::Add, Box::new(right.fold()))
                    })
                    .map_err(|_| "Expected addition expression");

                // Multiplication: expr * term
                let mul = (
                    (move |input: &'static str| number_parser().parse(input)),
                    "*".make_literal_matcher("Expected *"),
                    move |input: &'static str| term2.parse(input),
                )
                    .seq()
                    .map(|(left, _, right)| {
                        Expr::BinaryOp(Box::new(left), Op::Mul, Box::new(right.fold()))
                    })
                    .map_err(|_| "Expected multiplication expression");

                // Combine all expression types with alternatives
                // Try operations first, then fall back to simple terms
                let parser = (add, mul, move |input: &'static str| term.parse(input))
                    .alt()
                    .map(|x| x.map_2(|x| x.fold()))
                    .map(|x| x.fold())
                    .map_err(|_| "expr failed");

                Box::new(parser)
            });

        // Test parsing a simple term
        let result = expr_parser.parse("5");
        assert_eq!(result, Ok(("", Expr::Number(5.0))));

        // Test parsing a simple addition
        let result = expr_parser.parse("5+3");
        let expected = Expr::BinaryOp(
            Box::new(Expr::Number(5.0)),
            Op::Add,
            Box::new(Expr::Number(3.0)),
        );
        assert_eq!(result, Ok(("", expected)));

        // Test parsing a simple multiplication
        let result = expr_parser.parse("5*3");
        let expected = Expr::BinaryOp(
            Box::new(Expr::Number(5.0)),
            Op::Mul,
            Box::new(Expr::Number(3.0)),
        );
        assert_eq!(result, Ok(("", expected)));

        // Test parsing a chain of operations (should be left-associative)
        let result = expr_parser.parse("1+2+3");
        let expected = Expr::BinaryOp(
            Box::new(Expr::BinaryOp(
                Box::new(Expr::Number(1.0)),
                Op::Add,
                Box::new(Expr::Number(2.0)),
            )),
            Op::Add,
            Box::new(Expr::Number(3.0)),
        );
        assert_eq!(result, Ok(("", expected)));

        // Test parsing a mixed expression with precedence
        let result = expr_parser.parse("1+2*3");
        // Since we don't have precedence rules yet, this should be parsed left-to-right
        let expected = Expr::BinaryOp(
            Box::new(Expr::BinaryOp(
                Box::new(Expr::Number(1.0)),
                Op::Add,
                Box::new(Expr::Number(2.0)),
            )),
            Op::Mul,
            Box::new(Expr::Number(3.0)),
        );
        assert_eq!(result, Ok(("", expected)));
    }*/

    // Test custom input type with packrat parsing
    #[test]
    fn test_packrat_custom_input() {
        // Create a simple parser for a slice of bytes
        let byte_parser = |input: &'static [u8]| {
            if input.starts_with(&[b'a']) {
                Ok((&input[1..], b'a'))
            } else {
                Err((input, "Expected 'a'"))
            }
        };

        // Wrap it with packrat parsing
        let packrat_parser = byte_parser.packrat("byte");

        // Test parsing
        let result = packrat_parser.parse(b"abc");
        assert_eq!(result, Ok((b"bc" as &'static [u8], b'a')));

        // Test parsing invalid input
        let result = packrat_parser.parse(b"xyz");
        assert!(result.is_err());
    }
}
