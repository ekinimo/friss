//! # Parser Combinator Functions
//!
//! This module provides implementations for applicative functions used in parsers.

use crate::core::{ApplicativeFuncArgs, ApplicativeFunc};
use crate::types::ProdType;

// Implement ApplicativeFuncArgs for all product types
impl<T: ProdType> ApplicativeFuncArgs for T {}

// Implement ApplicativeFunc for functions with different arities
impl<T1, T2, Out, Function> ApplicativeFunc<(T1, T2), Out> for Function
where
    Function: Fn(T1, T2) -> Out,
{
    fn apply(self, args: (T1, T2)) -> Out {
        let (t1, t2) = args;
        self(t1, t2)
    }
}

impl<T1, T2, T3, Out, Function> ApplicativeFunc<(T1, T2, T3), Out> for Function
where
    Function: Fn(T1, T2, T3) -> Out,
{
    fn apply(self, args: (T1, T2, T3)) -> Out {
        let (t1, t2, t3) = args;
        self(t1, t2, t3)
    }
}

impl<T1, T2, T3, T4, Out, Function> ApplicativeFunc<(T1, T2, T3, T4), Out> for Function
where
    Function: Fn(T1, T2, T3, T4) -> Out,
{
    fn apply(self, args: (T1, T2, T3, T4)) -> Out {
        let (t1, t2, t3, t4) = args;
        self(t1, t2, t3, t4)
    }
}

impl<T1, T2, T3, T4, T5, Out, Function> ApplicativeFunc<(T1, T2, T3, T4, T5), Out> for Function
where
    Function: Fn(T1, T2, T3, T4, T5) -> Out,
{
    fn apply(self, args: (T1, T2, T3, T4, T5)) -> Out {
        let (t1, t2, t3, t4, t5) = args;
        self(t1, t2, t3, t4, t5)
    }
}

impl<T1, T2, T3, T4, T5, T6, Out, Function> ApplicativeFunc<(T1, T2, T3, T4, T5, T6), Out>
    for Function
where
    Function: Fn(T1, T2, T3, T4, T5, T6) -> Out,
{
    fn apply(self, args: (T1, T2, T3, T4, T5, T6)) -> Out {
        let (t1, t2, t3, t4, t5, t6) = args;
        self(t1, t2, t3, t4, t5, t6)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, Out, Function> ApplicativeFunc<(T1, T2, T3, T4, T5, T6, T7), Out>
    for Function
where
    Function: Fn(T1, T2, T3, T4, T5, T6, T7) -> Out,
{
    fn apply(self, args: (T1, T2, T3, T4, T5, T6, T7)) -> Out {
        let (t1, t2, t3, t4, t5, t6, t7) = args;
        self(t1, t2, t3, t4, t5, t6, t7)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, Out, Function>
    ApplicativeFunc<(T1, T2, T3, T4, T5, T6, T7, T8), Out> for Function
where
    Function: Fn(T1, T2, T3, T4, T5, T6, T7, T8) -> Out,
{
    fn apply(self, args: (T1, T2, T3, T4, T5, T6, T7, T8)) -> Out {
        let (t1, t2, t3, t4, t5, t6, t7, t8) = args;
        self(t1, t2, t3, t4, t5, t6, t7, t8)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, Out, Function>
    ApplicativeFunc<(T1, T2, T3, T4, T5, T6, T7, T8, T9), Out> for Function
where
    Function: Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Out,
{
    fn apply(self, args: (T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Out {
        let (t1, t2, t3, t4, t5, t6, t7, t8, t9) = args;
        self(t1, t2, t3, t4, t5, t6, t7, t8, t9)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, Out, Function>
    ApplicativeFunc<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), Out> for Function
where
    Function: Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Out,
{
    fn apply(self, args: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Out {
        let (t1, t2, t3, t4, t5, t6, t7, t8, t9, t10) = args;
        self(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10)
    }
}
