//! # Friss
//!
//! Friss is a parser combinator library for Rust.
//!
//! Parser combinators allow you to build complex parsers from simple components.
//! Combinators in this library act in such a way that it creates a Syntax Tree and an Error Tree simultaneously.
//!
//! The core trait is the `Parser` trait, which provides numerous useful methods for composing parsers.
//!
//! ## Basic Combinators
//!
//! The library offers several essential combinators:
//!
//! ### Sequence (`seq`)
//!
//! The `seq` combinator runs two parsers in sequence, returning both results as a tuple.
//! In case of failure it will return an Either returning the appropriate error :
//!
//! ```rust
//! use friss::*;
//!
//! let parser = "hello ".make_literal_matcher("Expected hello")
//!     .seq("world".make_literal_matcher("Expected world"));
//!
//! assert_eq!(parser.parse("hello world"), Ok(("", ("hello ", "world"))));
//! assert_eq!(parser.parse("hello space"), Err(("space", Either::Right("Expected world"))));

//! ```
//!
//! ### Alternative (`alt`)
//!
//! The `alt` combinator tries one parser, and if it fails, tries another
//! In case of failure it will return a tuple containing both errors.:
//!
//! ```rust
//! use friss::*;
//!
//! let parser = "yes".make_literal_matcher("Expected yes")
//!     .alt("no".make_literal_matcher("Expected no"));
//!
//! assert_eq!(parser.parse("yes"), Ok(("", Either::Left("yes"))));
//! assert_eq!(parser.parse("no"), Ok(("", Either::Right("no"))));
//! assert_eq!(parser.parse("maybe"), Err(("maybe", ("Expected yes","Expected no"))));
//! ```
//!
//! ### Maybe (`maybe`)
//!
//! The `maybe` combinator makes a parser optional, always succeeding with `None` if the parser fails:
//!
//! ```rust
//! use friss::*;
//!
//! let parser = "optional".make_literal_matcher("Expected optional").maybe();
//!
//! assert_eq!(parser.parse("optional"), Ok(("", Some("optional"))));
//! assert_eq!(parser.parse("something"), Ok(("something", None)));
//! ```
//!
//! ### Many (`many`)
//!
//! The `many` combinator applies a parser zero or more times, collecting all results:
//!
//! ```rust
//! use friss::*;
//!
//! let parser = "a".make_literal_matcher("Expected a").many();
//!
//! assert_eq!(parser.parse(""), Ok(("", vec![])));
//! assert_eq!(parser.parse("a"), Ok(("", vec!["a"])));
//! assert_eq!(parser.parse("aaa"), Ok(("", vec!["a", "a", "a"])));
//! ```
//!
//! ### Mapping (`map` and `map_err`)
//!
//! The `map` and `map_err` combinators transform parser outputs and errors:
//!
//! ```rust
//! use friss::*;
//!
//! let parser = "123".make_literal_matcher("Expected 123")
//!     .map(|s| s.parse::<i32>().unwrap());
//!
//! assert_eq!(parser.parse("123"), Ok(("", 123)));
//! ```
//!
//! ## Recursive Parsing
//!
//! Friss supports recursive parsers using the `recursive` function, which allows a parser to refer to itself:
//!
//! ```rust
//! use friss::*;
//! use friss::core::recursive;
//!
//! // Parser for handling nested parentheses
//! // Grammar: P -> (P) | empty
//! let paren_parser: Box<dyn Parser<&str, i32, &str>> = recursive(move |parser| {
//!     let nested = Box::new(
//!         '('.make_character_matcher("Expected opening paren")
//!             .seq(move |x| parser.parse(x))
//!             .map_err(|x| x.fold())
//!             .seq(')'.make_character_matcher("Expected closing paren"))
//!             .map_err(|x| x.fold())
//!             .map(|((_, inner), _)| inner + 1),
//!     );
//!
//!     let empty = "".make_literal_matcher("").map(|_| 0);
//!
//!     Box::new(
//!         nested
//!             .alt(empty)
//!             .map_err(|(a, b)| a)
//!             .map(|either| match either {
//!                 Either::Left(depth) => depth,
//!                 Either::Right(empty_result) => empty_result,
//!             }),
//!     )
//! });
//!
//! assert_eq!(paren_parser.parse(""), Ok(("", 0)));
//! assert_eq!(paren_parser.parse("()"), Ok(("", 1)));
//! assert_eq!(paren_parser.parse("(())"), Ok(("", 2)));
//! assert_eq!(paren_parser.parse("((()))"), Ok(("", 3)));
//! assert_eq!(paren_parser.parse("()extra"), Ok(("extra", 1)));
//! ```
//!
//! ## Parser Sugar
//!
//! The library provides syntactic sugar through the `ParserSugar` trait to make parser composition more ergonomic:
//!
//! ```rust
//! use friss::*;
//! use friss::sugar::ParserSugar;
//!
//! // Using tuple syntax for sequence
//! let seq_parser = ("hello".make_literal_matcher("Expected hello"),
//!                  "world".make_literal_matcher("Expected world")).seq();
//!
//! // Using tuple syntax for alternatives
//! let alt_parser = ("yes".make_literal_matcher("Expected yes"),
//!                  "no".make_literal_matcher("Expected no")).alt();
//!
//! // Creates a parser that tries all parsers in the tuple
//! let multi_alt_parser = (
//!     "one".make_literal_matcher("Expected one"),
//!     "two".make_literal_matcher("Expected two"),
//!     "three".make_literal_matcher("Expected three")
//! ).alt();
//!
//! assert_eq!(seq_parser.parse("helloworld"), Ok(("", ("hello", "world"))));
//! assert_eq!(alt_parser.parse("yes"), Ok(("", Either::Left("yes"))));
//! assert_eq!(multi_alt_parser.parse("two"), Ok(("", Either3::Middle("two"))));
//! ```
//!

// Re-export all public items
pub use crate::core::{fail, pure, recursive, Parsable, ParsableItem, Parser};
pub use crate::sugar::*;
pub use crate::types::*;

// Module declarations
pub mod combinators;
pub mod core;
pub mod parsers;
pub mod sugar;
pub mod types;
pub mod state;
pub mod memo;
// Include examples in tests
#[cfg(test)]
pub mod tests;
