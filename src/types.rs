//! # Type Definitions
//!
//! This module defines various type utilities used throughout the parser library,
//! including Either types, natural number types, and type traits for sum and product types.

use core::fmt::Debug;

/// A sum type representing one of two possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either<A, B> {
    /// The left variant.
    Left(A),
    /// The right variant.
    Right(B),
}

impl<A, B> Either<A, B> {
    /// Swaps the left and right variants.
    pub fn swap(self) -> Either<B, A> {
        match self {
            Either::Left(x) => Either::Right(x),
            Either::Right(x) => Either::Left(x),
        }
    }

    /// Maps both variants with separate functions.
    pub fn map<C1, C2, F1, F2>(self, mut f: F1, mut g: F2) -> Either<C1, C2>
    where
        F1: FnMut(A) -> C1,
        F2: FnMut(B) -> C2,
    {
        match self {
            Either::Right(a) => Either::Right(g(a)),
            Either::Left(a) => Either::Left(f(a)),
        }
    }
    
    /// Maps only the right variant, leaving the left variant unchanged.
    pub fn map_right<C, F>(self, mut f: F) -> Either<A, C>
    where
        F: FnMut(B) -> C,
    {
        match self {
            Either::Right(a) => Either::Right(f(a)),
            Either::Left(x) => Either::Left(x),
        }
    }
    
    /// Maps only the left variant, leaving the right variant unchanged.
    pub fn map_left<C, F>(self, mut f: F) -> Either<C, B>
    where
        F: FnMut(A) -> C,
    {
        match self {
            Either::Left(a) => Either::Left(f(a)),
            Either::Right(x) => Either::Right(x),
        }
    }
}

/// A sum type representing one of three possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either3<A1, A2, A3> {
    /// The left variant.
    Left(A1),
    /// The middle variant.
    Middle(A2),
    /// The right variant.
    Right(A3),
}

/// A sum type representing one of four possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either4<A1, A2, A3, A4> {
    /// The first variant.
    _1(A1),
    /// The second variant.
    _2(A2),
    /// The third variant.
    _3(A3),
    /// The fourth variant.
    _4(A4),
}

/// A sum type representing one of five possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either5<A1, A2, A3, A4, A5> {
    /// The first variant.
    _1(A1),
    /// The second variant.
    _2(A2),
    /// The third variant.
    _3(A3),
    /// The fourth variant.
    _4(A4),
    /// The fifth variant.
    _5(A5),
}

/// A sum type representing one of six possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either6<A1, A2, A3, A4, A5, A6> {
    /// The first variant.
    _1(A1),
    /// The second variant.
    _2(A2),
    /// The third variant.
    _3(A3),
    /// The fourth variant.
    _4(A4),
    /// The fifth variant.
    _5(A5),
    /// The sixth variant.
    _6(A6),
}

/// A sum type representing one of seven possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either7<A1, A2, A3, A4, A5, A6, A7> {
    /// The first variant.
    _1(A1),
    /// The second variant.
    _2(A2),
    /// The third variant.
    _3(A3),
    /// The fourth variant.
    _4(A4),
    /// The fifth variant.
    _5(A5),
    /// The sixth variant.
    _6(A6),
    /// The seventh variant.
    _7(A7),
}

/// A sum type representing one of eight possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either8<A1, A2, A3, A4, A5, A6, A7, A8> {
    /// The first variant.
    _1(A1),
    /// The second variant.
    _2(A2),
    /// The third variant.
    _3(A3),
    /// The fourth variant.
    _4(A4),
    /// The fifth variant.
    _5(A5),
    /// The sixth variant.
    _6(A6),
    /// The seventh variant.
    _7(A7),
    /// The eighth variant.
    _8(A8),
}

/// A sum type representing one of nine possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either9<A1, A2, A3, A4, A5, A6, A7, A8, A9> {
    /// The first variant.
    _1(A1),
    /// The second variant.
    _2(A2),
    /// The third variant.
    _3(A3),
    /// The fourth variant.
    _4(A4),
    /// The fifth variant.
    _5(A5),
    /// The sixth variant.
    _6(A6),
    /// The seventh variant.
    _7(A7),
    /// The eighth variant.
    _8(A8),
    /// The ninth variant.
    _9(A9),
}

/// A sum type representing one of ten possible values.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Either10<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> {
    /// The first variant.
    _1(A1),
    /// The second variant.
    _2(A2),
    /// The third variant.
    _3(A3),
    /// The fourth variant.
    _4(A4),
    /// The fifth variant.
    _5(A5),
    /// The sixth variant.
    _6(A6),
    /// The seventh variant.
    _7(A7),
    /// The eighth variant.
    _8(A8),
    /// The ninth variant.
    _9(A9),
    /// The tenth variant.
    _10(A10),
}

/// Placeholder for type-level zero.
pub struct Zero;

/// Placeholder for type-level successor of a type-level natural.
pub struct Succ<X>(X);

/// Trait for type-level natural numbers.
pub trait Natural {}
impl Natural for Zero {}
impl<X: Natural> Natural for Succ<X> {}

/// Common type aliases for type-level naturals.
pub type One = Succ<Zero>;
pub type Two = Succ<One>;
pub type Three = Succ<Two>;

/// Trait for folding nested Either types.
pub trait NFoldable<T> {
    /// The result type of the fold operation.
    type Result;
    
    /// Folds the nested Either structure into a single value.
    fn n_fold(self) -> Self::Result;
}

// Base case implementations
impl<A> NFoldable<(Zero, Zero)> for A {
    type Result = A;
    fn n_fold(self) -> A {
        self
    }
}

impl<A> NFoldable<(Zero, Zero, Zero)> for A {
    type Result = A;
    fn n_fold(self) -> A {
        self
    }
}

// Recursive case for nested Either values
impl<
        C,
        X: Natural,
        Y: Natural,
        A: NFoldable<(X, X), Result = C>,
        B: NFoldable<(Y, Y), Result = C>,
    > NFoldable<(Succ<X>, Succ<Y>)> for Either<A, B>
{
    type Result = C;
    fn n_fold(self) -> Self::Result {
        match self {
            Either::Left(x) => x.n_fold(),
            Either::Right(x) => x.n_fold(),
        }
    }
}

// Recursive case for nested Either3 values
impl<
        D,
        X: Natural,
        Y: Natural,
        Z: Natural,
        A: NFoldable<(X, X, X), Result = D>,
        B: NFoldable<(Y, Y, Y), Result = D>,
        C: NFoldable<(Z, Z, Z), Result = D>,
    > NFoldable<(Succ<X>, Succ<Y>, Succ<Z>)> for Either3<A, B, C>
{
    type Result = D;
    fn n_fold(self) -> Self::Result {
        match self {
            Either3::Left(x) => x.n_fold(),
            Either3::Middle(x) => x.n_fold(),
            Either3::Right(x) => x.n_fold(),
        }
    }
}

/// Trait for types that can be folded to a common result type.
pub trait Foldable {
    /// The result type of the fold operation.
    type Result;
    
    /// Folds the value into a single result.
    fn fold(self) -> Self::Result;
}

// Implementation for Either with the same type in both variants
impl<A> Foldable for Either<A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either::Left(c) | Either::Right(c) => c,
        }
    }
}

/// Marker trait for sum and product types.
pub trait SumAndProdType {}

/// Marker trait for product types.
pub trait ProdType: SumAndProdType {}

/// Marker trait for sum types.
pub trait SumType: SumAndProdType {}

// Implementations for tuple product types
impl ProdType for () {}
impl<T1> ProdType for (T1,) {}
impl<T1, T2> ProdType for (T1, T2) {}
impl<T1, T2, T3> ProdType for (T1, T2, T3) {}
impl<T1, T2, T3, T4> ProdType for (T1, T2, T3, T4) {}
impl<T1, T2, T3, T4, T5> ProdType for (T1, T2, T3, T4, T5) {}
impl<T1, T2, T3, T4, T5, T6> ProdType for (T1, T2, T3, T4, T5, T6) {}
impl<T1, T2, T3, T4, T5, T6, T7> ProdType for (T1, T2, T3, T4, T5, T6, T7) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8> ProdType for (T1, T2, T3, T4, T5, T6, T7, T8) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> ProdType for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> ProdType
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{}

// Implementations for tuple SumAndProdType
impl SumAndProdType for () {}
impl<T1> SumAndProdType for (T1,) {}
impl<T1, T2> SumAndProdType for (T1, T2) {}
impl<T1, T2, T3> SumAndProdType for (T1, T2, T3) {}
impl<T1, T2, T3, T4> SumAndProdType for (T1, T2, T3, T4) {}
impl<T1, T2, T3, T4, T5> SumAndProdType for (T1, T2, T3, T4, T5) {}
impl<T1, T2, T3, T4, T5, T6> SumAndProdType for (T1, T2, T3, T4, T5, T6) {}
impl<T1, T2, T3, T4, T5, T6, T7> SumAndProdType for (T1, T2, T3, T4, T5, T6, T7) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8> SumAndProdType for (T1, T2, T3, T4, T5, T6, T7, T8) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> SumAndProdType for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SumAndProdType
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{}

// Implementations for Empty SumType
impl SumType for () {}
impl<T1> SumType for (T1,) {}

// Implementations for Either SumType
impl<T1, T2> SumType for Either<T1, T2> {}
impl<T1, T2, T3> SumType for Either3<T1, T2, T3> {}
impl<T1, T2, T3, T4> SumType for Either4<T1, T2, T3, T4> {}
impl<T1, T2, T3, T4, T5> SumType for Either5<T1, T2, T3, T4, T5> {}
impl<T1, T2, T3, T4, T5, T6> SumType for Either6<T1, T2, T3, T4, T5, T6> {}
impl<T1, T2, T3, T4, T5, T6, T7> SumType for Either7<T1, T2, T3, T4, T5, T6, T7> {}
impl<T1, T2, T3, T4, T5, T6, T7, T8> SumType for Either8<T1, T2, T3, T4, T5, T6, T7, T8> {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> SumType for Either9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SumType
    for Either10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
{}

// Implementations for Either SumAndProdType
impl<T1, T2> SumAndProdType for Either<T1, T2> {}
impl<T1, T2, T3> SumAndProdType for Either3<T1, T2, T3> {}
impl<T1, T2, T3, T4> SumAndProdType for Either4<T1, T2, T3, T4> {}
impl<T1, T2, T3, T4, T5> SumAndProdType for Either5<T1, T2, T3, T4, T5> {}
impl<T1, T2, T3, T4, T5, T6> SumAndProdType for Either6<T1, T2, T3, T4, T5, T6> {}
impl<T1, T2, T3, T4, T5, T6, T7> SumAndProdType for Either7<T1, T2, T3, T4, T5, T6, T7> {}
impl<T1, T2, T3, T4, T5, T6, T7, T8> SumAndProdType for Either8<T1, T2, T3, T4, T5, T6, T7, T8> {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> SumAndProdType
    for Either9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
{}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SumAndProdType
    for Either10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
{}
    