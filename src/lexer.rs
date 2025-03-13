//! # Lexer Utilities
//!
//! Provides ergonomic parsing utilities for lexical analysis, 
//! including whitespace handling, token parsing, and lexeme helpers.


use crate::{core::{Parsable, Parser}, Either3, Either4, ParsableItem, ParserSugar};

/// Extension trait for lexical parsing utilities
pub trait LexerExt<'a, Output, Error>: Parser<&'a str, Output, Error> 
where 
    Error: Clone,
    Self: Sized + 'a,
{
    /// Skip all whitespace characters (space, tab, newline, carriage return)
    fn skip_whitespace(self) -> impl Parser<&'a str, Output, Error> {
        self.surrounded_by(whitespace().many(), whitespace().many()).map_err(|err| match err {
            crate::Either3::Left(_) | crate::Either3::Right(_) => panic!("shall never happen"),
            crate::Either3::Middle(r) => r,
        })
    }

    /// Skip specific type of whitespace
    fn skip_spaces(self) -> impl Parser<&'a str, Output, Error> {
        self.preceded_by(space().many()).map_err(|err| match err {
            crate::Either::Left(_) => panic!("shall never happen"),
            crate::Either::Right(r) => r,
        })
    }

    /// Skip line comments 
    fn skip_line_comment(self, comment_start: &'a str) -> impl Parser<&'a str, Output, Error> {
        self.preceded_by(line_comment(comment_start).maybe()).map_err(|err| match err {
            crate::Either::Left(_) => panic!("shall never happen"),
            crate::Either::Right(r) => r,
        })
    }

    /// Skip block comments
    fn skip_block_comment(self, start: &'a str, end: &'a str) -> impl Parser<&'a str, Output, Error> {
        self.preceded_by(block_comment(start, end).maybe()).map_err(|err| match err {
            crate::Either::Left(_) => panic!("shall never happen"),
            crate::Either::Right(r) => r,
        })
    }

    /// Create a lexeme (token with surrounding whitespace stripped)
    fn lexeme(self) -> impl Parser<&'a str, Output, Error> {
        self.skip_whitespace()
    }
}

/// Implement LexerExt for all compatible parsers
impl<'a, P, Output, Error> LexerExt<'a, Output, Error> for P 
where 
    P: Parser<&'a str, Output, Error> + 'a,
    Error: Clone,
{
}

/// Parse a single space character
pub fn space<'a>() -> impl Parser<&'a str, char, &'a str> {
    ' '.make_character_matcher("Expected space")
}

/// Parse a single tab character
pub fn tab<'a>() -> impl Parser<&'a str, char, &'a str> {
    '\t'.make_character_matcher("Expected tab")
}

/// Parse a single newline character
pub fn newline<'a>() -> impl Parser<&'a str, char, &'a str> {
    '\n'.make_character_matcher("Expected newline")
}

/// Parse a single carriage return character
pub fn carriage_return<'a>() -> impl Parser<&'a str, char, &'a str> {
    '\r'.make_character_matcher("Expected carriage return")
}

/// Parse any whitespace character
pub fn whitespace<'a>() -> impl Parser<&'a str, char, &'a str> {
    (space(), tab(), newline(), carriage_return())
        .alt()
        .map(|x| match x {
            Either4::_1(x) | 
            Either4::_2(x) | 
            Either4::_3(x) | 
            Either4::_4(x) => x
        }).map_err(|_|"Expected Whitespace")
}

/// Parse line comment starting with a specific prefix
pub fn line_comment<'a>(comment_start: &'a str) -> impl Parser<&'a str, String, &'a str> {
    (
        comment_start.make_literal_matcher("Expected comment start"),
        <&'a str as Parsable<&'a str>>::make_anything_matcher("Expected line comment content")
            .validate(|c| *c != '\n', "Comment content")
            .many()
            .map(|chars| chars.into_iter().collect::<String>()),
        newline().many()
    )
    .seq()
    .map(|(_, content, _)| content)
    .map_err(|x| match x {
        Either3::Left(e) | Either3::Middle(e) | Either3::Right(e) => e,
    })
}

/// Parse block comment with specified start and end delimiters
pub fn block_comment<'a>(start: &'a str, end: &'a str) -> impl Parser<&'a str, String, &'a str> {
    (
        start.make_literal_matcher("Expected block comment start"),
        <&'a str as Parsable<&'a str>>::make_anything_matcher("Expected block comment content")
            .validate(|c| c != &end.chars().next().unwrap(), "Block comment content")
            .many()
            .map(|chars| chars.into_iter().collect::<String>()),
        end.make_literal_matcher("Expected block comment end")
    )
    .seq()
    .map(|(_, content, _)| content)
    .map_err(|x| match x {
        Either3::Left(e) | Either3::Middle(e) | Either3::Right(e) => e,
    })
}

/// Utility for converting a single character parser into a string parser
pub fn char_to_string<'a, Error: Clone>(parser: impl Parser<&'a str, char, Error>) 
    -> impl Parser<&'a str, String, Error> 
{
    parser.map(|c| c.to_string())
}

/// Utility to parse and convert parser output to uppercase
pub fn to_uppercase<'a, Output: Clone>(parser: impl Parser<&'a str, Output, &'a str>) 
    -> impl Parser<&'a str, Output, &'a str> 
{
    parser.map(|out| out)
}

/// Utility to parse and convert parser output to lowercase
pub fn to_lowercase<'a, Output: Clone>(parser: impl Parser<&'a str, Output, &'a str>) 
    -> impl Parser<&'a str, Output, &'a str> 
{
    parser.map(|out| out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::*;

    #[test]
    fn test_whitespace_parsing() {
        let space_parser = space();
        let tab_parser = tab();
        let newline_parser = newline();
        let whitespace_parser = whitespace();

        assert!(space_parser.parse(" ").is_ok());
        assert!(tab_parser.parse("\t").is_ok());
        assert!(newline_parser.parse("\n").is_ok());
        
        assert!(whitespace_parser.parse(" ").is_ok());
        assert!(whitespace_parser.parse("\t").is_ok());
        assert!(whitespace_parser.parse("\n").is_ok());
        assert!(whitespace_parser.parse("\r").is_ok());
    }

    #[test]
    fn test_line_comment() {
        let comment_parser = line_comment("//");

        assert_eq!(
            comment_parser.parse("// This is a comment\n\n\nmore text"), 
            Ok(("more text", " This is a comment".to_string()))
        );
        
        assert_eq!(
            comment_parser.parse("// This is a comment"), 
            Ok(("", " This is a comment".to_string()))
        );
    }

    #[test]
    fn test_block_comment() {
        let comment_parser = block_comment("/*", "*/");

        assert_eq!(
            comment_parser.parse("/* This is a block comment */more text"), 
            Ok(("more text", " This is a block comment ".to_string()))
        );
    }

    #[test]
    fn test_lexeme() {
        let parser = "hello".make_literal_matcher("Not hello");
        let lexeme_parser = parser.lexeme();

        assert_eq!(lexeme_parser.parse("hello  world"), Ok(("world", "hello")));
        //assert_eq!(lexeme_parser.parse("hello world"), Ok(("world", "hello")));
    }
}
