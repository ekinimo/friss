//! # State Machine Integration
//!
//! This module provides integration between parsers and finite state machines (FSMs)
//! for tracking offsets, managing parse context, and improving error reporting.
//!
//! ## Key Features
//!
//! - Track byte offsets during parsing with generic state machines
//! - Compute line and column positions when needed for text-based inputs
//! - Maintain state context for context-sensitive grammars
//! - Enhance error messages with detailed position information
//! - Support for custom state transitions based on parser actions
//!
//! ## Example Usage
//!
//! ```rust
//! use friss::*;
//! use friss::state::*;
//!
//! // Basic offset tracking
//! let parser = "hello".make_literal_matcher("Expected hello")
//!     .with_byte_offset();
//!
//! // Parse with offset information
//! match parser.parse("world") {
//!     Ok((rest, (result, offset))) => println!("Success at offset {}", offset.offset),
//!     Err((rest, (err, offset))) => println!("Error at offset {}: {}", offset.offset, err),
//! }
//!
//! // For line/column information in errors:
//! let source = "hello\nworld";
//! let parser = "goodbye".make_literal_matcher("Expected goodbye")
//!     .with_positioned_errors(source);
//! ```

use std::marker::PhantomData;
use std::fmt::{self, Display, Formatter, Debug};
use crate::core::{Parser, Parsable, ParserOutput};

/// Represents a generic offset in the input source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offset {
    /// Byte offset from start (0-based)
    pub offset: usize,
}

impl Default for Offset {
    fn default() -> Self {
        Offset {
            offset: 0,
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "offset {}", self.offset)
    }
}

/// Trait for state machines that can be integrated with parsers.
pub trait StateMachine<Input: Clone, State: Clone> {
    /// Update the state based on consumed input and remaining input.
    fn update(&self, state: &mut State, consumed: &Input, remaining: &Input);
    
    /// Create a new initial state.
    fn initial_state(&self) -> State;
}

/// Generic offset tracker for string inputs.
pub struct OffsetTracker<'a> {
    /// Function to measure the size of consumed input
    measure_fn: Box<dyn Fn(&&'a str) -> usize + 'a>,
}

impl<'a> OffsetTracker<'a> {
    /// Creates a new offset tracker with a custom measurement function.
    pub fn new<F>(measure_fn: F) -> Self
    where
        F: Fn(&&'a str) -> usize + 'a,
    {
        OffsetTracker {
            measure_fn: Box::new(measure_fn),
        }
    }
    
    /// Creates a byte-based offset tracker
    pub fn bytes() -> Self {
        Self::new(|s| s.len())
    }
    
    /// Creates a character-based offset tracker
    pub fn chars() -> Self {
        Self::new(|s| s.chars().count())
    }
}

impl<'a> StateMachine<&'a str, Offset> for OffsetTracker<'a> {
    fn update(&self, state: &mut Offset, initial: &&'a str, rest: &&'a str) {
        // Calculate the consumed portion by finding the difference between
        // the initial input length and the remaining input length
        if initial.len() >= rest.len(){
        let consumed_len = initial.len() - rest.len();
        if consumed_len > 0 {
            // Create a slice representing only the consumed portion
            let consumed = &initial[..consumed_len];
            state.offset += (self.measure_fn)(&consumed);
        }
        }
    }

    fn initial_state(&self) -> Offset {
        Offset::default()
    }
}


/// Specialized position tracker that computes line/column from text
pub struct TextPositionTracker;

impl TextPositionTracker {
    /// Creates a new text position tracker.
    pub fn new() -> Self {
        TextPositionTracker
    }
    
    /// Compute line and column from offset and input text
    pub fn compute_line_column(text: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut last_line_start = 0;
        
        for (idx, c) in text.char_indices() {
            if idx >= offset {
                break;
            }
            
            if c == '\n' {
                line += 1;
                last_line_start = idx + 1;
            }
        }
        
        let column = offset - last_line_start + 1;
        (line, column)
    }
}

/// A parser that tracks state using a state machine.
pub struct StatefulParser<P, Input, Output, Error, State, SM> 
where
    P: Parser<Input, Output, Error>,
    Input: Parsable<Error> + Clone,
    Error: Clone,
    State: Clone,
    SM: StateMachine<Input, State>,
{
    parser: P,
    state_machine: SM,
    _phantom: PhantomData<(Input, Output, Error, State)>,
}

/// Extension trait to add state machine functionality to parsers.
/// This more specialized version is only for string slice parsers.
pub trait StringParserStateMachineExt<'a, Output, Error>: Parser<&'a str, Output, Error>
where
    Error: Clone,
    Self: Sized,
{
    /// Add a state machine to the parser to track state during parsing.
    fn with_state_machine<State, SM>(self, state_machine: SM) 
    -> StatefulParser<Self, &'a str, Output, Error, State, SM>
    where
        State: Clone,
        SM: StateMachine<&'a str, State>,
    {
        StatefulParser {
            parser: self,
            state_machine,
            _phantom: PhantomData,
        }
    }
    
    /// Add byte offset tracking for string inputs.
    fn with_byte_offset(self) -> StatefulParser<Self, &'a str, Output, Error, Offset, OffsetTracker<'a>> {
        self.with_state_machine(OffsetTracker::bytes())
    }
    
    /// Add character offset tracking for string inputs.
    fn with_char_offset(self) -> StatefulParser<Self, &'a str, Output, Error, Offset, OffsetTracker<'a>> {
        self.with_state_machine(OffsetTracker::chars())
    }
}

// Implement the extension trait for all parser types that work with string slices
impl<'a, P, Output, Error> StringParserStateMachineExt<'a, Output, Error> for P
where
    P: Parser<&'a str, Output, Error>,
    Error: Clone,
{
}

/// Implementation of Parser for StatefulParser
impl<P, Input, Output, Error, State, SM> Parser<Input, (Output, State), (Error, State)> 
    for StatefulParser<P, Input, Output, Error, State, SM>
where
    P: Parser<Input, Output, Error>,
    Input: Parsable<Error> + Parsable<(Error,State)> + Clone,
    Error: Clone,
    State: Clone,
    SM: StateMachine<Input, State>,
    (Output, State): ParserOutput,
{
    fn parse(&self, input: Input) -> Result<(Input, (Output, State)), (Input, (Error, State))> {
        let mut state = self.state_machine.initial_state();
        let initial_input = input.clone();
        
        match self.parser.parse(input) {
            Ok((rest, output)) => {
                // Update state based on consumed input
                self.state_machine.update(&mut state, &initial_input, &rest);
                
                Ok((rest, (output, state)))
            }
            Err((rest, error)) => {
                // For errors, we also want to track the position
                self.state_machine.update(&mut state, &initial_input, &rest);
                
                Err((rest, (error, state)))
            }
        }
    }
}

/// Error type that includes offset information.
#[derive(Clone, Debug)]
pub struct OffsetError<E> {
    /// The original error
    pub error: E,
    /// Offset where the error occurred
    pub offset: Offset,
}

impl<E: Display> Display for OffsetError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.error, self.offset)
    }
}

/// Trait for converting errors to offset errors.
pub trait ToOffsetError<E> {
    /// Convert to an offset error.
    fn with_offset(self, offset: Offset) -> OffsetError<E>;
}

impl<E> ToOffsetError<E> for E {
    fn with_offset(self, offset: Offset) -> OffsetError<E> {
        OffsetError {
            error: self,
            offset,
        }
    }
}

/// A richer error type that includes line and column derived from offset.
#[derive(Clone, Debug)]
pub struct PositionedError<E> {
    /// The original error
    pub error: E,
    /// Offset information
    pub offset: Offset,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
}

impl<E: Display> Display for PositionedError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}, column {} (offset {})", 
               self.error, self.line, self.column, self.offset.offset)
    }
}

/// Trait for converting offset errors to positioned errors.
pub trait OffsetErrorToPositioned<E> {
    /// Convert to a positioned error using the provided source text.
    fn with_line_column(self, source: &str) -> PositionedError<E>;
}

impl<E> OffsetErrorToPositioned<E> for OffsetError<E> {
    fn with_line_column(self, source: &str) -> PositionedError<E> {
        let (line, column) = TextPositionTracker::compute_line_column(source, self.offset.offset);
        PositionedError {
            error: self.error,
            offset: self.offset,
            line,
            column,
        }
    }
}

/// Extension trait for parsers that adds error enhancement for string parsers
pub trait StringParserErrorEnhancementExt<'a, Output, Error>: Parser<&'a str, Output, Error>
where
    Error: Clone + Debug,
    Self: Sized + 'a,
{
    /// Enhance error messages with a custom function
    fn with_enhanced_errors<F>(self, f: F) -> impl Parser<&'a str, Output, String>
    where
        F: Fn(&Error) -> String + 'static,
    {
        move |input: &'a str| match self.parse(input) {
            Ok(ok) => Ok(ok),
            Err((rest, error)) => Err((rest, f(&error))),
        }
    }
    
    /// Enhance error messages with offset information
    fn with_offset_errors(self) -> impl Parser<&'a str, Output, OffsetError<Error>> + 'a
    where
        &'a str: Parsable<OffsetError<Error>>,
        Output : 'a,
        Error : 'a
    {
        // First add byte offset tracking
        let offset_parser = self.with_byte_offset();
        
        move |input: &'a str| match offset_parser.parse(input) {
            Ok((rest, (output, _offset))) => Ok((rest, output)),
            Err((rest, (error, offset))) => {
                Err((rest, error.with_offset(offset)))
            }
        }
    }


    
    /// Enhance error messages with position information (line/column)
    /// Note: This requires storing the original input text to compute line/column
    fn with_positioned_errors(self, source_text: &'a str) -> impl Parser<&'a str, Output, PositionedError<Error>> + 'a
    where
        &'a str: Parsable<OffsetError<Error>> + Parsable<PositionedError<Error>>,
    Output : 'a,
    Error : 'a
    {
        // First add offset tracking and then convert to positioned errors
        let offset_parser = self.with_byte_offset();
        let source = source_text.to_owned();
        
        move |input: &'a str| match offset_parser.parse(input) {
            Ok((rest, (output, _offset))) => Ok((rest, output)),
            Err((rest, (error, offset))) => {
                let offset_error = error.with_offset(offset);
                Err((rest, offset_error.with_line_column(&source)))
            }
        }
    }
}

// Implement the error enhancement extension trait for all string parsers
impl<'a, P, Output, Error> StringParserErrorEnhancementExt<'a, Output, Error> for P
where
    P: Parser<&'a str, Output, Error> + 'a,
    Error: Clone + Debug,
{
}


/// A set of common state machines
pub mod common {
    use super::*;
    
    
    /// Generic token counter for lexical analysis
    pub struct TokenCounter {
        token_types: Vec<Box<dyn Fn(&str) -> bool>>,
    }
    
    impl TokenCounter {
        pub fn new() -> Self {
            TokenCounter {
                token_types: Vec::new(),
            }
        }
        
        pub fn add_token_type<F>(&mut self, predicate: F)
        where
            F: Fn(&str) -> bool + 'static,
        {
            self.token_types.push(Box::new(predicate));
        }
        
        pub fn count_tokens(&self, input: &str) -> Vec<usize> {
            let mut counts = vec![0; self.token_types.len()];
            
            // Simple tokenization logic - can be replaced with more sophisticated approach
            for word in input.split_whitespace() {
                for (idx, predicate) in self.token_types.iter().enumerate() {
                    if predicate(word) {
                        counts[idx] += 1;
                    }
                }
            }
            
            counts
        }
    }
    
    /// A context-sensitive state machine that can track parsing context
    pub struct ContextTracker<Context: Clone> {
        initial_context: Context,
        context_transitions: Vec<Box<dyn Fn(&Context, &str) -> Option<Context>>>,
    }

    impl<Context: Clone> ContextTracker<Context> {
        /// Create a new context tracker with an initial context
        pub fn new(initial_context: Context) -> Self {
            ContextTracker {
                initial_context,
                context_transitions: Vec::new(),
            }
        }
        
        /// Add a transition rule to the context tracker
        pub fn add_transition<F>(&mut self, transition: F)
        where
            F: Fn(&Context, &str) -> Option<Context> + 'static,
        {
            self.context_transitions.push(Box::new(transition));
        }
    }

    impl<'a, Context: Clone> StateMachine<&'a str, Context> for ContextTracker<Context> {
        fn update(&self, state: &mut Context, consumed: &&'a str, _remaining: &&'a str) {
            for transition in &self.context_transitions {
                if let Some(new_context) = transition(state, consumed) {
                    *state = new_context;
                    break;
                }
            }
        }
        
        fn initial_state(&self) -> Context {
            self.initial_context.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::*;
    
    #[test]
    fn test_offset_tracking() {
        let input = "hello\nworld";
        let parser = "hello".make_literal_matcher("Expected hello")
            .with_byte_offset();
            
        let result = parser.parse(input);
        if let Ok((rest, (output, offset))) = result {
            assert_eq!(output, "hello");
            assert_eq!(offset.offset, 5);  // "hello" is 5 bytes
            assert_eq!(rest, "\nworld");
        } else {
            panic!("Parser failed");
        }
    }
    
    #[test]
    fn test_error_with_offset() {
        let input = "hello\nworld";
        let parser = "goodbye".make_literal_matcher("Expected goodbye")
            .with_byte_offset();
            
        let result = parser.parse(input);
        if let Err((rest, (_error, offset))) = result {
            assert_eq!(offset.offset, 0);  // Error at the beginning
            assert_eq!(rest, "hello\nworld");
        } else {
            panic!("Parser unexpectedly succeeded");
        }
    }
    
    #[test]
    fn test_text_position_computation() {
        let text = "hello\nworld\ngoodbye";
        
        // Test different offsets
        let (line, col) = TextPositionTracker::compute_line_column(text, 0);
        assert_eq!(line, 1);
        assert_eq!(col, 1);
        
        let (line, col) = TextPositionTracker::compute_line_column(text, 5);
        assert_eq!(line, 1);
        assert_eq!(col, 6);
        
        let (line, col) = TextPositionTracker::compute_line_column(text, 6);
        assert_eq!(line, 2);
        assert_eq!(col, 1);
        
        let (line, col) = TextPositionTracker::compute_line_column(text, 12);
        assert_eq!(line, 3);
        assert_eq!(col, 1);
        
        let (line, col) = TextPositionTracker::compute_line_column(text, 15);
        assert_eq!(line, 3);
        assert_eq!(col, 4);
    }
    
    #[test]
    fn test_enhanced_error_messages() {
        let input = "hello\nworld";
        let parser = "goodbye".make_literal_matcher("Expected goodbye")
            .with_enhanced_errors(|err| format!("Enhanced error: {}", err));
        
        let result = parser.parse(input);
        if let Err((rest, error)) = result {
            assert_eq!(error, "Enhanced error: Expected goodbye");
            assert_eq!(rest, "hello\nworld");
        } else {
            panic!("Parser unexpectedly succeeded");
        }
    }
}
