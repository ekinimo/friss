use std::{cell::RefCell,  marker::PhantomData};

use crate::{core::ParserOutput, Parsable, Parser};

#[derive(Copy,Clone)]
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

pub struct ParserWithStateTransition<S, I, O, E, P, SuccesT, ErrorT> {
    parser: P,
    on_success: Option<RefCell<SuccesT>>,
    on_error: Option<RefCell<ErrorT>>,
    _phantom: PhantomData<(I, O, S, E)>,
}

impl<State, Input, Output, Error, P, SuccessT, ErrorT> ParserWithStateTransition<State, Input, Output, Error, P, SuccessT, ErrorT> {

    pub fn new_with_success_and_fail(parser: P,success:SuccessT,fail:ErrorT) -> Self
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
    pub fn map_parser<F, NewP>(self, f: F) -> ParserWithStateTransition<State, Input, Output, Error, NewP, SuccessT, ErrorT>
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





pub trait StatefulParser<State:Default, Input , Output, Error>: Parser<StateCarrier<State, Input>, Output, Error> where
    Input : Parsable<Error>,
    StateCarrier<State, Input> : Parsable<Error>,
    Output : ParserOutput,
    Error : Clone,
{

    

    fn underlying_parser(self)-> impl Parser<Input,Output,Error>;




    fn parse_with_state(
        &self,
        input:Input,state:State,
    ) -> Result<(StateCarrier<State, Input>, Output), (StateCarrier<State, Input>, Error)> {

         
        self.parse(StateCarrier::new(state, input))
    }
    fn get_last_state(self) -> impl Parser<StateCarrier<State, Input>, State, Error>
    where
        Self : Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let state_clone = input.state.clone();
            let (rest, _) = self.parse(input)?;
            Ok((rest, state_clone))
        }
    }

    fn get_current_state(self) -> impl Parser<StateCarrier<State, Input>, State, Error>
    where
        Self : Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let (rest, _) = self.parse(input)?;
            let state_clone = rest.state.clone();
            Ok((rest, state_clone))
        }
    }


    fn get_last_and_current_state(self) -> impl Parser<StateCarrier<State, Input>, (State,State), Error>
    where
        Self : Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            let (rest, _) = self.parse(input)?;
            let current = rest.state.clone();
            Ok((rest, (last,current)))
        }
    }



    fn inject_last_and_current_state(self)->impl Parser<StateCarrier<State, Input>, ((State,State),Output), ((State,State),Error)>
    where
        Self : Sized,
        State: Clone,
    StateCarrier<State, Input> : Parsable<((State,State),Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            match self.parse(input) {
                Ok((StateCarrier{state,input},out)) => Ok((StateCarrier{state:state.clone(),input},((last,state),out))),
                Err((StateCarrier{state,input},out)) => Err((StateCarrier{state:state.clone(),input},((last,state),out))),
            }
        }
    }

    fn inject_last_state(self)->impl Parser<StateCarrier<State, Input>, (State,Output), (State,Error)>
    where
        Self : Sized,
        State: Clone,
    StateCarrier<State, Input> : Parsable<(State,Error)>,
    
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            match self.parse(input) {
                Ok((rest,out)) => Ok((rest,(last,out))),
                Err((rest,out)) => Err((rest,(last,out))),
            }
        }
    }

    fn inject_current_state(self)->impl Parser<StateCarrier<State, Input>, (State,Output), (State,Error)>
    where
        Self : Sized,
        State: Clone,
    StateCarrier<State, Input> : Parsable<(State,Error)>,
    
    {
        move |input: StateCarrier<State, Input>| {
            match self.parse(input) {
                Ok((StateCarrier{state,input},out)) => Ok((StateCarrier{state:state.clone(),input},(state,out))),
                Err((StateCarrier{state,input},out)) => Err((StateCarrier{state:state.clone(),input},(state,out))),
            }
        }
        
    }

    
    fn inject_last_and_current_state_to_output(self) -> impl Parser<StateCarrier<State, Input>, ((State,State),Output), Error>
    where
        Self : Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            let (rest, output) = self.parse(input)?;
            let current = rest.state.clone();
            Ok((rest, ((last,current),output)))
        }
    }

    fn inject_last_state_to_output(self) -> impl Parser<StateCarrier<State, Input>, (State,Output), Error>
    where
        Self : Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let state_clone = input.state.clone();
            let (rest, output) = self.parse(input)?;
            Ok((rest, (state_clone,output)))
        }
    }

    

    fn inject_current_state_to_output(self) -> impl Parser<StateCarrier<State, Input>, (State,Output), Error>
    where
        Self : Sized,
        State: Clone,
    {
        move |input: StateCarrier<State, Input>| {
            let (rest, output) = self.parse(input)?;
            let state_clone = rest.state.clone();
            Ok((rest, (state_clone,output)))
        }
    }


    fn inject_last_state_to_error(self) -> impl Parser<StateCarrier<State, Input>, Output, (State,Error)>
    where
        Self : Sized,
        State: Clone,
    StateCarrier<State, Input> : Parsable<(State,Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            let state_clone = input.state.clone();
            match self.parse(input) {
                Err((StateCarrier { state, input },out)) => { Err((StateCarrier { state, input },(state_clone,out))) },
                Ok(x) => Ok(x),
            }
        }
    }

    fn inject_current_state_to_error(self) -> impl Parser<StateCarrier<State, Input>, Output, (State,Error)>
    where
        Self : Sized,
        State: Clone,
    StateCarrier<State, Input> : Parsable<(State,Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            //let state_clone = input.state.clone();
            match self.parse(input) {
                Err((StateCarrier { state, input },out)) => { Err((StateCarrier { state:state.clone(), input },(state,out))) },
                Ok(x) => Ok(x),
            }
        }
    }

    fn inject_last_and_current_state_to_error(self) -> impl Parser<StateCarrier<State, Input>, Output, ((State,State),Error)>
    where
        Self : Sized,
        State: Clone,
    StateCarrier<State, Input> : Parsable<((State,State),Error)>,
    {
        move |input: StateCarrier<State, Input>| {
            let last = input.state.clone();
            match self.parse(input) {
                Err((StateCarrier { state, input },out)) => { Err((StateCarrier { state:state.clone(), input },((last,state),out))) },
                Ok(x) => Ok(x),
            }
        }
    }


    fn general_bind<OutBind,ErrBind, P2, O2,E2>(self, out_bind:OutBind,err_bind:ErrBind) -> impl StatefulParser<State,Input,O2,E2>
    where
        Self:Sized,
        OutBind: Fn(State,Output) ->  P2 ,
        ErrBind: Fn(State,Error) ->  P2 ,
        P2: StatefulParser<State,Input, O2, E2>,
    Input : Clone +  Parsable<E2> ,
    StateCarrier<State, Input> : Parsable<Error> + Parsable<E2>,
       E2 : Clone,
       State : Clone,
    {
         move |input: StateCarrier<State, Input>| {
             match self.parse(input) {
                 Ok((StateCarrier { state, input },out)) => out_bind(state.clone(),out).parse(StateCarrier { state, input }),
                 Err((StateCarrier{ state, input },err)) => err_bind(state.clone(),err).parse(StateCarrier { state, input }),
             }
        }
    }
    
}


impl<State, Input, Output, Error, P, SuccesT, ErrorT>
    StatefulParser<State, Input, Output, Error>
    for ParserWithStateTransition<State, Input, Output, Error, P, SuccesT, ErrorT>
where
    State:Default,
    Error: Clone,
    Output: ParserOutput,
StateCarrier<State, Input>: Parsable<Error>,
    Input:  Parsable<Error> + Clone,
    P: Parser<Input, Output, Error>,
    SuccesT: FnMut(State, Input, Output, Input) -> (State, Input, Output),
    ErrorT: FnMut(State, Input, Error, Input) -> (State, Input, Error),
{
    fn underlying_parser(self)-> impl Parser<Input,Output,Error> {
        self.parser
    }
}

impl<State,Input, Output, Error, Function> StatefulParser<State,Input, Output, Error> for Function
where
    Function: Fn(StateCarrier<State, Input>) -> Result<(StateCarrier<State, Input>, Output), (StateCarrier<State, Input>, Error)> ,
    Input: Parsable<Error> + Clone,
    StateCarrier<State,Input> : Parsable<Error>,
    Error: Clone,
    State:Default,
    
{
    fn underlying_parser(self)-> impl Parser<Input,Output,Error> {
        move |input|{
            match self(StateCarrier::new(State::default(), input)){
                Ok((StateCarrier { state:_, input },out)) => Ok((input,out)),
                Err((StateCarrier { state:_, input },out)) => Err((input,out)),
            }
        }
    }
}

