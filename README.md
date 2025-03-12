# 🦀 Friss: A Robust Parser Combinator Library for Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Status: In Development](https://img.shields.io/badge/Status-In%20Development-blue)

## What is Friss?

Friss is a parser combinator library for Rust that focuses on robust error handling and composition. Friss builds two trees simultaneously, one for errors and one for the syntax. 
> 💡 **Parser combinators** allow you to build complex parsers from simple components, similar to how you compose functions in functional programming.

## 🌟 Key Features

- **Comprehensive error handling** - Build trees of both syntax and errors simultaneously
- **Strongly-typed parser composition** - Leveraging Rust's type system to catch errors at compile time
- **Rich combinator ecosystem** - Sequence, alternative, repetition, validation, and more
- **Support for recursive parsing** - Parse nested structures with ease
- **No external dependencies** - Lightweight and focused

## 📦 Installation

> ⚠️ **Note:** Friss is not yet published to crates.io. 

For now, add it to your project by adding the following to your `Cargo.toml`:

```toml
[dependencies]
friss = { git = "https://github.com/username/friss" }
```

## 🚀 Quick Start

```rust
use friss::*;
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
                .map_err(|(a, b)| a)
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
```

## 🧩 Core Combinators

### Sequence (`seq`)

The `seq` combinator runs two parsers in sequence and returns both results as a tuple. When things go wrong, `seq` reports exactly *which* part of the sequence failed.

```rust
let parser = "hello ".make_literal_matcher("Expected hello")
    .seq("world".make_literal_matcher("Expected world"));

// Success case
assert_eq!(parser.parse("hello world"), Ok(("", ("hello ", "world"))));

// Failure case - first parser succeeds but second fails
assert_eq!(parser.parse("hello universe"), 
    Err(("universe", Either::Right("Expected world"))));

// Failure case - first parser fails
assert_eq!(parser.parse("hi world"), 
    Err(("hi world", Either::Left("Expected hello"))));
```

#### The Power of `Either`

Notice how the error is wrapped in an `Either::Right` or `Either::Left`? This tells you precisely which side of the sequence failed, so you can provide better error messages to users.

### Alternative (`alt`)

The `alt` combinator tries one parser, and if that fails, it tries an alternative parser. If both fail, it returns a tuple containing *both* errors.

```rust
let parser = "yes".make_literal_matcher("Expected yes")
    .alt("no".make_literal_matcher("Expected no"));

// Success cases
assert_eq!(parser.parse("yes"), Ok(("", Either::Left("yes"))));
assert_eq!(parser.parse("no"), Ok(("", Either::Right("no"))));

// Failure case - both parsers fail
assert_eq!(parser.parse("maybe"), 
    Err(("maybe", ("Expected yes", "Expected no"))));
```

#### Why Return Both Errors?

This approach allows for much more informative error messages. Instead of just saying "parse failed," you can say "expected either 'yes' or 'no'."

### Maybe (`maybe`)

The `maybe` combinator makes a parser optional, allowing it to always succeed:

```rust
let parser = "optional".make_literal_matcher("Expected optional").maybe();

assert_eq!(parser.parse("optional"), Ok(("", Some("optional"))));
assert_eq!(parser.parse("something"), Ok(("something", None))); // No error!
```

### Many (`many`)

The `many` combinator applies a parser zero or more times, collecting all results:

```rust
let parser = "a".make_literal_matcher("Expected a").many();

assert_eq!(parser.parse(""), Ok(("", vec![]))); // Zero matches is fine
assert_eq!(parser.parse("aaa"), Ok(("", vec!["a", "a", "a"])));
assert_eq!(parser.parse("aaab"), Ok(("b", vec!["a", "a", "a"])));
```

## 🧠 Advanced Features

### Recursive Parsing

Friss supports recursive parsers, which are essential for parsing nested structures like parentheses, JSON, XML, etc.

```rust
use friss::core::recursive;

// Parser for handling nested parentheses - counts the depth
let paren_parser = recursive(move |parser| {
    let nested = Box::new(
        '('.make_character_matcher("Expected opening paren")
            .seq(move |x| parser.parse(x))
            .map_err(|x| x.fold())
            .seq(')'.make_character_matcher("Expected closing paren"))
            .map_err(|x| x.fold())
            .map(|((_, inner), _)| inner + 1),
    );

    let empty = "".make_literal_matcher("").map(|_| 0);

    Box::new(
        nested
            .alt(empty)
            .map_err(|(a, _)| a)
            .map(|either| match either {
                Either::Left(depth) => depth,
                Either::Right(empty_result) => empty_result,
            }),
    )
});

assert_eq!(paren_parser.parse("()"), Ok(("", 1)));
assert_eq!(paren_parser.parse("(())"), Ok(("", 2)));
assert_eq!(paren_parser.parse("((()))"), Ok(("", 3)));
```

### Syntactic Sugar with Tuples

Friss provides syntactic sugar through the `ParserSugar` trait to make parser composition more ergonomic:

```rust
use friss::sugar::ParserSugar;

// Using tuple syntax for sequence
let seq_parser = (
    "hello".make_literal_matcher("Expected hello"),
    "world".make_literal_matcher("Expected world")
).seq();

// Using tuple syntax for alternatives
let alt_parser = (
    "yes".make_literal_matcher("Expected yes"),
    "no".make_literal_matcher("Expected no"),
    "maybe".make_literal_matcher("Expected maybe")
).alt();
```

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Contributions

Contributions are welcome! Feel free to submit issues or pull requests.
