use std::{cell::RefCell, marker::PhantomData};

use crate::{core::ParserOutput, Parsable, Parser};

/// A container that carries both parser state and input.
///
/// The `StateCarrier` is fundamental to stateful parsing, enabling parsers to track
/// and maintain state while processing input. This allows for more advanced parsing
/// capabilities like tracking line/column positions, indentation levels, and other
/// contextual information.
///
/// # Type Parameters
///
/// * `State`: The type representing the state to be carried (e.g., `Position`, `Offset`)
/// * `Input`: The input type being parsed (e.g., `&str`, `&[T]`)
///
/// # Examples
///
/// ```rust
/// use friss::state::*;
/// use friss::parsers::*;
///
/// // Create a state carrier with position information
/// let carrier = "hello".with_state(Position::new(1, 1));
///
/// // Access state and input
/// println!("Line: {}, Column: {}", carrier.state.line, carrier.state.column);
/// println!("Input: {}", carrier.input);
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct StateCarrier<State, Input> {
    pub state: State,
    pub input: Input,
}

impl<State, Input> StateCarrier<State, Input> {
    /// Create a new state carrier
    pub fn new(state: State, input: Input) -> Self {
        StateCarrier { state, input }
    }

    /// Map the state using a function
    pub fn map_state<F, NewState>(self, f: F) -> StateCarrier<NewState, Input>
    where
        F: FnOnce(State) -> NewState,
    {
        StateCarrier {
            state: f(self.state),
            input: self.input,
        }
    }

    /// Map the input using a function
    pub fn map_input<F, NewInput>(self, f: F) -> StateCarrier<State, NewInput>
    where
        F: FnOnce(Input) -> NewInput,
    {
        StateCarrier {
            state: self.state,
            input: f(self.input),
        }
    }
}

/// A parser that includes state transition handling.
///
/// This struct wraps a regular parser and adds state transition capabilities through
/// success and failure handlers. These handlers define how state should be updated
/// when parsing succeeds or fails.
///
/// # Type Parameters
///
/// * `S`: State type
/// * `I`: Input type
/// * `O`: Output type
/// * `E`: Error type
/// * `P`: The underlying parser type
/// * `SuccesT`: Success handler function type
/// * `ErrorT`: Error handler function type
///
/// # Examples
///
/// ```rust
/// use friss::*;
/// use friss::core::*;
/// use friss::state::*;
/// use friss::parsers::*;
///
/// // Create a parser that tracks character positions
/// let char_parser = <&str as Parsable<&str>>::make_anything_matcher("Expected character")
///     .with_state_transition(
///         |mut pos: Position, _input, char, _orig| {
///             // Update position based on character
///             if char == '\n' {
///                 pos.advance_line();
///             } else {
///                 pos.advance_column(1);
///             }
///             (pos, _input, char)
///         },
///         |pos, input, err, _orig| (pos, input, err)
///     );
/// ```
pub struct ParserWithStateTransition<S, I, O, E, P, SuccesT, ErrorT> {
    parser: P,
    on_success: Option<RefCell<SuccesT>>,
    on_error: Option<RefCell<ErrorT>>,
    _phantom: PhantomData<(I, O, S, E)>,
}

impl<State, Input, Output, Error, P, SuccessT, ErrorT>
    ParserWithStateTransition<State, Input, Output, Error, P, SuccessT, ErrorT>
{
    /// Creates a new stateful parser with success and failure handlers.
    ///
    /// # Parameters
    ///
    /// * `parser`: The underlying parser to wrap
    /// * `success`: Handler function called when parsing succeeds
    /// * `fail`: Handler function called when parsing fails
    ///
    /// # Returns
    ///
    /// A new `ParserWithStateTransition` that manages state transitions
    pub fn new_with_success_and_fail(parser: P, success: SuccessT, fail: ErrorT) -> Self
    where
        for<'a> SuccessT: FnMut(State, Input, Output, Input) -> (State, Input, Output) + 'a,
        for<'a> ErrorT: FnMut(State, Input, Error, Input) -> (State, Input, Error) + 'a,
    {
        ParserWithStateTransition {
            parser,
            on_success: Some(RefCell::new(success)),
            on_error: Some(RefCell::new(fail)),
            _phantom: PhantomData,
        }
    }

    /// Map the parser
    pub fn map_parser<F, NewP>(
        self,
        f: F,
    ) -> ParserWithStateTransition<State, Input, Output, Error, NewP, SuccessT, ErrorT>
    where
        F: FnOnce(P) -> NewP,
    {
        ParserWithStateTransition {
            parser: f(self.parser),
            on_success: self.on_success,
            on_error: self.on_error,
            _phantom: PhantomData,
        }
    }

    fn succes_helper(
        &self,
        state: State,
        input: Input,
        output: Output,
        input_ref: Input,
    ) -> Result<(StateCarrier<State, Input>, Output), (StateCarrier<State, Input>, Error)>
    where
        SuccessT: FnMut(State, Input, Output, Input) -> (State, Input, Output),
    {
        match &self.on_success {
            Some(f) => {
                let (state, input, output) = (f.borrow_mut())(state, input, output, input_ref);
                Ok((StateCarrier { state, input }, output))
            }
            None => Ok((StateCarrier { state, input }, output)),
        }
    }

    fn error_helper(
        &self,
        state: State,
        input: Input,
        error: Error,
        input_ref: Input,
    ) -> Result<(StateCarrier<State, Input>, Output), (StateCarrier<State, Input>, Error)>
    where
        ErrorT: FnMut(State, Input, Error, Input) -> (State, Input, Error),
    {
        match &self.on_error {
            Some(f) => {
                let (state, input, error) = (f.borrow_mut())(state, input, error, input_ref);
                Err((StateCarrier { state, input }, error))
            }
            None => Err((StateCarrier { state, input }, error)),
        }
    }
}

impl<State, Input, Output, Error, P, SuccesT, ErrorT>
    Parser<StateCarrier<State, Input>, Output, Error>
    for ParserWithStateTransition<State, Input, Output, Error, P, SuccesT, ErrorT>
where
    Error: Clone,
    Output: ParserOutput,
    StateCarrier<State, Input>: Parsable<Error>,
    Input: Parsable<Error> + Clone,
    P: Parser<Input, Output, Error>,
    SuccesT: FnMut(State, Input, Output, Input) -> (State, Input, Output),
    ErrorT: FnMut(State, Input, Error, Input) -> (State, Input, Error),
{
    fn parse(
        &self,
        StateCarrier { state, input }: StateCarrier<State, Input>,
    ) -> Result<(StateCarrier<State, Input>, Output), (StateCarrier<State, Input>, Error)> {
        let input_ref = input.clone();
        match self.parser.parse(input) {
            Ok((input, output)) => self.succes_helper(state, input, output, input_ref),
            Err((input, error)) => self.error_helper(state, input, error, input_ref),
        }
    }
}

/// A trait for parsers that manage state during parsing.
///
/// This trait extends the core `Parser` trait with methods specifically for
/// state management. It enables parsers to track and manipulate state as they
/// process input, making it possible to handle context-sensitive grammars,
/// maintain position information, and track other forms of state.
///
/// # Type Parameters
///
/// * `State`: The state type to track (must implement `Default`)
/// * `Input`: The input type being parsed
/// * `Output`: The output type produced by parsing
/// * `Error`: The error type returned on parse failures
///
/// # Examples
///
/// ```rust
/// use friss::*;
/// use friss::state::*;
/// use friss::parsers::*;
///
/// // Create a stateful parser that tracks line and column position
/// let newline_parser = '\n'.make_character_matcher("Expected newline")
///     .with_state_transition(
///         |mut pos: Position, input, output, _orig| {
///             pos.advance_line();
///             (pos, input, output)
///         },
///         |pos, input, error, _orig| (pos, input, error)
///     );
///
/// // Use state capture to get position information
/// let pos_tracker = newline_parser.inject_current_state_to_output();
/// ```
pub trait StatefulParser<State: Default, Input, Output, Error>:
    Parser<StateCarrier<State, Input>, Output, Error>
where
    Input: Parsable<Error>,
    StateCarrier<State, Input>: Parsable<Error>,
    Output: ParserOutput,
    Error: Clone,
{
    /// Gets the underlying non-stateful parser.
    ///
    /// This method extracts the base parser without state handling.
    fn underlying_parser(self) -> impl Parser<Input, Output, Error>;

    /// Parses input with a specific initial state.
    ///
    /// This is a convenience method that wraps input in a state carrier
    /// and then applies the parser.
    ///
    /// # Parameters
    ///
    /// * `input`: The input to parse
    /// * `state`: The initial state to use
    ///
    /// # Returns
    ///
    /// The parse result with updated state information
    fn parse_with_state(
        &self,
        input: Input,
        state: State,
    ) -> Result<(StateCarrier<State, Input>, Output), (StateCarrier<State, Input>, Error)> {
        self.parse(StateCarrier::new(state, input))
    }

    /// Creates a parser that returns the state before parsing.
    ///
    /// This is useful for capturing the initial state for comparison or logging.
    ///
    /// # Returns
    ///
    /// A parser that returns the state as it was before parsing
    fn get_last_state(self) -> impl Parser<StateCarrier<State, Input>, State, Error>
    where
        Self: Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let state_clone = input.state.clone();
            let (rest, _) = self.parse(input)?;
            Ok((rest, state_clone))
        }
    }

    /// Creates a parser that returns the state after parsing.
    ///
    /// This is useful for capturing the final state after a parsing operation.
    ///
    /// # Returns
    ///
    /// A parser that returns the state as it is after parsing
    fn get_current_state(self) -> impl Parser<StateCarrier<State, Input>, State, Error>
    where
        Self: Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let (rest, _) = self.parse(input)?;
            let state_clone = rest.state.clone();
            Ok((rest, state_clone))
        }
    }

    /// Creates a parser that returns both the initial and final states.
    ///
    /// This is useful for tracking state changes during parsing.
    ///
    /// # Returns
    ///
    /// A parser that returns a tuple of (last_state, current_state)
    fn get_last_and_current_state(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, (State, State), Error>
    where
        Self: Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            let (rest, _) = self.parse(input)?;
            let current = rest.state.clone();
            Ok((rest, (last, current)))
        }
    }

    /// Creates a parser that injects the last (state before parsing) and current (state after parsing) states into both the output and error.
    ///
    /// This is useful for debugging and tracing state transitions.
    ///
    /// # Returns
    ///
    /// A parser with state information injected into all results
    fn inject_last_and_current_state(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, ((State, State), Output), ((State, State), Error)>
    where
        Self: Sized,
        State: Clone,
        StateCarrier<State, Input>: Parsable<((State, State), Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            match self.parse(input) {
                Ok((StateCarrier { state, input }, out)) => Ok((
                    StateCarrier {
                        state: state.clone(),
                        input,
                    },
                    ((last, state), out),
                )),
                Err((StateCarrier { state, input }, out)) => Err((
                    StateCarrier {
                        state: state.clone(),
                        input,
                    },
                    ((last, state), out),
                )),
            }
        }
    }

    /// Creates a parser that injects the last state (state before parsing) into both the output and error.
    ///
    /// # Returns
    ///
    /// A parser with initial state information injected into all results
    fn inject_last_state(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, (State, Output), (State, Error)>
    where
        Self: Sized,
        State: Clone,
        StateCarrier<State, Input>: Parsable<(State, Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            match self.parse(input) {
                Ok((rest, out)) => Ok((rest, (last, out))),
                Err((rest, out)) => Err((rest, (last, out))),
            }
        }
    }

    /// Creates a parser that injects the current state (state after parsing) into both the output and error.
    ///
    /// # Returns
    ///
    /// A parser with final state information injected into all results
    fn inject_current_state(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, (State, Output), (State, Error)>
    where
        Self: Sized,
        State: Clone,
        StateCarrier<State, Input>: Parsable<(State, Error)>,
    {
        move |input: StateCarrier<State, Input>| match self.parse(input) {
            Ok((StateCarrier { state, input }, out)) => Ok((
                StateCarrier {
                    state: state.clone(),
                    input,
                },
                (state, out),
            )),
            Err((StateCarrier { state, input }, out)) => Err((
                StateCarrier {
                    state: state.clone(),
                    input,
                },
                (state, out),
            )),
        }
    }

    /// Creates a parser that injects the last (state before parsing) and current (state after parsing) states into the output.
    ///
    /// This preserves the original error type.
    ///
    /// # Returns
    ///
    /// A parser that includes state information in the output but not errors
    fn inject_last_and_current_state_to_output(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, ((State, State), Output), Error>
    where
        Self: Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            let (rest, output) = self.parse(input)?;
            let current = rest.state.clone();
            Ok((rest, ((last, current), output)))
        }
    }

    /// Creates a parser that injects the last state (state before parsing) into the output.
    ///
    /// This preserves the original error type.
    ///
    /// # Returns
    ///
    /// A parser that includes last state information in the output
    fn inject_last_state_to_output(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, (State, Output), Error>
    where
        Self: Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let state_clone = input.state.clone();
            let (rest, output) = self.parse(input)?;
            Ok((rest, (state_clone, output)))
        }
    }

    /// Creates a parser that injects the current state (state after parsing) into the output.
    ///
    /// This preserves the original error type.
    ///
    /// # Returns
    ///
    /// A parser that includes current state information in the output

    fn inject_current_state_to_output(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, (State, Output), Error>
    where
        Self: Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let (rest, output) = self.parse(input)?;
            let state_clone = rest.state.clone();
            Ok((rest, (state_clone, output)))
        }
    }

    /// Creates a parser that injects the last state (state before parsing) into the error.
    ///
    /// This preserves the original output type.
    ///
    /// # Returns
    ///
    /// A parser that includes last state information in errors
    fn inject_last_state_to_error(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, Output, (State, Error)>
    where
        Self: Sized,
        State: Clone,
        StateCarrier<State, Input>: Parsable<(State, Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            let state_clone = input.state.clone();
            match self.parse(input) {
                Err((StateCarrier { state, input }, out)) => {
                    Err((StateCarrier { state, input }, (state_clone, out)))
                }
                Ok(x) => Ok(x),
            }
        }
    }

    /// Creates a parser that injects the current state (state after parsing) into the error.
    ///
    /// This preserves the original output type.
    ///
    /// # Returns
    ///
    /// A parser that includes current state information in errors
    fn inject_current_state_to_error(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, Output, (State, Error)>
    where
        Self: Sized,
        State: Clone,
        StateCarrier<State, Input>: Parsable<(State, Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            //let state_clone = input.state.clone();
            match self.parse(input) {
                Err((StateCarrier { state, input }, out)) => Err((
                    StateCarrier {
                        state: state.clone(),
                        input,
                    },
                    (state, out),
                )),
                Ok(x) => Ok(x),
            }
        }
    }

    /// Creates a parser that injects both the last (state before parsing) and current states (state after parsing) into the error.
    ///
    /// This preserves the original output type.
    ///
    /// # Returns
    ///
    /// A parser that includes both state values in errors
    fn inject_last_and_current_state_to_error(
        self,
    ) -> impl Parser<StateCarrier<State, Input>, Output, ((State, State), Error)>
    where
        Self: Sized,
        State: Clone,
        StateCarrier<State, Input>: Parsable<((State, State), Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            match self.parse(input) {
                Err((StateCarrier { state, input }, out)) => Err((
                    StateCarrier {
                        state: state.clone(),
                        input,
                    },
                    ((last, state), out),
                )),
                Ok(x) => Ok(x),
            }
        }
    }

    /// A state-aware binding operation for both success and error paths.
    ///
    /// This is a general-purpose binding method for stateful parsers that properly
    /// handles state transitions in both success and error cases.
    ///
    /// # Type Parameters
    ///
    /// * `OutBind`: Function type for binding successful parse results
    /// * `ErrBind`: Function type for binding parse failures
    /// * `P2`: The target parser type
    /// * `O2`: The output type of the target parser
    /// * `E2`: The error type of the target parser
    ///
    /// # Parameters
    ///
    /// * `out_bind`: Function that takes the current state and successful output and returns a new parser
    /// * `err_bind`: Function that takes the current state and error and returns a new parser
    ///
    /// # Returns
    ///
    /// A new stateful parser that applies the appropriate binding based on parse result
    ///
    /// # Examples
    ///
    /// ```rust
    /// use friss::*;
    /// use friss::state::*;
    /// use friss::parsers::*;
    ///
    /// // Create a parser that parses one thing based on another's result
    ///  let digit = "digit".with_state(Position::new(1, 1)).make_literal_matcher("no digit");
    ///
    ///  // Use general_bind to choose the next parser based on first
    ///  let p = digit.general_bind(
    ///      |_state, _digit| {
    ///              // create a new parser using state and output here
    ///             "haha".with_state(Position::new(1, 1)).make_literal_matcher("No haha")
    ///         },
    ///         |_state, _error| {
    ///             // create a new parser using state and error here
    ///             "hehe".with_state(Position::new(1, 1)).make_literal_matcher("No hehe")
    ///         }
    ///  );
    ///  let r1 = p.parse_with_state("digithaha",Position::new(0, 0));
    ///  assert_eq!(r1,Ok((StateCarrier::new(Position{column:9,line:0}, ""),StateCarrier::new(Position{column:1,line:1}, "haha"))))
    ///
    ///
    /// ```
    fn general_bind<OutBind, ErrBind, P2, O2, E2>(
        self,
        out_bind: OutBind,
        err_bind: ErrBind,
    ) -> impl StatefulParser<State, Input, O2, E2>
    where
        Self: Sized,
        OutBind: Fn(State, Output) -> P2,
        ErrBind: Fn(State, Error) -> P2,
        P2: StatefulParser<State, Input, O2, E2>,
        Input: Clone + Parsable<E2>,
        StateCarrier<State, Input>: Parsable<Error> + Parsable<E2>,
        E2: Clone,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| match self.parse(input) {
            Ok((StateCarrier { state, input }, out)) => {
                out_bind(state.clone(), out).parse(StateCarrier { state, input })
            }
            Err((StateCarrier { state, input }, err)) => {
                err_bind(state.clone(), err).parse(StateCarrier { state, input })
            }
        }
    }
}

impl<State, Input, Output, Error, P, SuccesT, ErrorT> StatefulParser<State, Input, Output, Error>
    for ParserWithStateTransition<State, Input, Output, Error, P, SuccesT, ErrorT>
where
    State: Default,
    Error: Clone,
    Output: ParserOutput,
    StateCarrier<State, Input>: Parsable<Error>,
    Input: Parsable<Error> + Clone,
    P: Parser<Input, Output, Error>,
    SuccesT: FnMut(State, Input, Output, Input) -> (State, Input, Output),
    ErrorT: FnMut(State, Input, Error, Input) -> (State, Input, Error),
{
    fn underlying_parser(self) -> impl Parser<Input, Output, Error> {
        self.parser
    }
}

impl<State, Input, Output, Error, Function> StatefulParser<State, Input, Output, Error> for Function
where
    Function:
        Fn(
            StateCarrier<State, Input>,
        )
            -> Result<(StateCarrier<State, Input>, Output), (StateCarrier<State, Input>, Error)>,
    Input: Parsable<Error> + Clone,
    StateCarrier<State, Input>: Parsable<Error>,
    Error: Clone,
    State: Default,
{
    fn underlying_parser(self) -> impl Parser<Input, Output, Error> {
        move |input| match self(StateCarrier::new(State::default(), input)) {
            Ok((StateCarrier { state: _, input }, out)) => Ok((input, out)),
            Err((StateCarrier { state: _, input }, out)) => Err((input, out)),
        }
    }
}

/*
pub fn lift_parser<State: Default, Input, Output, Error>(
    parser: impl Parser<Input, Output, Error>,
) -> impl StatefulParser<State, Input, Output, Error>
where
    Input: Clone + Parsable<Error>,
StateCarrier<State, Input>: Parsable<Error>,
    Error: Clone,
{
    move |carrier: StateCarrier<State, Input>| {
        let state = carrier.state;
        let input = carrier.input;

        match parser.parse(input) {
            Ok((rest, output)) => Ok((StateCarrier { state, input: rest }, output)),
            Err((rest, error)) => Err((StateCarrier { state, input: rest }, error)),
        }
    }
}
*/
