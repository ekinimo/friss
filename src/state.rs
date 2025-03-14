use std::{cell::RefCell, marker::PhantomData};

use crate::{core::ParserOutput, Parsable, Parser};




pub struct StateCarrier<State,Input>{
    pub state:State,
    pub input:Input,
}



pub struct TransitionParser<S,I,O,E,P,SuccesT,ErrorT>{
    parser:P,
    on_succes:Option<RefCell<SuccesT>>,
    on_error:Option<RefCell<ErrorT>>,
    _phantom:PhantomData<(I,O,S,E)>,
}


impl <State,Input,Output,Error,P,SuccesT,ErrorT> Parser<StateCarrier<State,Input>,Output,Error> for TransitionParser< State,Input,Output,Error,P,SuccesT,ErrorT>
    where
    Error: Clone,
    Output: ParserOutput,
    StateCarrier<State,Input> : Parsable<Error>,
    Input : Parsable<Error> + Clone,
    P : Parser<Input,Output,Error>,
    SuccesT:FnMut(State,Input,Output,Input)->(State,Input,Output),
    ErrorT:FnMut(State,Input,Error,Input)->(State,Input,Error),
{
    fn parse(&self, StateCarrier { state, input }: StateCarrier<State,Input>) -> Result<(StateCarrier<State,Input>, Output), (StateCarrier<State,Input>, Error)> {
        let input_ref = input.clone();
        match self.parser.parse(input) {
            Ok((input,output)) => {
                match &self.on_succes {
                    Some(f) => {
                        let (state,input,output) = (f.borrow_mut())(state,input,output,input_ref);
                        Ok((StateCarrier{state,input},output))
                    }
                    None => Ok((StateCarrier{state,input},output)),
                }
            }
            Err((input,error)) => {
                match &self.on_error 
                    {
                        Some(f) => {
                            let (state,input,error) = (f.borrow_mut())(state,input,error,input_ref);
                            Err((StateCarrier{state,input},error))
                        }
                        None => Err((StateCarrier{state,input},error)),
                    }
                
            },
        }
        
    }
}




