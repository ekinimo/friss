//! # Type Definitions
//!
//! This module defines various type utilities used throughout the parser library,
//! including Either types, natural number types, and type traits for sum and product types.

use core::fmt::Debug;
use std::marker::PhantomData;

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

impl<A1, A2, A3> Either3<A1, A2, A3> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, F1, F2, F3>(self, mut f1: F1, mut f2: F2, mut f3: F3) -> Either3<B1, B2, B3>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
    {
        match self {
            Either3::Left(a) => Either3::Left(f1(a)),
            Either3::Middle(a) => Either3::Middle(f2(a)),
            Either3::Right(a) => Either3::Right(f3(a)),
        }
    }

    /// Maps only the left variant, leaving others unchanged.
    pub fn map_left<B, F>(self, mut f: F) -> Either3<B, A2, A3>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either3::Left(a) => Either3::Left(f(a)),
            Either3::Middle(a) => Either3::Middle(a),
            Either3::Right(a) => Either3::Right(a),
        }
    }

    /// Maps only the middle variant, leaving others unchanged.
    pub fn map_middle<B, F>(self, mut f: F) -> Either3<A1, B, A3>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either3::Left(a) => Either3::Left(a),
            Either3::Middle(a) => Either3::Middle(f(a)),
            Either3::Right(a) => Either3::Right(a),
        }
    }

    /// Maps only the right variant, leaving others unchanged.
    pub fn map_right<B, F>(self, mut f: F) -> Either3<A1, A2, B>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either3::Left(a) => Either3::Left(a),
            Either3::Middle(a) => Either3::Middle(a),
            Either3::Right(a) => Either3::Right(f(a)),
        }
    }
}

impl<A1, A2, A3, A4> Either4<A1, A2, A3, A4> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, B4, F1, F2, F3, F4>(
        self,
        mut f1: F1,
        mut f2: F2,
        mut f3: F3,
        mut f4: F4,
    ) -> Either4<B1, B2, B3, B4>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
        F4: FnMut(A4) -> B4,
    {
        match self {
            Either4::_1(a) => Either4::_1(f1(a)),
            Either4::_2(a) => Either4::_2(f2(a)),
            Either4::_3(a) => Either4::_3(f3(a)),
            Either4::_4(a) => Either4::_4(f4(a)),
        }
    }

    /// Maps only the first variant, leaving others unchanged.
    pub fn map_1<B, F>(self, mut f: F) -> Either4<B, A2, A3, A4>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either4::_1(a) => Either4::_1(f(a)),
            Either4::_2(a) => Either4::_2(a),
            Either4::_3(a) => Either4::_3(a),
            Either4::_4(a) => Either4::_4(a),
        }
    }

    /// Maps only the second variant, leaving others unchanged.
    pub fn map_2<B, F>(self, mut f: F) -> Either4<A1, B, A3, A4>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either4::_1(a) => Either4::_1(a),
            Either4::_2(a) => Either4::_2(f(a)),
            Either4::_3(a) => Either4::_3(a),
            Either4::_4(a) => Either4::_4(a),
        }
    }

    /// Maps only the third variant, leaving others unchanged.
    pub fn map_3<B, F>(self, mut f: F) -> Either4<A1, A2, B, A4>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either4::_1(a) => Either4::_1(a),
            Either4::_2(a) => Either4::_2(a),
            Either4::_3(a) => Either4::_3(f(a)),
            Either4::_4(a) => Either4::_4(a),
        }
    }

    /// Maps only the fourth variant, leaving others unchanged.
    pub fn map_4<B, F>(self, mut f: F) -> Either4<A1, A2, A3, B>
    where
        F: FnMut(A4) -> B,
    {
        match self {
            Either4::_1(a) => Either4::_1(a),
            Either4::_2(a) => Either4::_2(a),
            Either4::_3(a) => Either4::_3(a),
            Either4::_4(a) => Either4::_4(f(a)),
        }
    }
}

impl<A1, A2, A3, A4, A5> Either5<A1, A2, A3, A4, A5> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, B4, B5, F1, F2, F3, F4, F5>(
        self,
        mut f1: F1,
        mut f2: F2,
        mut f3: F3,
        mut f4: F4,
        mut f5: F5,
    ) -> Either5<B1, B2, B3, B4, B5>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
        F4: FnMut(A4) -> B4,
        F5: FnMut(A5) -> B5,
    {
        match self {
            Either5::_1(a) => Either5::_1(f1(a)),
            Either5::_2(a) => Either5::_2(f2(a)),
            Either5::_3(a) => Either5::_3(f3(a)),
            Either5::_4(a) => Either5::_4(f4(a)),
            Either5::_5(a) => Either5::_5(f5(a)),
        }
    }

    /// Maps only the specified variant, leaving others unchanged.
    pub fn map_1<B, F>(self, mut f: F) -> Either5<B, A2, A3, A4, A5>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either5::_1(a) => Either5::_1(f(a)),
            Either5::_2(a) => Either5::_2(a),
            Either5::_3(a) => Either5::_3(a),
            Either5::_4(a) => Either5::_4(a),
            Either5::_5(a) => Either5::_5(a),
        }
    }

    pub fn map_2<B, F>(self, mut f: F) -> Either5<A1, B, A3, A4, A5>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either5::_1(a) => Either5::_1(a),
            Either5::_2(a) => Either5::_2(f(a)),
            Either5::_3(a) => Either5::_3(a),
            Either5::_4(a) => Either5::_4(a),
            Either5::_5(a) => Either5::_5(a),
        }
    }

    pub fn map_3<B, F>(self, mut f: F) -> Either5<A1, A2, B, A4, A5>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either5::_1(a) => Either5::_1(a),
            Either5::_2(a) => Either5::_2(a),
            Either5::_3(a) => Either5::_3(f(a)),
            Either5::_4(a) => Either5::_4(a),
            Either5::_5(a) => Either5::_5(a),
        }
    }

    pub fn map_4<B, F>(self, mut f: F) -> Either5<A1, A2, A3, B, A5>
    where
        F: FnMut(A4) -> B,
    {
        match self {
            Either5::_1(a) => Either5::_1(a),
            Either5::_2(a) => Either5::_2(a),
            Either5::_3(a) => Either5::_3(a),
            Either5::_4(a) => Either5::_4(f(a)),
            Either5::_5(a) => Either5::_5(a),
        }
    }

    pub fn map_5<B, F>(self, mut f: F) -> Either5<A1, A2, A3, A4, B>
    where
        F: FnMut(A5) -> B,
    {
        match self {
            Either5::_1(a) => Either5::_1(a),
            Either5::_2(a) => Either5::_2(a),
            Either5::_3(a) => Either5::_3(a),
            Either5::_4(a) => Either5::_4(a),
            Either5::_5(a) => Either5::_5(f(a)),
        }
    }
}

impl<A1, A2, A3, A4, A5, A6> Either6<A1, A2, A3, A4, A5, A6> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, B4, B5, B6, F1, F2, F3, F4, F5, F6>(
        self,
        mut f1: F1,
        mut f2: F2,
        mut f3: F3,
        mut f4: F4,
        mut f5: F5,
        mut f6: F6,
    ) -> Either6<B1, B2, B3, B4, B5, B6>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
        F4: FnMut(A4) -> B4,
        F5: FnMut(A5) -> B5,
        F6: FnMut(A6) -> B6,
    {
        match self {
            Either6::_1(a) => Either6::_1(f1(a)),
            Either6::_2(a) => Either6::_2(f2(a)),
            Either6::_3(a) => Either6::_3(f3(a)),
            Either6::_4(a) => Either6::_4(f4(a)),
            Either6::_5(a) => Either6::_5(f5(a)),
            Either6::_6(a) => Either6::_6(f6(a)),
        }
    }

    /// Maps only the first variant, leaving others unchanged.
    pub fn map_1<B, F>(self, mut f: F) -> Either6<B, A2, A3, A4, A5, A6>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either6::_1(a) => Either6::_1(f(a)),
            Either6::_2(a) => Either6::_2(a),
            Either6::_3(a) => Either6::_3(a),
            Either6::_4(a) => Either6::_4(a),
            Either6::_5(a) => Either6::_5(a),
            Either6::_6(a) => Either6::_6(a),
        }
    }

    pub fn map_2<B, F>(self, mut f: F) -> Either6<A1, B, A3, A4, A5, A6>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either6::_1(a) => Either6::_1(a),
            Either6::_2(a) => Either6::_2(f(a)),
            Either6::_3(a) => Either6::_3(a),
            Either6::_4(a) => Either6::_4(a),
            Either6::_5(a) => Either6::_5(a),
            Either6::_6(a) => Either6::_6(a),
        }
    }

    pub fn map_3<B, F>(self, mut f: F) -> Either6<A1, A2, B, A4, A5, A6>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either6::_1(a) => Either6::_1(a),
            Either6::_2(a) => Either6::_2(a),
            Either6::_3(a) => Either6::_3(f(a)),
            Either6::_4(a) => Either6::_4(a),
            Either6::_5(a) => Either6::_5(a),
            Either6::_6(a) => Either6::_6(a),
        }
    }

    pub fn map_4<B, F>(self, mut f: F) -> Either6<A1, A2, A3, B, A5, A6>
    where
        F: FnMut(A4) -> B,
    {
        match self {
            Either6::_1(a) => Either6::_1(a),
            Either6::_2(a) => Either6::_2(a),
            Either6::_3(a) => Either6::_3(a),
            Either6::_4(a) => Either6::_4(f(a)),
            Either6::_5(a) => Either6::_5(a),
            Either6::_6(a) => Either6::_6(a),
        }
    }

    pub fn map_5<B, F>(self, mut f: F) -> Either6<A1, A2, A3, A4, B, A6>
    where
        F: FnMut(A5) -> B,
    {
        match self {
            Either6::_1(a) => Either6::_1(a),
            Either6::_2(a) => Either6::_2(a),
            Either6::_3(a) => Either6::_3(a),
            Either6::_4(a) => Either6::_4(a),
            Either6::_5(a) => Either6::_5(f(a)),
            Either6::_6(a) => Either6::_6(a),
        }
    }

    pub fn map_6<B, F>(self, mut f: F) -> Either6<A1, A2, A3, A4, A5, B>
    where
        F: FnMut(A6) -> B,
    {
        match self {
            Either6::_1(a) => Either6::_1(a),
            Either6::_2(a) => Either6::_2(a),
            Either6::_3(a) => Either6::_3(a),
            Either6::_4(a) => Either6::_4(a),
            Either6::_5(a) => Either6::_5(a),
            Either6::_6(a) => Either6::_6(f(a)),
        }
    }
}

// Implementation for Either7
impl<A1, A2, A3, A4, A5, A6, A7> Either7<A1, A2, A3, A4, A5, A6, A7> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, B4, B5, B6, B7, F1, F2, F3, F4, F5, F6, F7>(
        self,
        mut f1: F1,
        mut f2: F2,
        mut f3: F3,
        mut f4: F4,
        mut f5: F5,
        mut f6: F6,
        mut f7: F7,
    ) -> Either7<B1, B2, B3, B4, B5, B6, B7>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
        F4: FnMut(A4) -> B4,
        F5: FnMut(A5) -> B5,
        F6: FnMut(A6) -> B6,
        F7: FnMut(A7) -> B7,
    {
        match self {
            Either7::_1(a) => Either7::_1(f1(a)),
            Either7::_2(a) => Either7::_2(f2(a)),
            Either7::_3(a) => Either7::_3(f3(a)),
            Either7::_4(a) => Either7::_4(f4(a)),
            Either7::_5(a) => Either7::_5(f5(a)),
            Either7::_6(a) => Either7::_6(f6(a)),
            Either7::_7(a) => Either7::_7(f7(a)),
        }
    }

    /// Maps only the first variant, leaving others unchanged.
    pub fn map_1<B, F>(self, mut f: F) -> Either7<B, A2, A3, A4, A5, A6, A7>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either7::_1(a) => Either7::_1(f(a)),
            Either7::_2(a) => Either7::_2(a),
            Either7::_3(a) => Either7::_3(a),
            Either7::_4(a) => Either7::_4(a),
            Either7::_5(a) => Either7::_5(a),
            Either7::_6(a) => Either7::_6(a),
            Either7::_7(a) => Either7::_7(a),
        }
    }

    pub fn map_2<B, F>(self, mut f: F) -> Either7<A1, B, A3, A4, A5, A6, A7>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either7::_1(a) => Either7::_1(a),
            Either7::_2(a) => Either7::_2(f(a)),
            Either7::_3(a) => Either7::_3(a),
            Either7::_4(a) => Either7::_4(a),
            Either7::_5(a) => Either7::_5(a),
            Either7::_6(a) => Either7::_6(a),
            Either7::_7(a) => Either7::_7(a),
        }
    }

    pub fn map_3<B, F>(self, mut f: F) -> Either7<A1, A2, B, A4, A5, A6, A7>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either7::_1(a) => Either7::_1(a),
            Either7::_2(a) => Either7::_2(a),
            Either7::_3(a) => Either7::_3(f(a)),
            Either7::_4(a) => Either7::_4(a),
            Either7::_5(a) => Either7::_5(a),
            Either7::_6(a) => Either7::_6(a),
            Either7::_7(a) => Either7::_7(a),
        }
    }

    pub fn map_4<B, F>(self, mut f: F) -> Either7<A1, A2, A3, B, A5, A6, A7>
    where
        F: FnMut(A4) -> B,
    {
        match self {
            Either7::_1(a) => Either7::_1(a),
            Either7::_2(a) => Either7::_2(a),
            Either7::_3(a) => Either7::_3(a),
            Either7::_4(a) => Either7::_4(f(a)),
            Either7::_5(a) => Either7::_5(a),
            Either7::_6(a) => Either7::_6(a),
            Either7::_7(a) => Either7::_7(a),
        }
    }

    pub fn map_5<B, F>(self, mut f: F) -> Either7<A1, A2, A3, A4, B, A6, A7>
    where
        F: FnMut(A5) -> B,
    {
        match self {
            Either7::_1(a) => Either7::_1(a),
            Either7::_2(a) => Either7::_2(a),
            Either7::_3(a) => Either7::_3(a),
            Either7::_4(a) => Either7::_4(a),
            Either7::_5(a) => Either7::_5(f(a)),
            Either7::_6(a) => Either7::_6(a),
            Either7::_7(a) => Either7::_7(a),
        }
    }

    pub fn map_6<B, F>(self, mut f: F) -> Either7<A1, A2, A3, A4, A5, B, A7>
    where
        F: FnMut(A6) -> B,
    {
        match self {
            Either7::_1(a) => Either7::_1(a),
            Either7::_2(a) => Either7::_2(a),
            Either7::_3(a) => Either7::_3(a),
            Either7::_4(a) => Either7::_4(a),
            Either7::_5(a) => Either7::_5(a),
            Either7::_6(a) => Either7::_6(f(a)),
            Either7::_7(a) => Either7::_7(a),
        }
    }

    pub fn map_7<B, F>(self, mut f: F) -> Either7<A1, A2, A3, A4, A5, A6, B>
    where
        F: FnMut(A7) -> B,
    {
        match self {
            Either7::_1(a) => Either7::_1(a),
            Either7::_2(a) => Either7::_2(a),
            Either7::_3(a) => Either7::_3(a),
            Either7::_4(a) => Either7::_4(a),
            Either7::_5(a) => Either7::_5(a),
            Either7::_6(a) => Either7::_6(a),
            Either7::_7(a) => Either7::_7(f(a)),
        }
    }
}

impl<A1, A2, A3, A4, A5, A6, A7, A8> Either8<A1, A2, A3, A4, A5, A6, A7, A8> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, B4, B5, B6, B7, B8, F1, F2, F3, F4, F5, F6, F7, F8>(
        self,
        mut f1: F1,
        mut f2: F2,
        mut f3: F3,
        mut f4: F4,
        mut f5: F5,
        mut f6: F6,
        mut f7: F7,
        mut f8: F8,
    ) -> Either8<B1, B2, B3, B4, B5, B6, B7, B8>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
        F4: FnMut(A4) -> B4,
        F5: FnMut(A5) -> B5,
        F6: FnMut(A6) -> B6,
        F7: FnMut(A7) -> B7,
        F8: FnMut(A8) -> B8,
    {
        match self {
            Either8::_1(a) => Either8::_1(f1(a)),
            Either8::_2(a) => Either8::_2(f2(a)),
            Either8::_3(a) => Either8::_3(f3(a)),
            Either8::_4(a) => Either8::_4(f4(a)),
            Either8::_5(a) => Either8::_5(f5(a)),
            Either8::_6(a) => Either8::_6(f6(a)),
            Either8::_7(a) => Either8::_7(f7(a)),
            Either8::_8(a) => Either8::_8(f8(a)),
        }
    }

    /// Maps only the first variant, leaving others unchanged.
    pub fn map_1<B, F>(self, mut f: F) -> Either8<B, A2, A3, A4, A5, A6, A7, A8>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(f(a)),
            Either8::_2(a) => Either8::_2(a),
            Either8::_3(a) => Either8::_3(a),
            Either8::_4(a) => Either8::_4(a),
            Either8::_5(a) => Either8::_5(a),
            Either8::_6(a) => Either8::_6(a),
            Either8::_7(a) => Either8::_7(a),
            Either8::_8(a) => Either8::_8(a),
        }
    }

    pub fn map_2<B, F>(self, mut f: F) -> Either8<A1, B, A3, A4, A5, A6, A7, A8>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(a),
            Either8::_2(a) => Either8::_2(f(a)),
            Either8::_3(a) => Either8::_3(a),
            Either8::_4(a) => Either8::_4(a),
            Either8::_5(a) => Either8::_5(a),
            Either8::_6(a) => Either8::_6(a),
            Either8::_7(a) => Either8::_7(a),
            Either8::_8(a) => Either8::_8(a),
        }
    }

    pub fn map_3<B, F>(self, mut f: F) -> Either8<A1, A2, B, A4, A5, A6, A7, A8>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(a),
            Either8::_2(a) => Either8::_2(a),
            Either8::_3(a) => Either8::_3(f(a)),
            Either8::_4(a) => Either8::_4(a),
            Either8::_5(a) => Either8::_5(a),
            Either8::_6(a) => Either8::_6(a),
            Either8::_7(a) => Either8::_7(a),
            Either8::_8(a) => Either8::_8(a),
        }
    }

    pub fn map_4<B, F>(self, mut f: F) -> Either8<A1, A2, A3, B, A5, A6, A7, A8>
    where
        F: FnMut(A4) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(a),
            Either8::_2(a) => Either8::_2(a),
            Either8::_3(a) => Either8::_3(a),
            Either8::_4(a) => Either8::_4(f(a)),
            Either8::_5(a) => Either8::_5(a),
            Either8::_6(a) => Either8::_6(a),
            Either8::_7(a) => Either8::_7(a),
            Either8::_8(a) => Either8::_8(a),
        }
    }

    pub fn map_5<B, F>(self, mut f: F) -> Either8<A1, A2, A3, A4, B, A6, A7, A8>
    where
        F: FnMut(A5) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(a),
            Either8::_2(a) => Either8::_2(a),
            Either8::_3(a) => Either8::_3(a),
            Either8::_4(a) => Either8::_4(a),
            Either8::_5(a) => Either8::_5(f(a)),
            Either8::_6(a) => Either8::_6(a),
            Either8::_7(a) => Either8::_7(a),
            Either8::_8(a) => Either8::_8(a),
        }
    }

    pub fn map_6<B, F>(self, mut f: F) -> Either8<A1, A2, A3, A4, A5, B, A7, A8>
    where
        F: FnMut(A6) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(a),
            Either8::_2(a) => Either8::_2(a),
            Either8::_3(a) => Either8::_3(a),
            Either8::_4(a) => Either8::_4(a),
            Either8::_5(a) => Either8::_5(a),
            Either8::_6(a) => Either8::_6(f(a)),
            Either8::_7(a) => Either8::_7(a),
            Either8::_8(a) => Either8::_8(a),
        }
    }

    pub fn map_7<B, F>(self, mut f: F) -> Either8<A1, A2, A3, A4, A5, A6, B, A8>
    where
        F: FnMut(A7) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(a),
            Either8::_2(a) => Either8::_2(a),
            Either8::_3(a) => Either8::_3(a),
            Either8::_4(a) => Either8::_4(a),
            Either8::_5(a) => Either8::_5(a),
            Either8::_6(a) => Either8::_6(a),
            Either8::_7(a) => Either8::_7(f(a)),
            Either8::_8(a) => Either8::_8(a),
        }
    }

    pub fn map_8<B, F>(self, mut f: F) -> Either8<A1, A2, A3, A4, A5, A6, A7, B>
    where
        F: FnMut(A8) -> B,
    {
        match self {
            Either8::_1(a) => Either8::_1(a),
            Either8::_2(a) => Either8::_2(a),
            Either8::_3(a) => Either8::_3(a),
            Either8::_4(a) => Either8::_4(a),
            Either8::_5(a) => Either8::_5(a),
            Either8::_6(a) => Either8::_6(a),
            Either8::_7(a) => Either8::_7(a),
            Either8::_8(a) => Either8::_8(f(a)),
        }
    }
}


impl<A1, A2, A3, A4, A5, A6, A7, A8,A9> Either9<A1, A2, A3, A4, A5, A6, A7, A8,A9> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, B4, B5, B6, B7, B8,B9, F1, F2, F3, F4, F5, F6, F7, F8,F9>(
        self,
        mut f1: F1,
        mut f2: F2,
        mut f3: F3,
        mut f4: F4,
        mut f5: F5,
        mut f6: F6,
        mut f7: F7,
        mut f8: F8,
        mut f9: F9,
    ) -> Either9<B1, B2, B3, B4, B5, B6, B7, B8,B9>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
        F4: FnMut(A4) -> B4,
        F5: FnMut(A5) -> B5,
        F6: FnMut(A6) -> B6,
        F7: FnMut(A7) -> B7,
        F8: FnMut(A8) -> B8,
        F9: FnMut(A9) -> B9,
    {
        match self {
            Either9::_1(a) => Either9::_1(f1(a)),
            Either9::_2(a) => Either9::_2(f2(a)),
            Either9::_3(a) => Either9::_3(f3(a)),
            Either9::_4(a) => Either9::_4(f4(a)),
            Either9::_5(a) => Either9::_5(f5(a)),
            Either9::_6(a) => Either9::_6(f6(a)),
            Either9::_7(a) => Either9::_7(f7(a)),
            Either9::_8(a) => Either9::_8(f8(a)),
            Either9::_9(a) => Either9::_9(f9(a)),
        }
    }

    /// Maps only the first variant, leaving others unchanged.
    pub fn map_1<B, F>(self, mut f: F) -> Either9<B, A2, A3, A4, A5, A6, A7, A8,A9>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(f(a)),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_2<B, F>(self, mut f: F) -> Either9<A1, B, A3, A4, A5, A6, A7, A8,A9>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(f(a)),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_3<B, F>(self, mut f: F) -> Either9<A1, A2, B, A4, A5, A6, A7, A8,A9>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(f(a)),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_4<B, F>(self, mut f: F) -> Either9<A1, A2, A3, B, A5, A6, A7, A8,A9>
    where
        F: FnMut(A4) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(f(a)),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_5<B, F>(self, mut f: F) -> Either9<A1, A2, A3, A4, B, A6, A7, A8,A9>
    where
        F: FnMut(A5) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(f(a)),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_6<B, F>(self, mut f: F) -> Either9<A1, A2, A3, A4, A5, B, A7, A8,A9>
    where
        F: FnMut(A6) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(f(a)),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_7<B, F>(self, mut f: F) -> Either9<A1, A2, A3, A4, A5, A6, B, A8,A9>
    where
        F: FnMut(A7) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(f(a)),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_8<B, F>(self, mut f: F) -> Either9<A1, A2, A3, A4, A5, A6, A7, B,A9>
    where
        F: FnMut(A8) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(f(a)),
            Either9::_9(a) => Either9::_9(a),
        }
    }

    pub fn map_9<B, F>(self, mut f: F) -> Either9<A1, A2, A3, A4, A5, A6, A7, A8,B>
    where
        F: FnMut(A9) -> B,
    {
        match self {
            Either9::_1(a) => Either9::_1(a),
            Either9::_2(a) => Either9::_2(a),
            Either9::_3(a) => Either9::_3(a),
            Either9::_4(a) => Either9::_4(a),
            Either9::_5(a) => Either9::_5(a),
            Either9::_6(a) => Either9::_6(a),
            Either9::_7(a) => Either9::_7(a),
            Either9::_8(a) => Either9::_8(a),
            Either9::_9(a) => Either9::_9(f(a)),
        }
    }
}

impl<A1, A2, A3, A4, A5, A6, A7, A8,A9,A10> Either10<A1, A2, A3, A4, A5, A6, A7, A8,A9,A10> {
    /// Maps all variants with separate functions.
    pub fn map<B1, B2, B3, B4, B5, B6, B7, B8,B9,B10, F1, F2, F3, F4, F5, F6, F7, F8,F9,F10>(
        self,
        mut f1: F1,
        mut f2: F2,
        mut f3: F3,
        mut f4: F4,
        mut f5: F5,
        mut f6: F6,
        mut f7: F7,
        mut f8: F8,
        mut f9: F9,
        mut f10: F10,
    ) -> Either10<B1, B2, B3, B4, B5, B6, B7, B8,B9,B10>
    where
        F1: FnMut(A1) -> B1,
        F2: FnMut(A2) -> B2,
        F3: FnMut(A3) -> B3,
        F4: FnMut(A4) -> B4,
        F5: FnMut(A5) -> B5,
        F6: FnMut(A6) -> B6,
        F7: FnMut(A7) -> B7,
        F8: FnMut(A8) -> B8,
        F9: FnMut(A9) -> B9,
        F10: FnMut(A10) -> B10,
    {
        match self {
            Either10::_1(a) => Either10::_1(f1(a)),
            Either10::_2(a) => Either10::_2(f2(a)),
            Either10::_3(a) => Either10::_3(f3(a)),
            Either10::_4(a) => Either10::_4(f4(a)),
            Either10::_5(a) => Either10::_5(f5(a)),
            Either10::_6(a) => Either10::_6(f6(a)),
            Either10::_7(a) => Either10::_7(f7(a)),
            Either10::_8(a) => Either10::_8(f8(a)),
            Either10::_9(a) => Either10::_9(f9(a)),
            Either10::_10(a) => Either10::_10(f10(a)),
        }
    }

    /// Maps only the first variant, leaving others unchanged.
    pub fn map_1<B, F>(self, mut f: F) -> Either10<B, A2, A3, A4, A5, A6, A7, A8,A9,A10>
    where
        F: FnMut(A1) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(f(a)),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
            
        }
    }

    pub fn map_2<B, F>(self, mut f: F) -> Either10<A1, B, A3, A4, A5, A6, A7, A8,A9, A10>
    where
        F: FnMut(A2) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(f(a)),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
        }
    }

    pub fn map_3<B, F>(self, mut f: F) -> Either10<A1, A2, B, A4, A5, A6, A7, A8,A9, A10>
    where
        F: FnMut(A3) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(f(a)),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
        }
    }

    pub fn map_4<B, F>(self, mut f: F) -> Either10<A1, A2, A3, B, A5, A6, A7, A8,A9, A10>
    where
        F: FnMut(A4) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(f(a)),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
        }
    }

    pub fn map_5<B, F>(self, mut f: F) -> Either10<A1, A2, A3, A4, B, A6, A7, A8,A9, A10>
    where
        F: FnMut(A5) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(f(a)),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
        }
    }

    pub fn map_6<B, F>(self, mut f: F) -> Either10<A1, A2, A3, A4, A5, B, A7, A8,A9, A10>
    where
        F: FnMut(A6) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(f(a)),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
        }
    }

    pub fn map_7<B, F>(self, mut f: F) -> Either10<A1, A2, A3, A4, A5, A6, B, A8,A9, A10>
    where
        F: FnMut(A7) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(f(a)),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
        }
    }

    pub fn map_8<B, F>(self, mut f: F) -> Either10<A1, A2, A3, A4, A5, A6, A7, B,A9, A10>
    where
        F: FnMut(A8) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(f(a)),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(a),
        }
    }

    pub fn map_9<B, F>(self, mut f: F) -> Either10<A1, A2, A3, A4, A5, A6, A7, A8,B,A10>
    where
        F: FnMut(A9) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(f(a)),
            Either10::_10(a) => Either10::_10(a),
        }
    }
    pub fn map_10<B, F>(self, mut f: F) -> Either10<A1, A2, A3, A4, A5, A6, A7, A8,A9,B>
    where
        F: FnMut(A10) -> B,
    {
        match self {
            Either10::_1(a) => Either10::_1(a),
            Either10::_2(a) => Either10::_2(a),
            Either10::_3(a) => Either10::_3(a),
            Either10::_4(a) => Either10::_4(a),
            Either10::_5(a) => Either10::_5(a),
            Either10::_6(a) => Either10::_6(a),
            Either10::_7(a) => Either10::_7(a),
            Either10::_8(a) => Either10::_8(a),
            Either10::_9(a) => Either10::_9(a),
            Either10::_10(a) => Either10::_10(f(a)),
        }
    }
}




impl<A> Foldable for Either3<A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either3::Left(a) | Either3::Middle(a) | Either3::Right(a) => a,
        }
    }
}

impl<A> Foldable for Either4<A, A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either4::_1(a) | Either4::_2(a) | Either4::_3(a) | Either4::_4(a) => a,
        }
    }
}

impl<A> Foldable for Either5<A, A, A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either5::_1(a) | Either5::_2(a) | Either5::_3(a) | Either5::_4(a) | Either5::_5(a) => a,
        }
    }
}

impl<A> Foldable for Either6<A, A, A, A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either6::_1(a) | Either6::_2(a) | Either6::_3(a) | 
            Either6::_4(a) | Either6::_5(a) | Either6::_6(a) => a,
        }
    }
}

impl<A> Foldable for Either7<A, A, A, A, A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either7::_1(a) | Either7::_2(a) | Either7::_3(a) | Either7::_4(a) |
            Either7::_5(a) | Either7::_6(a) | Either7::_7(a) => a,
        }
    }
}

impl<A> Foldable for Either8<A, A, A, A, A, A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either8::_1(a) | Either8::_2(a) | Either8::_3(a) | Either8::_4(a) |
            Either8::_5(a) | Either8::_6(a) | Either8::_7(a) | Either8::_8(a) => a,
        }
    }
}

impl<A> Foldable for Either9<A, A, A, A, A, A, A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either9::_1(a) | Either9::_2(a) | Either9::_3(a) | Either9::_4(a) |
            Either9::_5(a) | Either9::_6(a) | Either9::_7(a) | Either9::_8(a) |
            Either9::_9(a) => a,
        }
    }
}

impl<A> Foldable for Either10<A, A, A, A, A, A, A, A, A, A> {
    type Result = A;
    fn fold(self) -> A {
        match self {
            Either10::_1(a) | Either10::_2(a) | Either10::_3(a) | Either10::_4(a) |
            Either10::_5(a) | Either10::_6(a) | Either10::_7(a) | Either10::_8(a) |
            Either10::_9(a) | Either10::_10(a) => a,
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

/// Trait for deep folding of nested Either types.
///
/// This trait allows for recursively collapsing nested Either types to a common result type.
/// It uses the Foldable trait as a base case for non-nested types.
pub trait DeepFoldable<N> {
    /// The result type of the deep fold operation.
    type Result;

    /// Recursively folds the value into a single result.
    fn deep_fold(self) -> Self::Result;
}

/// Type-level Peano numbers for tracking recursion depth
pub struct Zero;
pub struct Succ<N>(std::marker::PhantomData<N>);


impl<A> DeepFoldable<(Zero, Zero)> for Either<A, A>
{
    type Result = A;
    
    fn deep_fold(self) -> Self::Result {
        self.fold()
    }
}

impl<A, B, N> DeepFoldable<(Succ<N>, Zero)> for Either<A, B>
where
    A: DeepFoldable<(N, Zero),Result = B>,
{
    type Result = A::Result;
    
    fn deep_fold(self) -> Self::Result {
        match self {
            Either::Left(a) => a.deep_fold(),
            Either::Right(b) => b,
        }
    }
}

impl<A, B, N> DeepFoldable< (Zero,Succ<N>)> for Either<A, B>
where
    B: DeepFoldable<(Zero, N),Result = A>,
{
    type Result = B::Result;
    
    fn deep_fold(self) -> Self::Result {
        match self {
            Either::Right(a) => a.deep_fold(),
            Either::Left(b) => b,
        }
    }
}

impl<T,A, B, N,M> DeepFoldable< (Succ<N>,Succ<M> )> for Either<A, B>
where
    B: DeepFoldable<(N, M),Result = T>,
    A: DeepFoldable<(N, M),Result = T>,
{
    type Result = B::Result;
    
    fn deep_fold(self) -> Self::Result {
        match self {
            Either::Right(a) => a.deep_fold(),
            Either::Left(b) => b.deep_fold(),
        }
    }
}


impl<A> DeepFoldable<(Zero, Zero,Zero)> for Either3<A, A,A>
{
    type Result = A;
    
    fn deep_fold(self) -> Self::Result {
        self.fold()
    }
}

impl<A, B, N> DeepFoldable<(Succ<N>, Zero,Zero)> for Either3<A, B,B>
where
    A: DeepFoldable<(N, Zero,Zero),Result = B>,
{
    type Result = A::Result;
    
    fn deep_fold(self) -> Self::Result {
        match self {
            Either3::Left(a) => a.deep_fold(),
            Either3::Middle(b) => b,
            Either3::Right(b) => b,
        }
    }
}

impl<A, B, N> DeepFoldable<(Zero,Succ<N>,Zero)> for Either3<B,A,B>
where
    A: DeepFoldable<(Zero,N,Zero),Result = B>,
{
    type Result = A::Result;
    
    fn deep_fold(self) -> Self::Result {
        match self {
            Either3::Left(b) => b,
            Either3::Middle(a) => a.deep_fold(),
            Either3::Right(b) => b,
        }
    }
}



impl<A, B, N> DeepFoldable<(Zero,Zero,Succ<N>)> for Either3<B,B,A>
where
    A: DeepFoldable<(Zero,Zero,N),Result = B>,
{
    type Result = A::Result;
    
    fn deep_fold(self) -> Self::Result {
        match self {
            Either3::Left(b) => b,
            Either3::Middle(b) => b,
            Either3::Right(a) => a.deep_fold(),
        }
    }
}

impl<T,A, B,C, N,M,L> DeepFoldable< (Succ<N>,Succ<M>,Succ<L> )> for Either3<A, B,C>
where
    B: DeepFoldable<(N, M,L),Result = T>,
    A: DeepFoldable<(N, M,L),Result = T>,
    C: DeepFoldable<(N, M,L),Result = T>,
{
    type Result = B::Result;
    
    fn deep_fold(self) -> Self::Result {
        match self {
            Either3::Left(a) => a.deep_fold(),
            Either3::Middle(b) => b.deep_fold(),
            Either3::Right(c) => c.deep_fold(),
        }
    }
}


//TODO find a way to generalize DeepFoldable 

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
{
}

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
{
}

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
{
}

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
{
}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SumAndProdType
    for Either10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
{
}
