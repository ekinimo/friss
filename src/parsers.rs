//! # Input Parser Implementations
//!
//! This module provides implementations of the `Parsable` trait for common input types
//! like strings and slices.

use crate::core::{Parser, Parsable};

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
