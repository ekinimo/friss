//! # Core Parser Traits
//!
//! This module defines the fundamental traits for the parser combinator library.
//!
//! The primary traits are:
//! - `Parser`: Core trait for all parsers
//! - `Parsable`: Trait for types that can be parsed
//! - `ParsableItem`: Trait for individual items within parsable types
//!
//! ## Example Usage
//!
//! ```rust
//! use friss::*;
//! use friss::core::*;
//!
//! // Create a parser for a digit
//! let digit_parser = '1'.make_character_matcher("Not a digit 1");
//!
//! // Parse the input
//! assert_eq!(digit_parser.parse("123"), Ok(("23", '1')));
//! assert_eq!(digit_parser.parse("abc"), Err(("abc", "Not a digit 1")));
//!
//! // Combine parsers
//! let combined = digit_parser
//!     .seq('2'.make_character_matcher("Not a digit 2"))
//!     .map(|(a, b)| format!("{}{}", a, b));
//!
//! assert_eq!(combined.parse("123"), Ok(("3", "12".to_string())));
//! ```
use std::cell::RefCell;
use crate::{state::{StateCarrier, StatefulParser, TransitionParser}, types::*};

/// Trait for items within a `Parsable` type.
///
/// This trait allows for the creation of a parser that matches a specific character
/// or item within a parsable input.
///
/// ## Example
///
/// ```rust
/// use friss::*;
///
/// // Create a parser that matches the character 'a'
/// let a_parser = 'a'.make_character_matcher("Expected 'a'");
///
/// assert_eq!(a_parser.parse("abc"), Ok(("bc", 'a')));
/// assert_eq!(a_parser.parse("xyz"), Err(("xyz", "Expected 'a'")));
/// ```
pub trait ParsableItem<Parent: Parsable<Error>, Error: Clone>: Sized {
    /// Creates a parser that matches the current item.
    fn make_character_matcher(self, err: Error) -> impl Parser<Parent, Self, Error>;
}

/// Trait for types that can be parsed.
///
/// This trait defines methods for creating basic parsers for a type.
///
/// ## Example
///
/// ```rust
/// use friss::*;
///
/// // Create a parser that matches the literal "hello"
/// let hello_parser = "hello".make_literal_matcher("Expected 'hello'");
///
/// assert_eq!(hello_parser.parse("hello world"), Ok((" world", "hello")));
/// assert_eq!(hello_parser.parse("hi"), Err(("hi", "Expected 'hello'")));
///
/// // Create a parser that matches any character
/// let any_char = <&str as Parsable<&str>>::make_anything_matcher("Expected any character");
///
/// assert_eq!(any_char.parse("abc"), Ok(("bc", 'a')));
/// assert_eq!(any_char.parse(""), Err(("", "Expected any character")));
/// ```
pub trait Parsable<Error: Clone>: Sized {
    /// The type of individual items within this parsable type.
    type Item;

    /// Creates a parser that matches the entire instance, consuming it from the input.
    fn make_literal_matcher(self, err: Error) -> impl Parser<Self, Self, Error>;

    /// Creates a parser that matches any single item from the input.
    fn make_anything_matcher(err: Error) -> impl Parser<Self, Self::Item, Error>;

    /// Creates a parser that matches a specific item from the input.
    fn make_item_matcher(character: Self::Item, err: Error)
        -> impl Parser<Self, Self::Item, Error>;

    /// Creates a parser that matches empty input.
    fn make_empty_matcher(err: Error) -> impl Parser<Self, (), Error>;
}

/// Marker trait for parser outputs.
pub trait ParserOutput {}
impl<T> ParserOutput for T {}

/// Core trait for all parsers in the library.
///
/// A parser takes an input, attempts to parse it, and returns either:
/// - A successful result with the remaining input and the parsed output
/// - An error with the remaining input and an error value
///
/// ## Example
///
/// ```rust
/// use friss::*;
///
/// // Create a parser that matches the literal "hello"
/// let hello_parser = "hello".make_literal_matcher("Expected 'hello'");
///
/// // Use the parser to parse input
/// let result = hello_parser.parse("hello world");
/// assert_eq!(result, Ok((" world", "hello")));
///
/// // Handle parsing failure
/// let error_result = hello_parser.parse("goodbye");
/// assert_eq!(error_result, Err(("goodbye", "Expected 'hello'")));
/// ```
pub trait Parser<Input: Parsable<Error>, Output: ParserOutput, Error: Clone> {
    /// Attempts to parse the input, returning either a success with the remainder and output,
    /// or an error with the remainder and error value.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "hello".make_literal_matcher("Expected hello");
    /// let result = parser.parse("hello world");
    /// assert_eq!(result, Ok((" world", "hello")));
    /// ```
    fn parse(&self, input: Input) -> Result<(Input, Output), (Input, Error)>;



    fn with_state_transition<State,SuccessF,FailF>(self,succes:SuccessF,fail:FailF)-> impl StatefulParser<State,Input,Output,Error>
        where
        Input : Clone,
        Self : Sized,
    for<'a> SuccessF: FnMut(State, Input, Output, Input) -> (State, Input, Output) + 'a,
    for<'a> FailF: FnMut(State, Input, Error, Input) -> (State, Input, Error) + 'a,
    StateCarrier<State, Input>: Parsable<Error>,
    {

        TransitionParser::new_with_success_and_fail(self,succes,fail)
     
    }


    fn transition_on_success<State,SuccessF,FailF>(self,success:SuccessF)-> impl StatefulParser<State,Input,Output,Error>
    where
        Input : Clone,
    Self : Sized,
    for<'a> SuccessF: FnMut(State, Input, Output, Input) -> (State, Input, Output) + 'a,
    StateCarrier<State, Input>: Parsable<Error>,
    {

        TransitionParser::new_with_success_and_fail(self,success,|state,rest,error,_orig| (state,rest,error))
    }

    fn transition_on_error<State,SuccessF,FailF>(self,fail:FailF)-> impl StatefulParser<State,Input,Output,Error>
    where
        Input : Clone,
    Self : Sized,
    for<'a> FailF: FnMut(State, Input, Error, Input) -> (State, Input, Error) + 'a,
    StateCarrier<State, Input>: Parsable<Error>,
    {

        TransitionParser::new_with_success_and_fail(self,|state,rest,error,_orig| (state,rest,error),fail)
    }


    /// Validates the output of the parser with a predicate.
    ///
    /// Returns an error if the predicate returns false.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// // Parse a digit and validate it's even
    /// let digit = <&str as Parsable<&str>>::make_anything_matcher("Expected digit")
    ///     .validate(|c| c.is_digit(10) && c.to_digit(10).unwrap() % 2 == 0, "Expected even digit");
    ///
    /// assert_eq!(digit.parse("2"), Ok(("", '2')));
    /// assert_eq!(digit.parse("3"), Err(("3", "Expected even digit")));
    /// ```
    fn validate<Pred>(self, predicate: Pred, err: Error) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        Input: Clone,
        Error: Clone,
        Pred: Fn(&Output) -> bool,
    {
        move |input: Input| {
            let ipt = input.clone();
            let (rest, result) = self.parse(ipt)?;
            if predicate(&result) {
                Ok((rest, result))
            } else {
                Err((input, err.clone()))
            }
        }
    }

    /// Maps the output of the parser with a function.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "123".make_literal_matcher("Expected 123")
    ///     .map(|s| s.parse::<i32>().unwrap());
    ///
    /// assert_eq!(parser.parse("123"), Ok(("", 123)));
    /// ```
    fn map<Out2, Fun>(self, f: Fun) -> impl Parser<Input, Out2, Error>
    where
        Fun: Fn(Output) -> Out2,
        Self: Sized,
    {
        move |input: Input| match self.parse(input) {
            Ok((rest, ret)) => Ok((rest, f(ret))),
            Err((rest, ret)) => Err((rest, ret)),
        }
    }

    /// Maps the error of the parser with a function.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "hello".make_literal_matcher("simple error")
    ///     .map_err(|_| "detailed error message");
    ///
    /// assert_eq!(parser.parse("world"), Err(("world", "detailed error message")));
    /// ```
    fn map_err<Err2, Fun>(self, f: Fun) -> impl Parser<Input, Output, Err2>
    where
        Fun: Fn(Error) -> Err2,
        Input: Parsable<Err2>,
        Err2: Clone,
        Self: Sized,
    {
        move |input: Input| match self.parse(input) {
            Ok((rest, ret)) => Ok((rest, ret)),
            Err((rest, ret)) => Err((rest, f(ret))),
        }
    }

    /// Binds the output of this parser to another parser.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let digit = <&str as Parsable<&str>>::make_anything_matcher("Expected digit")
    ///     .validate(|c| c.is_digit(10), "Not a digit");
    ///
    /// // Parse a digit, then parse that many 'a' characters
    /// let count_parser = digit.bind_output(|c| {
    ///     let count = c.to_digit(10).unwrap() as usize;
    ///     "a".make_literal_matcher("Expected 'a'")
    ///         .exactly_n::<3>("Need exactly 3 'a's")
    /// });
    ///
    /// // assert_eq!(count_parser.parse("3aaa"), Ok(("", Box::new(["a", "a", "a"]))));
    /// ```
    fn bind_output<Out2, BindFun, Ret: Parser<Input, Out2, Error>>(
        self,
        f: BindFun,
    ) -> impl Parser<Input, Out2, Error>
    where
        BindFun: Fn(Output) -> Ret,
        Self: Sized,
    {
        move |input: Input| {
            let (rest, ret) = self.parse(input)?;
            f(ret).parse(rest)
        }
    }


    fn bind_output_mapping_error<Out2,Err2, BindFun,ErrMapper, Ret: Parser<Input, Out2, Err2>>(
        self,
        bind_fun: BindFun,
        err_map: ErrMapper,

    ) -> impl Parser<Input, Out2, Err2>
    where
        BindFun: Fn(Output) -> Ret,
        ErrMapper: Fn(Error) -> Err2,
        Self: Sized,
    Err2 : Clone,
    Input : Parsable<Err2>
    {
        move |input: Input| {
            match self.parse(input) {
                Ok((rest,out)) => bind_fun(out).parse(rest),
                Err((rest,err)) => Err((rest,err_map(err))),
            }
        }
    }

    //TODO document
    fn bind_err<Out2,Err2, BindFun,ErrFun, Ret: Parser<Input, Out2, Err2>>(
        self,
        f: BindFun,
        err:Err2
    ) -> impl Parser<Input, Out2, Err2>
    where
        BindFun: Fn(Error) -> Ret,
        Self: Sized,
    Err2 : Clone,
    Input : Parsable<Err2>
    {
        move |input: Input| {
            match self.parse(input) {
                Ok((rest,_out)) => Err((rest,err.clone())),
                Err((rest,err)) => f(err).parse(rest),
            }

        }
    }

    //TODO document also this is not general enough (stateful parsers should be able to bind with their state as well ie not enough args) 
    fn bind<Out2,Err2, OutBind,ErrBind, Ret: Parser<Input, Out2, Err2>>(
        self,
        out_bind: OutBind,
        err_bind: ErrBind,
    ) -> impl Parser<Input, Out2, Err2>
    where
        OutBind: Fn(Output) -> Ret,
        ErrBind: Fn(Error) -> Ret,
        Self: Sized,
        Err2 : Clone,
        Input : Parsable<Err2>
    {
        move |input: Input| {
            match self.parse(input) {
                Ok((rest,out)) => out_bind(out).parse(rest),
                Err((rest,err)) => err_bind(err).parse(rest),
            }
        }
    }



    /// Sequences this parser with another parser.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "hello".make_literal_matcher("Expected hello")
    ///     .seq("world".make_literal_matcher("Expected world"));
    ///
    /// assert_eq!(parser.parse("helloworld"), Ok(("", ("hello", "world"))));
    /// ```
    fn seq<Output2, Error2>(
        self,
        p: impl Parser<Input, Output2, Error2>,
    ) -> impl SeqParser<Input, Output, Output2, Error, Error2>
    where
        Self: Sized,
        Error2: Clone,
        Input: Parsable<Error2> + Parsable<Either<Error, Error2>>,
    {
        move |input: Input| match self.parse(input) {
            Ok((rest, result)) => match p.parse(rest) {
                Ok((rest2, result2)) => Ok((rest2, (result, result2))),
                Err((rest2, err1)) => Err((rest2, Either::Right(err1))),
            },
            Err((rest, err1)) => Err((rest, Either::Left(err1))),
        }
    }

    /// Runs this parser followed by the skip parser, but only returns the result of this parser.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "hello".make_literal_matcher("Expected hello")
    ///     .skip(" ".make_literal_matcher("Expected space"));
    ///
    /// assert_eq!(parser.parse("hello world"), Ok(("world", "hello")));
    /// ```
    fn skip<Output2>(
        self,
        skip_parser: impl Parser<Input, Output2, Error>,
    ) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        Input: Parsable<Error> + Parsable<Either<Error, Error>>,
    {
        self.seq(skip_parser)
            .map(|(out, _)| out)
            .map_err(|x| match x {
                Either::Left(l) => l,
                Either::Right(l) => l,
            })
    }
    /// Tries this parser, and if it fails, tries the alternative parser.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "yes".make_literal_matcher("Expected yes")
    ///     .alt("no".make_literal_matcher("Expected no"));
    ///
    /// assert_eq!(parser.parse("yes"), Ok(("", Either::Left("yes"))));
    /// assert_eq!(parser.parse("no"), Ok(("", Either::Right("no"))));
    /// assert_eq!(parser.parse("maybe"), Err(("maybe", ("Expected yes", "Expected no"))));
    /// ```
    fn alt<Output2, Error2>(
        self,
        p: impl Parser<Input, Output2, Error2>,
    ) -> impl AltParser<Input, Output, Output2, Error, Error2>
    where
        Self: Sized,
        Error2: Clone,
        Input: Parsable<Error2> + Parsable<(Error, Error2)>,
    {
        move |input: Input| match self.parse(input) {
            Ok((rest, ret)) => Ok((rest, Either::Left(ret))),
            Err((rest, e1)) => match p.parse(rest) {
                Ok((rest, ret)) => Ok((rest, Either::Right(ret))),
                Err((rest, e2)) => Err((rest, (e1, e2))),
            },
        }
    }

    /// Makes the parser optional, always succeeding with None if the parser fails.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "optional".make_literal_matcher("Expected optional").maybe();
    ///
    /// assert_eq!(parser.parse("optional"), Ok(("", Some("optional"))));
    /// assert_eq!(parser.parse("something"), Ok(("something", None)));
    /// ```
    fn maybe(self) -> impl MaybeParser<Input, Output, Error>
    where
        Self: Sized,
    {
        move |input: Input| match self.parse(input) {
            Ok((rest, ret)) => Ok((rest, Some(ret))),
            Err((rest, _e1)) => Ok((rest, None)),
        }
    }

    /// Applies the parser zero or more times, collecting all results.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "a".make_literal_matcher("Expected a").many();
    ///
    /// assert_eq!(parser.parse(""), Ok(("", vec![])));
    /// assert_eq!(parser.parse("a"), Ok(("", vec!["a"])));
    /// assert_eq!(parser.parse("aaa"), Ok(("", vec!["a", "a", "a"])));
    /// assert_eq!(parser.parse("aaab"), Ok(("b", vec!["a", "a", "a"])));
    /// ```
    fn many(self) -> impl ManyParser<Input, Output, Error>
    where
        Self: Sized,
        Input: PartialEq,
    {
        move |input: Input| {
            let mut result = Vec::new();
            let mut rest = input;

            loop {
                //let rest_ref = &rest;
                match self.parse(rest) {
                    Ok((new_rest, ret)) => {
                        /*
                        //we probably dont need this
                        if &new_rest == rest_ref {
                            break;
                        }*/
                        rest = new_rest;
                        result.push(ret);
                    }
                    Err((new_rest, _err)) => {
                        rest = new_rest;
                        break;
                    }
                }
            }

            Ok((rest, result))
        }
    }

    /// Applies the parser at least n times, returning an error if fewer than n matches are found.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "a".make_literal_matcher("Expected a")
    ///     .at_least_n(2, "Need at least 2 'a's");
    ///
    /// assert_eq!(parser.parse("aaa"), Ok(("a", vec!["a", "a"])));
    /// assert_eq!(parser.parse("a"), Err(("", "Need at least 2 'a's")));
    /// ```
    fn at_least_n(self, n: usize, err: Error) -> impl AtLeastNParser<Input, Output, Error>
    where
        Self: Sized,
        Input: PartialEq ,
        Error: Clone,
    {
        move |input: Input| {
            let mut result = Vec::new();
            let mut rest = input;
            let mut remaining = n;

            while remaining > 0 {
                match self.parse(rest) {
                    Ok((new_rest, ret)) => {
                        /*if new_rest == rest {
                            break;
                        }*/
                        rest = new_rest;
                        result.push(ret);
                        remaining -= 1;
                    }
                    Err((rest, _)) => {
                        return Err((rest, err.clone()));
                    }
                }
            }

            Ok((rest, result))
        }
    }

    /// Applies the parser at most N times, collecting all results.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "a".make_literal_matcher("Expected a")
    ///     .at_most_n::<3>();
    ///
    /// // Will parse up to 3 'a's
    /// let result = parser.parse("aaaaa");
    /// assert_eq!(result.unwrap().0, "aa"); // Remaining: "aa"
    /// ```
    fn at_most_n<const N: usize>(self) -> impl AtMostNParser<N, Input, Output, Error>
    where
        Self: Sized,
        Error: Clone,
        Input: PartialEq,
        Output: Copy,
    {
        move |input: Input| {
            let mut result = [None; N];
            let mut rest = input;
            let mut remaining = N;

            while remaining > 0 {
                match self.parse(rest) {
                    Ok((new_rest, ret)) => {
                        /*if new_rest == rest {
                            break;
                        }*/
                        rest = new_rest;
                        result[N - remaining] = Some(ret);
                        remaining -= 1;
                    }
                    Err((new_rest, _)) => { rest = new_rest;  break},
                }
            }
            Ok((rest, Box::new(result)))
        }
    }

    /// Applies the parser exactly N times, returning an error if fewer than N matches are found.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "a".make_literal_matcher("Expected a")
    ///     .exactly_n::<2>("Need exactly 2 'a's");
    ///
    /// assert_eq!(parser.parse("aa"), Ok(("", Box::new(["a", "a"]))));
    /// assert_eq!(parser.parse("a"), Err(("a", "Need exactly 2 'a's")));
    /// assert_eq!(parser.parse("aaa"), Ok(("a", Box::new(["a", "a"]))));
    /// ```
    fn exactly_n<const N: usize>(self, err: Error) -> impl ExactlyNParser<N, Input, Output, Error>
    where
        Self: Sized,
        Error: Clone,
        Input:  PartialEq,
        Output: Copy,
    {
        move |input: Input| {
            let mut result = Vec::with_capacity(N);
            let mut rest = input;
            let mut remaining = N;
            let mut maybe_err = None;
            while remaining > 0 {
                match self.parse(rest) {
                    Ok((new_rest, ret)) => {
                        /*if new_rest == rest {
                            break;
                        }*/
                        rest = new_rest;
                        result.push(ret);
                        remaining -= 1;
                    }
                    Err((new_rest, err)) => { rest = new_rest; maybe_err = Some(err);  break;},
                }
            }

            if let Some(err) = maybe_err{
                return Err((rest, err.clone()))
            }
            if result.len() == N {
                result.shrink_to_fit();
                if let Some((r, _)) = result.split_first_chunk_mut::<N>() {
                    Ok((rest, Box::new(r.to_owned())))
                } else {
                    Err((rest, err.clone()))
                }
            } else {
                Err((rest, err.clone()))
            }
        }
    }

    /// Tries to recover from an error using a recovery function.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "valid".make_literal_matcher("Not valid")
    ///     .recover_with(|_| "fallback".make_literal_matcher("Not fallback"));
    ///
    /// assert_eq!(parser.parse("valid"), Ok(("", "valid")));
    /// assert_eq!(parser.parse("fallback"), Ok(("", "fallback")));
    /// assert_eq!(parser.parse("invalid"), Err(("invalid", "Not fallback")));
    /// ```
    fn recover_with<P, F>(self, recovery: F) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        P: Parser<Input, Output, Error>,
        F: Fn(Error) -> P,
        Input: Clone,
    {
        move |input: Input| {
            self.parse(input.clone())
                .or_else(|(_, err)| recovery(err).parse(input))
        }
    }
    /// Succeeds if the given parser fails, returning the original input.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "a".make_literal_matcher("Expected a").not("Not expected a");
    ///
    /// assert_eq!(parser.parse("b"), Ok(("b", ()))); // Success when "a" parser fails
    /// assert_eq!(parser.parse("a"), Err(("a", "Not expected a"))); // Error when "a" parser succeeds
    /// ```
    fn not(self, err: Error) -> impl Parser<Input, (), Error>
    where
        Self: Sized,
        Input: Clone,
        Error: Clone,
    {
        move |input: Input| match self.parse(input.clone()) {
            Ok(_) => Err((input, err.clone())),
            Err(_) => Ok((input, ())),
        }
    }

    /// Looks ahead in the input stream without consuming it.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "abc".make_literal_matcher("Not abc").peek();
    ///
    /// // peek doesn't consume the input
    /// assert_eq!(parser.parse("abcdef"), Ok(("abcdef", "abc")));
    /// ```
    fn peek(self) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        Input: Clone,
    {
        move |input: Input| {
            let (_, output) = self.parse(input.clone())?;
            Ok((input, output))
        }
    }

    /// Similar to peek, but with more control over the behavior.
    ///
    /// - Allows examining the input without consuming it
    /// - Can be combined with `not` for negative lookahead
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// // Check if the next characters are "abc" without consuming them
    /// let parser = "abc".make_literal_matcher("Not abc").lookahead();
    ///
    /// assert_eq!(parser.parse("abcdef"), Ok(("abcdef", "abc"))); // Input is not consumed
    /// ```
    fn lookahead(self) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        Input: Clone,
    {
        move |input: Input| match self.parse(input.clone()) {
            Ok((_, output)) => Ok((input, output)),
            Err((_, err)) => Err((input, err)),
        }
    }

    /// Parses content between two delimiters, returning only the content.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let open = "(".make_literal_matcher("Expected opening paren");
    /// let close = ")".make_literal_matcher("Expected closing paren");
    /// let content = "hello".make_literal_matcher("Expected content");
    ///
    /// let parser = content.surrounded_by(open, close);
    ///
    /// assert_eq!(parser.parse("(hello)"), Ok(("", "hello")));
    /// assert_eq!(parser.parse("hello)"), Err(( "hello)", Either3::Left("Expected opening paren"))));
    /// ```
    fn surrounded_by<OutputLeft, OutputRight, ErrorLeft, ErrorRight>(
        self,
        left: impl Parser<Input, OutputLeft, ErrorLeft>,
        right: impl Parser<Input, OutputRight, ErrorRight>,
    ) -> impl Parser<Input, Output, Either3<ErrorLeft, Error, ErrorRight>>
    where
        Self: Sized,
        Error: Clone,
        ErrorLeft: Clone,
        ErrorRight: Clone,
        Input: Parsable<ErrorLeft>
            + Parsable<ErrorRight>
            + Parsable<Either3<ErrorLeft, Error, ErrorRight>>,
    {
        move |input: Input| match left.parse(input) {
            Ok((rest1, _)) => match self.parse(rest1) {
                Ok((rest2, output)) => match right.parse(rest2) {
                    Ok((rest3, _)) => Ok((rest3, output)),
                    Err((rest3, err)) => Err((rest3, Either3::Right(err))),
                },
                Err((rest2, err)) => Err((rest2, Either3::Middle(err))),
            },
            Err((rest1, err)) => Err((rest1, Either3::Left(err))),
        }
    }

    /// Applies the parser repeatedly, separated by the separator parser.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let item = "item".make_literal_matcher("Expected item");
    /// let comma = ",".make_literal_matcher("Expected comma");
    /// let parser = item.sep_by(comma);
    ///
    /// assert_eq!(parser.parse("item,item,item"), Ok(("", vec!["item", "item", "item"])));
    /// assert_eq!(parser.parse("item"), Ok(("", vec!["item"])));
    /// assert_eq!(parser.parse(""), Ok(("", vec![])));
    /// ```
    fn sep_by(
        self,
        sep: impl Parser<Input, Output, Error>,
    ) -> impl Parser<Input, Vec<Output>, Error>
    where
        Self: Sized,
        Input: Clone,
    {
        move |mut input: Input| {
            let mut results = Vec::new();

            match self.parse(input.clone()) {
                Ok((rest, item)) => {
                    results.push(item);
                    input = rest;
                }
                Err(_) => return Ok((input, results)),
            }

            while let Ok((rest1, _)) = sep.parse(input.clone()) {
                if let Ok((rest2, item)) = self.parse(rest1) {
                    results.push(item);
                    input = rest2;
                } else {
                    break;
                }
            }

            Ok((input, results))
        }
    }

    /// Similar to `sep_by` but requires at least one item.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let item = "item".make_literal_matcher("Expected item");
    /// let comma = ",".make_literal_matcher("Expected comma");
    /// let parser = item.sep_by1(comma, "At least one item required");
    ///
    /// assert_eq!(parser.parse("item,item,item"), Ok(("", vec!["item", "item", "item"])));
    /// assert_eq!(parser.parse("item"), Ok(("", vec!["item"])));
    /// assert_eq!(parser.parse(""), Err(("", "At least one item required")));
    /// ```
    fn sep_by1(
        self,
        sep: impl Parser<Input, Output, Error>,
        err: Error,
    ) -> impl Parser<Input, Vec<Output>, Error>
    where
        Self: Sized,
        Input: Clone,
        Error: Clone,
    {
        move |input: Input| match self.parse(input.clone()) {
            Ok((rest, first)) => {
                let mut results = vec![first];
                let mut current_input = rest;

                while let Ok((rest1, _)) = sep.parse(current_input.clone()) {
                    match self.parse(rest1) {
                        Ok((rest2, item)) => {
                            results.push(item);
                            current_input = rest2;
                        }
                        Err(_) => break,
                    }
                }

                Ok((current_input, results))
            }
            Err(_) => Err((input, err.clone())),
        }
    }

    /// Chains parsers with left-associative operators.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let num = "1".make_literal_matcher("Not one").map(|_| 1);
    /// let add = "+".make_literal_matcher("No plus")
    ///     .map(|_| Box::new(|a: i32, b: i32| a + b) as Box<dyn Fn(i32, i32) -> i32>);
    ///
    /// let parser = num.chainl1(add);
    /// assert_eq!(parser.parse("1+1+1"), Ok(("", 3))); // (1+1)+1 = 3
    /// ```
    fn chainl1(
        self,
        op: impl Parser<Input, Box<dyn Fn(Output, Output) -> Output>, Error>,
    ) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        Input: Clone,
    {
        move |input: Input| {
            let (mut rest, mut acc) = self.parse(input.clone())?;

            while let Ok((new_rest, f)) = op.parse(rest.clone()) {
                match self.parse(new_rest.clone()) {
                    Ok((next_rest, val)) => {
                        acc = f(acc, val);
                        rest = next_rest;
                    }
                    Err(_) => break,
                }
            }

            Ok((rest, acc))
        }
    }

    /// Similar to `chainl1` but produces right-associative operations.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let num = "1".make_literal_matcher("Not one").map(|_| 1);
    /// let pow = "^".make_literal_matcher("No power")
    ///     .map(|_| Box::new(|a: i32, b: i32| a.pow(b as u32)) as Box<dyn Fn(i32, i32) -> i32>);
    ///
    /// let parser = num.chainr1(pow);
    /// assert_eq!(parser.parse("1^1^1"), Ok(("", 1))); // 1^(1^1) = 1^1 = 1
    /// ```
    fn chainr1(
        self,
        op: impl Parser<Input, Box<dyn Fn(Output, Output) -> Output>, Error>,
    ) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        Input: Clone,
        Output: Clone,
    {
        // Helper recursive function to handle the right-associative parsing
        fn parse_right<I, O, E, P, OP>(
            term_parser: &P,
            op_parser: &OP,
            input: I,
        ) -> Result<(I, O), (I, E)>
        where
            P: Parser<I, O, E>,
            OP: Parser<I, Box<dyn Fn(O, O) -> O>, E>,
            I: Parsable<E> + Clone,
            O: Clone,
            E: Clone,
        {
            // Parse the leftmost term
            let (rest, left_term) = term_parser.parse(input)?;

            // Try to parse an operator
            match op_parser.parse(rest.clone()) {
                Ok((rest_after_op, op_func)) => {
                    // Recursively parse the right side
                    match parse_right(term_parser, op_parser, rest_after_op) {
                        Ok((final_rest, right_term)) => {
                            // Apply the operator with right associativity
                            Ok((final_rest, op_func(left_term, right_term)))
                        }
                        Err(_) => {
                            // If right side fails, just return the left term
                            Ok((rest, left_term))
                        }
                    }
                }
                Err(_) => {
                    // No operator found, just return the term
                    Ok((rest, left_term))
                }
            }
        }

        // The actual parser implementation
        move |input: Input| parse_right(&self, &op, input)
    }

    /// Applies a function to the parser's output. Output is flattened .
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let fun_parser = "fun".make_literal_matcher("Expected fun")
    ///     .map(|_| |x: i32,y: i32| x + y);
    /// let arg_parser = "42".make_literal_matcher("Expected 42")
    ///     .map(|_| 42)
    ///     .seq("42".make_literal_matcher("Expected 42")
    ///     .map(|_| 11)).map_err(|_| "err");
    ///
    /// let result_parser = fun_parser.apply(arg_parser);
    /// assert_eq!(result_parser.parse("fun4242"), Ok(("", 53)));
    /// ```
    fn apply<Args, Out, T, P>(self, arg_supplier: P) -> impl Parser<Input, Out, Error>
    where
        Args: ApplicativeFuncArgs,
        P: Parser<Input, Args, Error>,
        T: ApplicativeFunc<Args, Out>,
        Self: Sized + Parser<Input, T, Error>,
    {
        move |input: Input| {
            let (rest, fun): (Input, T) = self.parse(input)?;
            let (rest, args) = arg_supplier.parse(rest)?;
            Ok((rest, fun.apply(args)))
        }
    }

    /// Parses content that is preceded by a specific parser, returning only the content.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let prefix = "#".make_literal_matcher("Expected #");
    /// let content = "tag".make_literal_matcher("Expected tag");
    ///
    /// let parser = content.preceded_by(prefix);
    ///
    /// assert_eq!(parser.parse("#tag"), Ok(("", "tag")));
    /// assert_eq!(parser.parse("tag"), Err(("tag", Either::Left("Expected #"))));
    /// ```
    fn preceded_by<OutputLeft, ErrorLeft>(
        self,
        prefix: impl Parser<Input, OutputLeft, ErrorLeft>,
    ) -> impl Parser<Input, Output, Either<ErrorLeft, Error>>
    where
        Self: Sized,
        Error: Clone,
        ErrorLeft: Clone,
        Input: Parsable<ErrorLeft> + Parsable<Either<ErrorLeft, Error>>,
    {
        move |input: Input| match prefix.parse(input) {
            Ok((rest, _)) => match self.parse(rest) {
                Ok((final_rest, output)) => Ok((final_rest, output)),
                Err((final_rest, err)) => Err((final_rest, Either::Right(err))),
            },
            Err((rest, err)) => Err((rest, Either::Left(err))),
        }
    }

    /// Tries to parse the input and backtracks on failure.
    ///
    /// Unlike normal parsing which may consume input even on failure,
    /// backtracking ensures the original input is returned on failure.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let parser = "abc".make_literal_matcher("Expected abc")
    ///     .seq("def".make_literal_matcher("Expected def"))
    ///     .backtrack();
    ///
    /// let parser2 = "abc".make_literal_matcher("Expected abc")
    ///     .seq("def".make_literal_matcher("Expected def"));
    ///
    /// // Without backtrack, this would leave partial input consumed
    /// assert_eq!(parser.parse("abcXYZ"), Err(("abcXYZ", Either::Right("Expected def"))));
    /// assert_eq!(parser2.parse("abcXYZ"), Err(("XYZ", Either::Right("Expected def"))));
    /// ```
    fn backtrack(self) -> impl Parser<Input, Output, Error>
    where
        Self: Sized,
        Input: Clone,
    {
        move |input: Input| match self.parse(input.clone()) {
            Ok(result) => Ok(result),
            Err((_, err)) => Err((input, err)),
        }
    }

    /// Tries both parsers and returns the results of both that succeeded.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let a = "a".make_literal_matcher("Not a");
    /// let b = "b".make_literal_matcher("Not b");
    ///
    /// let parser = a.or(b);
    ///
    /// assert_eq!(parser.parse("ab"), Ok(("ab", (Some(("b","a")), None))));
    /// assert_eq!(parser.parse("ba"), Ok(("ba", (None, Some(("a","b"))))));
    /// assert_eq!(parser.parse("c"), Err(("c", ("Not a", "Not b"))));
    /// ```
    fn or<Output2, Error2>(
        self,
        other: impl Parser<Input, Output2, Error2>,
    ) -> impl Parser<Input, (Option<(Input, Output)>, Option<(Input, Output2)>), (Error, Error2)>
    where
        Self: Sized,
        Input: Clone + Parsable<Error2> + Parsable<(Error, Error2)>,
        Error: Clone,
        Error2: Clone,
    {
        move |input: Input| {
            let first = self.parse(input.clone());
            let second = other.parse(input.clone());

            let ret = match (first, second) {
                (Ok(a), Ok(b)) => Ok((input, (Some(a), Some(b)))),
                (Ok(a), Err(_)) => Ok((input, (Some(a), None))),
                (Err(_), Ok(b)) => Ok((input, (None, Some(b)))),
                (Err((_, a)), Err((_, b))) => Err((input, (a, b))),
            };
            ret
        }
    }

    /// Applies a function to the parser's output.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let fun_parser = "fun".make_literal_matcher("Expected fun")
    ///     .map(|_| |x: i32| x * 2);
    /// let arg_parser = "42".make_literal_matcher("Expected 42")
    ///     .map(|_| 42);
    ///
    /// let result_parser = fun_parser.fapply(arg_parser);
    /// // Would return 84 (42 * 2)
    /// assert_eq!(result_parser.parse("fun42"), Ok(("", 84)));
    /// ```
    fn fapply<Args, Out, T, P>(self, arg_supplier: P) -> impl Parser<Input, Out, Error>
    where
        P: Parser<Input, Args, Error>,
        T: Fn(Args) -> Out,
        Self: Sized + Parser<Input, T, Error>,
    {
        move |input: Input| {
            let (rest, fun): (Input, T) = self.parse(input)?;
            let (rest, args) = arg_supplier.parse(rest)?;
            Ok((rest, fun(args)))
        }
    }

    /// Chains this parser with another parser that depends on the output of this parser.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use friss::*;
    ///
    /// let count_parser = "3".make_literal_matcher("Expected 3")
    ///     .and_then(|_| "abc".make_literal_matcher("Expected abc"));
    ///
    /// assert_eq!(count_parser.parse("3abc"), Ok(("", "abc")));
    /// ```
    fn and_then<F, T, NextOut>(self, f: F) -> impl Parser<Input, NextOut, Error>
    where
        Self: Sized,
        T: Parser<Input, NextOut, Error>,
        F: Fn(Output) -> T,
    {
        move |input: Input| {
            let (rest, out) = self.parse(input)?;
            f(out).parse(rest)
        }
    }

    /// Reverses the application order, applying a function from the parser to the arguments.
    ///
    /// ## Example
    ///
    /// let parser = (
    ///        [1].make_literal_matcher("1 failed"),
    ///        [2].make_literal_matcher("2 failed"),
    ///    )
    ///        .seq()
    ///        .rev_apply(move |input: &'a [i32]| Ok((input, move |a: &[i32], b: &[i32]| b[0] + a[0])));
    ///    match parser.parse(&[1, 2]) {
    ///        Ok((_, x)) => assert_eq!(x, 3),
    ///        _ => assert!(false, "Parser failed"),
    ///    }
    ///
    ///
    ///
    ///
    ///
    ///

    /// ```
    fn rev_apply<Args, Out, T, P>(self, fun_supplier: P) -> impl Parser<Input, Out, Error>
    where
        Args: ApplicativeFuncArgs,
        P: Parser<Input, T, Error>,
        T: ApplicativeFunc<Args, Out>,
        Self: Sized + Parser<Input, Args, Error>,
    {
        move |input: Input| {
            let (rest, fun): (Input, T) = fun_supplier.parse(input)?;
            let (rest, args) = self.parse(rest)?;
            Ok((rest, fun.apply(args)))
        }
    }
}
/// Creates a parser that always returns the given output without consuming input.
///
/// ## Example
///
/// ```rust
/// use friss::*;
/// use friss::core::Parser;
///
/// let value : i32 = 42;
/// let pure_parser : Box<dyn Parser<&str,i32,&str>> = Box::new(pure(value));
///
/// assert_eq!(pure_parser.parse("hello"), Ok(("hello", 42)));
/// ```
pub fn pure<Input, Output, Error>(out: Output) -> impl Parser<Input, Output, Error>
where
    Input: Parsable<Error> + Clone,
    Output: Clone,
    Error: Clone,
{
    move |input: Input| Ok((input, out.clone()))
}

/// Creates a parser that always fails with the given error.
///
/// ## Example
///
/// ```rust
/// use friss::*;
/// use friss::core::Parser;
///
/// let fail_parser : Box<dyn Parser<&str,(),&str>>= Box::new(fail("Always fails"));
///
/// assert_eq!(fail_parser.parse("anything"), Err(("anything", "Always fails")));
/// ```
pub fn fail<Input, Output, Error>(err: Error) -> impl Parser<Input, Output, Error>
where
    Input: Parsable<Error> + Clone,
    Error: Clone,
{
    move |input: Input| Err((input, err.clone()))
}

/// Creates a recursive parser that can reference itself.
pub fn recursive<Input, Output, Error, F>(f: F) -> Box<dyn Parser<Input, Output, Error>>
where
    Input: Parsable<Error> + Clone + 'static,
    Output: 'static,
    Error: Clone + 'static,
    F: Fn(Box<dyn Parser<Input, Output, Error>>) -> Box<dyn Parser<Input, Output, Error>> + 'static,
{
    let cell: std::rc::Rc<RefCell<Option<Box<dyn Parser<Input, Output, Error>>>>> =
        std::rc::Rc::new(RefCell::new(None));

    let cell_for_placeholder = cell.clone();

    let placeholder: Box<dyn Parser<Input, Output, Error>> = Box::new(move |input: Input| {
        // Borrow the inner parser and delegate to it
        let borrowed = cell_for_placeholder.as_ref().borrow();
        match &*borrowed {
            Some(parser) => parser.parse(input),
            None => panic!("Recursive parser used before being initialized"),
        }
    });

    let actual = f(placeholder);

    *cell.as_ref().borrow_mut() = Some(actual);

    let cell_for_final = cell.clone();

    Box::new(move |input: Input| {
        let borrowed = cell_for_final.as_ref().borrow();
        match &*borrowed {
            Some(parser) => parser.parse(input),
            None => panic!("Recursive parser not initialized"),
        }
    })
}

/// Marker trait for arguments to applicative functions.
pub trait ApplicativeFuncArgs {}

/// Trait for functions that can be applied to arguments.
pub trait ApplicativeFunc<Args: ApplicativeFuncArgs, Out> {
    /// Applies the function to the arguments.
    fn apply(self, args: Args) -> Out;
}

/// Trait for parsers that sequence two parsers together.
pub trait SeqParser<
    Input: Parsable<Error1> + Parsable<Error2> + Parsable<Either<Error1, Error2>>,
    Output1,
    Output2,
    Error1: Clone,
    Error2: Clone,
>: Parser<Input, (Output1, Output2), Either<Error1, Error2>>
{
    /// Returns only the first component of the tuple result.
    fn first(self) -> impl Parser<Input, Output1, Either<Error1, Error2>>
    where
        Self: Sized,
    {
        move |input: Input| {
            let (rest, (ret, _)) = self.parse(input)?;
            Ok((rest, ret))
        }
    }

    /// Returns only the second component of the tuple result.
    fn second(self) -> impl Parser<Input, Output2, Either<Error1, Error2>>
    where
        Self: Sized,
    {
        move |input: Input| {
            let (rest, (_, ret)) = self.parse(input)?;
            Ok((rest, ret))
        }
    }
}

/// Trait for parsers that provide alternative choices.
pub trait AltParser<
    Input: Parsable<Error1> + Parsable<Error2> + Parsable<(Error1, Error2)>,
    Output1,
    Output2,
    Error1: Clone,
    Error2: Clone,
>: Parser<Input, Either<Output1, Output2>, (Error1, Error2)>
{
}

/// Trait for parsers that make a result optional.
pub trait MaybeParser<Input: Parsable<Error>, Output, Error: Clone>:
    Parser<Input, Option<Output>, Error>
{
}

/// Trait for parsers that apply a parser many times.
pub trait ManyParser<Input: Parsable<Error>, Output, Error: Clone>:
    Parser<Input, Vec<Output>, Error>
{
}

/// Trait for parsers that apply a parser at least N times.
pub trait AtLeastNParser<Input: Parsable<Error>, Output, Error: Clone>:
    Parser<Input, Vec<Output>, Error>
{
}

/// Trait for parsers that apply a parser at most N times.
pub trait AtMostNParser<const N: usize, Input: Parsable<Error>, Output, Error: Clone>:
    Parser<Input, Box<[Option<Output>; N]>, Error>
{
}

/// Trait for parsers that apply a parser exactly N times.
pub trait ExactlyNParser<const N: usize, Input: Parsable<Error>, Output, Error: Clone>:
    Parser<Input, Box<[Output; N]>, Error>
{
}

// Implement the `Parser` trait for functions that match the parser signature
impl<Input, Output, Error, Function> Parser<Input, Output, Error> for Function
where
    Function: Fn(Input) -> Result<(Input, Output), (Input, Error)>,
    Input: Parsable<Error>,
    Error: Clone,
{
    fn parse(&self, input: Input) -> Result<(Input, Output), (Input, Error)> {
        self(input)
    }
}

// Implement specialized parser traits for functions
impl<Input, Output1, Output2, Error1, Error2, Function>
    SeqParser<Input, Output1, Output2, Error1, Error2> for Function
where
    Function: Fn(Input) -> Result<(Input, (Output1, Output2)), (Input, Either<Error1, Error2>)>,
    Input: Parsable<Error1> + Parsable<Error2> + Parsable<Either<Error1, Error2>>,
    Error1: Clone,
    Error2: Clone,
{
}

impl<Input, Output1, Output2, Error1, Error2, Function>
    AltParser<Input, Output1, Output2, Error1, Error2> for Function
where
    Function: Fn(Input) -> Result<(Input, Either<Output1, Output2>), (Input, (Error1, Error2))>,
    Input: Parsable<Error1> + Parsable<Error2> + Parsable<(Error1, Error2)>,
    Error1: Clone,
    Error2: Clone,
{
}

impl<Input, Output, Error, Function> MaybeParser<Input, Output, Error> for Function
where
    Function: Fn(Input) -> Result<(Input, Option<Output>), (Input, Error)>,
    Input: Parsable<Error>,
    Error: Clone,
{
}

impl<Input, Output, Error, Function> ManyParser<Input, Output, Error> for Function
where
    Function: Fn(Input) -> Result<(Input, Vec<Output>), (Input, Error)>,
    Input: Parsable<Error>,
    Error: Clone,
{
}

impl<Input, Output, Error, Function> AtLeastNParser<Input, Output, Error> for Function
where
    Function: Fn(Input) -> Result<(Input, Vec<Output>), (Input, Error)>,
    Input: Parsable<Error>,
    Error: Clone,
{
}

impl<const N: usize, Input, Output, Error, Function> AtMostNParser<N, Input, Output, Error>
    for Function
where
    Function: Fn(Input) -> Result<(Input, Box<[Option<Output>; N]>), (Input, Error)>,
    Input: Parsable<Error>,
    Error: Clone,
{
}

impl<const N: usize, Input, Output, Error, Function> ExactlyNParser<N, Input, Output, Error>
    for Function
where
    Function: Fn(Input) -> Result<(Input, Box<[Output; N]>), (Input, Error)>,
    Input: Parsable<Error>,
    Error: Clone,
{
}

// Implement ParsableItem
impl<Error: Clone, T, Parent> ParsableItem<Parent, Error> for T
where
    Parent: Parsable<Error, Item = T>,
{
    fn make_character_matcher(self, err: Error) -> impl Parser<Parent, Self, Error> {
        Parent::make_item_matcher(self, err)
    }
}
