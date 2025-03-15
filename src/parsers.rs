//! # Input Parser Implementations
//!
//! This module provides implementations of the `Parsable` trait for common input types
//! like strings and slices.

use crate::core::{Parsable, Parser};

/// Implementation of `Parsable` for string slices.
impl<'a, Error: Clone> Parsable<Error> for &'a str {
    type Item = char;

    fn make_literal_matcher(self, err: Error) -> impl Parser<Self, Self, Error>
    where
        Error: Clone,
    {
        move |input: &'a str| {
            if input.len() < self.len() {
                return Err((input, err.clone()));
            }
            if input.starts_with(self) {
                let (ret, rest) = input.split_at(self.len());
                Ok((rest, ret))
            } else {
                {
                    Err((input, err.clone()))
                }
            }
        }
    }

    fn make_anything_matcher(err: Error) -> impl Parser<Self, Self::Item, Error>
    where
        Error: Clone,
    {
        move |input: &'a str| {
            if input.is_empty() {
                return Err((input, err.clone()));
            }
            let (ret, rest) = input.split_at(1);
            Ok((rest, ret.chars().next().unwrap()))
        }
    }

    fn make_item_matcher(character: Self::Item, err: Error) -> impl Parser<Self, Self::Item, Error>
    where
        Error: Clone,
    {
        move |input: &'a str| {
            if input.is_empty() {
                return Err((input, err.clone()));
            }
            let (ret, rest) = input.split_at(1);
            let ret = ret.chars().next().unwrap();
            if ret == character {
                Ok((rest, ret))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    fn make_empty_matcher(err: Error) -> impl Parser<Self, (), Error>
    where
        Error: Clone,
    {
        move |input: &'a str| {
            if input.is_empty() {
                return Ok((input, ()));
            }
            Err((input, err.clone()))
        }
    }
}

/// Implementation of `Parsable` for slices.
impl<'a, Error: Clone, Input: Eq> Parsable<Error> for &'a [Input] {
    type Item = &'a Input;

    fn make_literal_matcher(self, err: Error) -> impl Parser<Self, Self, Error>
    where
        Error: Clone,
    {
        move |input: &'a [Input]| {
            if input.len() < self.len() {
                return Err((input, err.clone()));
            }
            for i in 0..self.len() {
                if input[i] != self[i] {
                    return Err((input, err.clone()));
                }
            }
            let (ret, rest) = input.split_at(self.len());
            Ok((rest, ret))
        }
    }

    fn make_anything_matcher(err: Error) -> impl Parser<Self, Self::Item, Error>
    where
        Error: Clone,
    {
        move |input: &'a [Input]| {
            if input.is_empty() {
                return Err((input, err.clone()));
            }
            let (ret, rest) = input.split_at(1);
            Ok((rest, &ret[0]))
        }
    }

    fn make_empty_matcher(err: Error) -> impl Parser<Self, (), Error>
    where
        Error: Clone,
    {
        move |input: &'a [Input]| {
            if input.is_empty() {
                return Ok((input, ()));
            }

            Err((input, err.clone()))
        }
    }

    fn make_item_matcher(character: Self::Item, err: Error) -> impl Parser<Self, Self::Item, Error>
    where
        Error: Clone,
    {
        move |input: &'a [Input]| {
            if input.is_empty() {
                return Err((input, err.clone()));
            }
            let (ret, rest) = input.split_at(1);
            if ret[0] == *character {
                Ok((rest, &ret[0]))
            } else {
                Err((input, err.clone()))
            }
        }
    }
}

use crate::state::{StateCarrier,  StatefulParser};
use std::fmt::{self, Display, Formatter};

/// Offset state that works for all parsable types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Offset(pub usize);

impl Offset {
    pub fn new(offset: usize) -> Self {
        Offset(offset)
    }

    pub fn increment(&mut self, n: usize) {
        self.0 += n;
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Position state for string slices with line and column information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub fn advance_column(&mut self, n: usize) {
        self.column += n;
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Indentation state for tracking indentation levels in string parsing
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Indentation {
    pub levels: Vec<usize>,
    pub current: usize,
}

impl Indentation {
    pub fn new() -> Self {
        Indentation {
            levels: Vec::new(),
            current: 0,
        }
    }

    pub fn push_level(&mut self, level: usize) {
        self.levels.push(level);
        self.current = level;
    }

    pub fn pop_level(&mut self) -> Option<usize> {
        let result = self.levels.pop();
        self.current = self.levels.last().copied().unwrap_or(0);
        result
    }

    pub fn current_level(&self) -> usize {
        self.current
    }

    pub fn depth(&self) -> usize {
        self.levels.len()
    }
}

/// Span information for tracking source positions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Span { start: 0, end: 0 }
    }
}

// Implement Parsable for StateCarrier<Offset, Input>
impl<'a, Error: Clone> Parsable<Error> for StateCarrier<Offset, &'a str> {
    type Item = char;

    #[allow(refining_impl_trait)]
    fn make_literal_matcher(self, err: Error) -> impl StatefulParser<Offset,&'a str, Self, Error> {
        move |input: StateCarrier<Offset, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.len() < self.input.len() {
                return Err((input, err.clone()));
            }

            if inner.starts_with(self.input) {
                let (ret, rest) = inner.split_at(self.input.len());
                state.increment(self.input.len());
                Ok((
                    StateCarrier { state, input: rest },
                    StateCarrier {
                        state: self.state,
                        input: ret,
                    },
                ))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_anything_matcher(err: Error) -> impl StatefulParser<Offset,&'a str, Self::Item, Error> {
        move |input: StateCarrier<Offset, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let c = ret.chars().next().unwrap();
            state.increment(1);
            Ok((StateCarrier { state, input: rest }, c))
        }
    }

    #[allow(refining_impl_trait)]
    fn make_item_matcher(
        character: Self::Item,
        err: Error,
    ) -> impl Parser<Self, Self::Item, Error> {
        move |input: StateCarrier<Offset, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let ret_char = ret.chars().next().unwrap();

            if ret_char == character {
                state.increment(1);
                Ok((StateCarrier { state, input: rest }, ret_char))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_empty_matcher(err: Error) -> impl StatefulParser<Offset,&'a str, (), Error> {
        move |input: StateCarrier<Offset, &'a str>| {
            if input.input.is_empty() {
                return Ok((input, ()));
            }

            Err((input, err.clone()))
        }
    }
}

// Implement Parsable for StateCarrier<Position, &str>
impl<'a, Error: Clone> Parsable<Error> for StateCarrier<Position, &'a str> {
    type Item = char;

    #[allow(refining_impl_trait)]
    fn make_literal_matcher(self, err: Error) -> impl StatefulParser<Position, &'a str, Self, Error> {
        move |input: StateCarrier<Position, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.len() < self.input.len() {
                return Err((input, err.clone()));
            }

            if inner.starts_with(self.input) {
                let (ret, rest) = inner.split_at(self.input.len());

                // Update position based on newlines in the matched text
                for c in self.input.chars() {
                    if c == '\n' {
                        state.advance_line();
                    } else {
                        state.advance_column(1);
                    }
                }

                Ok((
                    StateCarrier { state, input: rest },
                    StateCarrier {
                        state: self.state,
                        input: ret,
                    },
                ))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_anything_matcher(err: Error) -> impl StatefulParser<Position, &'a str, Self::Item, Error> {
        move |input: StateCarrier<Position, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let c = ret.chars().next().unwrap();

            if c == '\n' {
                state.advance_line();
            } else {
                state.advance_column(1);
            }

            Ok((StateCarrier { state, input: rest }, c))
        }
    }

    #[allow(refining_impl_trait)]
    fn make_item_matcher(
        character: Self::Item,
        err: Error,
    ) -> impl StatefulParser<Position, &'a str, Self::Item, Error> {
        move |input: StateCarrier<Position, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let ret_char = ret.chars().next().unwrap();

            if ret_char == character {
                if ret_char == '\n' {
                    state.advance_line();
                } else {
                    state.advance_column(1);
                }

                Ok((StateCarrier { state, input: rest }, ret_char))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_empty_matcher(err: Error) -> impl StatefulParser<Position,&'a str, (), Error> {
        move |input: StateCarrier<Position, &'a str>| {
            if input.input.is_empty() {
                return Ok((input, ()));
            }

            Err((input, err.clone()))
        }
    }
}

// Implement Parsable for StateCarrier<Indentation, &str>
impl<'a, Error: Clone> Parsable<Error> for StateCarrier<Indentation, &'a str> {
    type Item = char;

    #[allow(refining_impl_trait)]
    fn make_literal_matcher(self, err: Error) -> impl StatefulParser<Indentation, &'a str, Self, Error> {
        move |input: StateCarrier<Indentation, &'a str>| {
            let StateCarrier {
                state,
                input: inner,
            } = input.clone();

            if inner.len() < self.input.len() {
                return Err((input, err.clone()));
            }

            if inner.starts_with(self.input) {
                let (ret, rest) = inner.split_at(self.input.len());
                Ok((
                    StateCarrier { state, input: rest },
                    StateCarrier {
                        state: self.state.clone(),
                        input: ret,
                    },
                ))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_anything_matcher(err: Error) -> impl StatefulParser<Indentation, &'a str, Self::Item, Error> {
        move |input: StateCarrier<Indentation, &'a str>| {
            let StateCarrier {
                state,
                input: inner,
            } = input.clone();

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let c = ret.chars().next().unwrap();

            Ok((StateCarrier { state, input: rest }, c))
        }
    }

    #[allow(refining_impl_trait)]
    fn make_item_matcher(
        character: Self::Item,
        err: Error,
    ) -> impl StatefulParser<Indentation, &'a str, Self::Item, Error> {
        move |input: StateCarrier<Indentation, &'a str>| {
            let StateCarrier {
                state,
                input: inner,
            } = input.clone();

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let ret_char = ret.chars().next().unwrap();

            if ret_char == character {
                Ok((StateCarrier { state, input: rest }, ret_char))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_empty_matcher(err: Error) -> impl StatefulParser<Indentation, &'a str, (), Error> {
        move |input: StateCarrier<Indentation, &'a str>| {
            if input.input.is_empty() {
                return Ok((input, ()));
            }

            Err((input, err.clone()))
        }
    }
}

// Implement Parsable for StateCarrier<Span, &str>
impl<'a, Error: Clone> Parsable<Error> for StateCarrier<Span, &'a str> {
    type Item = char;

    #[allow(refining_impl_trait)]
    fn make_literal_matcher(self, err: Error) -> impl StatefulParser<Span, &'a str, Self, Error> {
        move |input: StateCarrier<Span, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.len() < self.input.len() {
                return Err((input, err.clone()));
            }

            if inner.starts_with(self.input) {
                let (ret, rest) = inner.split_at(self.input.len());

                // Update span
                let old_end = state.end;
                state.end += self.input.len();

                // Create result span
                let result_span = Span::new(old_end, state.end);

                Ok((
                    StateCarrier { state, input: rest },
                    StateCarrier {
                        state: result_span,
                        input: ret,
                    },
                ))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_anything_matcher(err: Error) -> impl StatefulParser<Span, &'a str, Self::Item, Error> {
        move |input: StateCarrier<Span, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let c = ret.chars().next().unwrap();

            // Update span
            state.end += 1;

            Ok((StateCarrier { state, input: rest }, c))
        }
    }

    #[allow(refining_impl_trait)]
    fn make_item_matcher(
        character: Self::Item,
        err: Error,
    ) -> impl StatefulParser<Span, &'a str, Self::Item, Error> {
        move |input: StateCarrier<Span, &'a str>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            let ret_char = ret.chars().next().unwrap();

            if ret_char == character {
                // Update span
                state.end += 1;

                Ok((StateCarrier { state, input: rest }, ret_char))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_empty_matcher(err: Error) -> impl StatefulParser<Span, &'a str, (), Error> {
        move |input: StateCarrier<Span, &'a str>| {
            if input.input.is_empty() {
                return Ok((input, ()));
            }

            Err((input, err.clone()))
        }
    }
}

// Implementation for StateCarrier<Offset, &[T]>
impl<'a, Error: Clone, T: Eq> Parsable<Error> for StateCarrier<Offset, &'a [T]> {
    type Item = &'a T;

    #[allow(refining_impl_trait)]
    fn make_literal_matcher(self, err: Error) -> impl StatefulParser<Offset, &'a [T], Self, Error> {
        move |input: StateCarrier<Offset, &'a [T]>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.len() < self.input.len() {
                return Err((input, err.clone()));
            }

            for i in 0..self.input.len() {
                if inner[i] != self.input[i] {
                    return Err((input, err.clone()));
                }
            }

            let (ret, rest) = inner.split_at(self.input.len());
            state.increment(self.input.len());

            Ok((
                StateCarrier { state, input: rest },
                StateCarrier {
                    state: self.state,
                    input: ret,
                },
            ))
        }
    }

    #[allow(refining_impl_trait)]
    fn make_anything_matcher(err: Error) -> impl StatefulParser<Offset, &'a [T], Self::Item, Error> {
        move |input: StateCarrier<Offset, &'a [T]>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);
            state.increment(1);

            Ok((StateCarrier { state, input: rest }, &ret[0]))
        }
    }

    #[allow(refining_impl_trait)]
    fn make_item_matcher(
        character: Self::Item,
        err: Error,
    ) -> impl StatefulParser<Offset, &'a [T], Self::Item, Error> {
        move |input: StateCarrier<Offset, &'a [T]>| {
            let StateCarrier {
                mut state,
                input: inner,
            } = input;

            if inner.is_empty() {
                return Err((input, err.clone()));
            }

            let (ret, rest) = inner.split_at(1);

            if ret[0] == *character {
                state.increment(1);
                Ok((StateCarrier { state, input: rest }, &ret[0]))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn make_empty_matcher(err: Error) -> impl StatefulParser<Offset, &'a [T], (), Error> {
        move |input: StateCarrier<Offset, &'a [T]>| {
            if input.input.is_empty() {
                return Ok((input, ()));
            }

            Err((input, err.clone()))
        }
    }
}
