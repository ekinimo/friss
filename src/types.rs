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



// Macro to define Either types
macro_rules! define_either {
    ($($name:ident($($param:ident),+)),+) => {
        $(
            #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
            pub enum $name<$($param),+> {
                $(
                    $param($param),
                )+
            }
        )+
    };
}

define_either! {
    Either3(Left, Middle, Right),
    Either4(_1, _2, _3, _4),
    Either5(_1, _2, _3, _4, _5),
    Either6(_1, _2, _3, _4, _5, _6),
    Either7(_1, _2, _3, _4, _5, _6, _7),
    Either8(_1, _2, _3, _4, _5, _6, _7, _8),
    Either9(_1, _2, _3, _4, _5, _6, _7, _8, _9),
    Either10(_1, _2, _3, _4, _5, _6, _7, _8, _9, _10)
}

// Macro to implement map method for Either types
macro_rules! impl_either_map {
    ($either:ident, $($variant:ident => $type1:ident => $type2:ident =>  $fun:ident),+) => {
        impl<$($type1),+> $either<$($type1),+>
        {
            pub fn m_map<$($type2,)+ >(
                self,
                $($fun: impl FnOnce($type1) -> $type2,)+
            ) -> $either<$($type2),+>
            {
                match self {
                    $($either::$variant(a) => $either::$variant($fun(a)),)+
                }
            }
        }
    };
}






impl_either_map!(Either, Left => A => B => f, Right => C => D => g);
impl_either_map!(Either3, Left => A => B => f, Middle => C => D => g, Right => E => F => h);
impl_either_map!(Either4, _1 => A1 => B1 => f1, _2 => A2 => B2 => f2, _3 => A3 => B3 => f3, _4 => A4 => B4 => f4);
impl_either_map!(Either5, _1 => A1 => B1 => f1, _2 => A2 => B2 => f2, _3 => A3 => B3 => f3, _4 => A4 => B4 => f4, _5 => A5 => B5 => f5);
impl_either_map!(Either6, _1 => A1 => B1 => f1, _2 => A2 => B2 => f2, _3 => A3 => B3 => f3, _4 => A4 => B4 => f4, _5 => A5 => B5 => f5, _6 => A6 => B6 => f6);
impl_either_map!(Either7, _1 => A1 => B1 => f1, _2 => A2 => B2 => f2, _3 => A3 => B3 => f3, _4 => A4 => B4 => f4, _5 => A5 => B5 => f5, _6 => A6 => B6 => f6, _7 => A7 => B7 => f7);
impl_either_map!(Either8, _1 => A1 => B1 => f1, _2 => A2 => B2 => f2, _3 => A3 => B3 => f3, _4 => A4 => B4 => f4, _5 => A5 => B5 => f5, _6 => A6 => B6 => f6, _7 => A7 => B7 => f7, _8 => A8 => B8 => f8);
impl_either_map!(Either9, _1 => A1 => B1 => f1, _2 => A2 => B2 => f2, _3 => A3 => B3 => f3, _4 => A4 => B4 => f4, _5 => A5 => B5 => f5, _6 => A6 => B6 => f6, _7 => A7 => B7 => f7, _8 => A8 => B8 => f8, _9 => A9 => B9 => f9);
impl_either_map!(Either10, _1 => A1 => B1 => f1, _2 => A2 => B2 => f2, _3 => A3 => B3 => f3, _4 => A4 => B4 => f4, _5 => A5 => B5 => f5, _6 => A6 => B6 => f6, _7 => A7 => B7 => f7, _8 => A8 => B8 => f8, _9 => A9 => B9 => f9, _10 => A10 => B10 => f10);




macro_rules! impl_map_n {
    ($fun:ident for $type:ident => [ $($pre:ident = $pre_var:ident),* ] : $A:ident = $Var:ident : [ $($post:ident = $post_var:ident),* ] ) => {
        impl<$($pre,)* $A, $($post),*> $type<$($pre,)* $A, $($post),*> {
            pub fn $fun<B, F>(self, mut f: F) -> $type<$($pre,)* B, $($post),*>
            where
                F: FnMut($A) -> B,
            {
                match self {
                    $($type::$pre_var(a) => $type::$pre_var(a),)*
                        $type::$Var(a) => $type::$Var(f(a)),
                    $($type::$post_var(a) => $type::$post_var(a),)*
                }
            }
        }
    };
}


// Either - map_0, map_1
impl_map_n!(map_0 for Either => [] : A1 = Left : [A2 = Right]);
impl_map_n!(map_1 for Either => [A1 = Left] : A2 = Right : []);

// Either3 - map_0, map_1, map_2
impl_map_n!(map_0 for Either3 => [] : A1 = Left : [A2 = Middle, A3 = Right]);
impl_map_n!(map_1 for Either3 => [A1 = Left] : A2 = Middle : [A3 = Right]);
impl_map_n!(map_2 for Either3 => [A1 = Left, A2 = Middle] : A3 = Right : []);

// Either4 - map_0, map_1, map_2, map_3
impl_map_n!(map_0 for Either4 => [] : A1 = _1 : [A2 = _2, A3 = _3, A4 = _4]);
impl_map_n!(map_1 for Either4 => [A1 = _1] : A2 = _2 : [A3 = _3, A4 = _4]);
impl_map_n!(map_2 for Either4 => [A1 = _1, A2 = _2] : A3 = _3 : [A4 = _4]);
impl_map_n!(map_3 for Either4 => [A1 = _1, A2 = _2, A3 = _3] : A4 = _4 : []);

// Either5 - map_0 through map_4
impl_map_n!(map_0 for Either5 => [] : A1 = _1 : [A2 = _2, A3 = _3, A4 = _4, A5 = _5]);
impl_map_n!(map_1 for Either5 => [A1 = _1] : A2 = _2 : [A3 = _3, A4 = _4, A5 = _5]);
impl_map_n!(map_2 for Either5 => [A1 = _1, A2 = _2] : A3 = _3 : [A4 = _4, A5 = _5]);
impl_map_n!(map_3 for Either5 => [A1 = _1, A2 = _2, A3 = _3] : A4 = _4 : [A5 = _5]);
impl_map_n!(map_4 for Either5 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4] : A5 = _5 : []);

// Either6 - map_0 through map_5
impl_map_n!(map_0 for Either6 => [] : A1 = _1 : [A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6]);
impl_map_n!(map_1 for Either6 => [A1 = _1] : A2 = _2 : [A3 = _3, A4 = _4, A5 = _5, A6 = _6]);
impl_map_n!(map_2 for Either6 => [A1 = _1, A2 = _2] : A3 = _3 : [A4 = _4, A5 = _5, A6 = _6]);
impl_map_n!(map_3 for Either6 => [A1 = _1, A2 = _2, A3 = _3] : A4 = _4 : [A5 = _5, A6 = _6]);
impl_map_n!(map_4 for Either6 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4] : A5 = _5 : [A6 = _6]);
impl_map_n!(map_5 for Either6 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5] : A6 = _6 : []);

// Either7 - map_0 through map_6
impl_map_n!(map_0 for Either7 => [] : A1 = _1 : [A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7]);
impl_map_n!(map_1 for Either7 => [A1 = _1] : A2 = _2 : [A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7]);
impl_map_n!(map_2 for Either7 => [A1 = _1, A2 = _2] : A3 = _3 : [A4 = _4, A5 = _5, A6 = _6, A7 = _7]);
impl_map_n!(map_3 for Either7 => [A1 = _1, A2 = _2, A3 = _3] : A4 = _4 : [A5 = _5, A6 = _6, A7 = _7]);
impl_map_n!(map_4 for Either7 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4] : A5 = _5 : [A6 = _6, A7 = _7]);
impl_map_n!(map_5 for Either7 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5] : A6 = _6 : [A7 = _7]);
impl_map_n!(map_6 for Either7 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6] : A7 = _7 : []);

// Either8 - map_0 through map_7
impl_map_n!(map_0 for Either8 => [] : A1 = _1 : [A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8]);
impl_map_n!(map_1 for Either8 => [A1 = _1] : A2 = _2 : [A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8]);
impl_map_n!(map_2 for Either8 => [A1 = _1, A2 = _2] : A3 = _3 : [A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8]);
impl_map_n!(map_3 for Either8 => [A1 = _1, A2 = _2, A3 = _3] : A4 = _4 : [A5 = _5, A6 = _6, A7 = _7, A8 = _8]);
impl_map_n!(map_4 for Either8 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4] : A5 = _5 : [A6 = _6, A7 = _7, A8 = _8]);
impl_map_n!(map_5 for Either8 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5] : A6 = _6 : [A7 = _7, A8 = _8]);
impl_map_n!(map_6 for Either8 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6] : A7 = _7 : [A8 = _8]);
impl_map_n!(map_7 for Either8 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7] : A8 = _8 : []);

// Either9 - map_0 through map_8
impl_map_n!(map_0 for Either9 => [] : A1 = _1 : [A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9]);
impl_map_n!(map_1 for Either9 => [A1 = _1] : A2 = _2 : [A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9]);
impl_map_n!(map_2 for Either9 => [A1 = _1, A2 = _2] : A3 = _3 : [A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9]);
impl_map_n!(map_3 for Either9 => [A1 = _1, A2 = _2, A3 = _3] : A4 = _4 : [A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9]);
impl_map_n!(map_4 for Either9 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4] : A5 = _5 : [A6 = _6, A7 = _7, A8 = _8, A9 = _9]);
impl_map_n!(map_5 for Either9 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5] : A6 = _6 : [A7 = _7, A8 = _8, A9 = _9]);
impl_map_n!(map_6 for Either9 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6] : A7 = _7 : [A8 = _8, A9 = _9]);
impl_map_n!(map_7 for Either9 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7] : A8 = _8 : [A9 = _9]);
impl_map_n!(map_8 for Either9 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8] : A9 = _9 : []);

// Either10 - map_0 through map_9
impl_map_n!(map_0 for Either10 => [] : A1 = _1 : [A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9, A10 = _10]);
impl_map_n!(map_1 for Either10 => [A1 = _1] : A2 = _2 : [A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9, A10 = _10]);
impl_map_n!(map_2 for Either10 => [A1 = _1, A2 = _2] : A3 = _3 : [A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9, A10 = _10]);
impl_map_n!(map_3 for Either10 => [A1 = _1, A2 = _2, A3 = _3] : A4 = _4 : [A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9, A10 = _10]);
impl_map_n!(map_4 for Either10 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4] : A5 = _5 : [A6 = _6, A7 = _7, A8 = _8, A9 = _9, A10 = _10]);
impl_map_n!(map_5 for Either10 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5] : A6 = _6 : [A7 = _7, A8 = _8, A9 = _9, A10 = _10]);
impl_map_n!(map_6 for Either10 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6] : A7 = _7 : [A8 = _8, A9 = _9, A10 = _10]);
impl_map_n!(map_7 for Either10 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7] : A8 = _8 : [A9 = _9, A10 = _10]);
impl_map_n!(map_8 for Either10 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8] : A9 = _9 : [A10 = _10]);
impl_map_n!(map_9 for Either10 => [A1 = _1, A2 = _2, A3 = _3, A4 = _4, A5 = _5, A6 = _6, A7 = _7, A8 = _8, A9 = _9] : A10 = _10 : []);











/// Trait for types that can be folded to a common result type.
pub trait Foldable {
    /// The result type of the fold operation.
    type Result;

    /// Folds the value into a single result.
    fn fold(self) -> Self::Result;
}


macro_rules! ident_as_a {
    ( $t: ident ) => { A };
}

/// Macro to implement Foldable trait for Either types with same variants
macro_rules! impl_either_foldable {
 

    

    // For Either3 through Either10
    ($type:ident, $($variant:ident),+) => {
        impl<A> Foldable for $type<$(ident_as_a!($variant),)+> {
            type Result = A;
            fn fold(self) -> A {
                match self {
                    $($type::$variant(a) )|+ => a,
                }
            }
        }
    };
}

// Apply the macro for each Either type
impl_either_foldable!(Either, Left, Right);
impl_either_foldable!(Either3, Left, Middle, Right);
impl_either_foldable!(Either4, _1, _2, _3, _4);
impl_either_foldable!(Either5, _1, _2, _3, _4, _5);
impl_either_foldable!(Either6, _1, _2, _3, _4, _5, _6);
impl_either_foldable!(Either7, _1, _2, _3, _4, _5, _6, _7);
impl_either_foldable!(Either8, _1, _2, _3, _4, _5, _6, _7, _8);
impl_either_foldable!(Either9, _1, _2, _3, _4, _5, _6, _7, _8, _9);
impl_either_foldable!(Either10, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10);



/// Macro to implement MultiFoldable traits for all Either types
macro_rules! impl_all_multi_foldables {
    ($variant_type:ident, $($type_param:ident),+) => {
        // Level 1 (Base level using Foldable)
        impl<A, $($type_param),+> MultiFoldable1 for $variant_type<$($type_param),+>
        where
            $(
                $type_param: Foldable<Result = A>,
            )+
        {
            type Result = A;
            fn multi_fold_1(self) -> A {
                match self {
                    $(
                        $variant_type::$type_param(a) => a.fold(),
                    )+
                }
            }
        }

        // Level 2 (Using MultiFoldable1)
        impl<A, $($type_param),+> MultiFoldable2 for $variant_type<$($type_param),+>
        where
            $(
                $type_param: MultiFoldable1<Result = A>,
            )+
        {
            type Result = A;
            fn multi_fold_2(self) -> A {
                match self {
                    $(
                        $variant_type::$type_param(a) => a.multi_fold_1(),
                    )+
                }
            }
        }

        // Level 3 (Using MultiFoldable2)
        impl<A, $($type_param),+> MultiFoldable3 for $variant_type<$($type_param),+>
        where
            $(
                $type_param: MultiFoldable2<Result = A>,
            )+
        {
            type Result = A;
            fn multi_fold_3(self) -> A {
                match self {
                    $(
                        $variant_type::$type_param(a) => a.multi_fold_2(),
                    )+
                }
            }
        }

        // Level 4 (Using MultiFoldable3)
        impl<A, $($type_param),+> MultiFoldable4 for $variant_type<$($type_param),+>
        where
            $(
                $type_param: MultiFoldable3<Result = A>,
            )+
        {
            type Result = A;
            fn multi_fold_4(self) -> A {
                match self {
                    $(
                        $variant_type::$type_param(a) => a.multi_fold_3(),
                    )+
                }
            }
        }

        // Level 5 (Using MultiFoldable4)
        impl<A, $($type_param),+> MultiFoldable5 for $variant_type<$($type_param),+>
        where
            $(
                $type_param: MultiFoldable4<Result = A>,
            )+
        {
            type Result = A;
            fn multi_fold_5(self) -> A {
                match self {
                    $(
                        $variant_type::$type_param(a) => a.multi_fold_4(),
                    )+
                }
            }
        }
    };
}

// Implement MultiFoldable traits for all Either types
impl_all_multi_foldables!(Either, Left, Right);
impl_all_multi_foldables!(Either3, Left, Middle, Right);
impl_all_multi_foldables!(Either4, _1, _2, _3, _4);
impl_all_multi_foldables!(Either5, _1, _2, _3, _4, _5);
impl_all_multi_foldables!(Either6, _1, _2, _3, _4, _5, _6);
impl_all_multi_foldables!(Either7, _1, _2, _3, _4, _5, _6, _7);
impl_all_multi_foldables!(Either8, _1, _2, _3, _4, _5, _6, _7, _8);
impl_all_multi_foldables!(Either9, _1, _2, _3, _4, _5, _6, _7, _8, _9);
impl_all_multi_foldables!(Either10, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10);





/// Trait for types that can be folded to a common result type.
pub trait MultiFoldable<Zero> {
    /// The result type of the fold operation.
    type Result;

    /// Folds the value into a single result.
    fn multi_fold(self) -> Self::Result;
}


impl <T,A> MultiFoldable<Zero> for T where
    T : MultiFoldable1<Result = A>
{
    type Result = A;

    fn multi_fold(self) -> Self::Result {
        self.multi_fold_1()
    }
}


impl <T,A> MultiFoldable<S<Zero>> for T where
    T : MultiFoldable2<Result = A>
{
    type Result = A;

    fn multi_fold(self) -> Self::Result {
        self.multi_fold_2()
    }
}

impl <T,A> MultiFoldable<S<S<Zero>>> for T where
    T : MultiFoldable3<Result = A>
{
    type Result = A;

    fn multi_fold(self) -> Self::Result {
        self.multi_fold_3()
    }
}

impl <T,A> MultiFoldable<S<S<S<Zero>>>> for T where
    T : MultiFoldable4<Result = A>
{
    type Result = A;

    fn multi_fold(self) -> Self::Result {
        self.multi_fold_4()
    }
}

impl <T,A> MultiFoldable<S<S<S<S<Zero>>>>> for T where
    T : MultiFoldable5<Result = A>
{
    type Result = A;

    fn multi_fold(self) -> Self::Result {
        self.multi_fold_5()
    }
}




/// Trait for types that can be folded to a common result type.
pub trait MultiFoldable1 {
    /// The result type of the fold operation.
    type Result;

    /// Folds the value into a single result.
    fn multi_fold_1(self) -> Self::Result;
}

/// Trait for types that can be folded to a common result type.
pub trait MultiFoldable2 {
    /// The result type of the fold operation.
    type Result;

    /// Folds the value into a single result.
    fn multi_fold_2(self) -> Self::Result;
}

/// Trait for types that can be folded to a common result type.
pub trait MultiFoldable3 {
    /// The result type of the fold operation.
    type Result;

    /// Folds the value into a single result.
    fn multi_fold_3(self) -> Self::Result;
}

/// Trait for types that can be folded to a common result type.
pub trait MultiFoldable4 {
    /// The result type of the fold operation.
    type Result;

    /// Folds the value into a single result.
    fn multi_fold_4(self) -> Self::Result;
}

/// Trait for types that can be folded to a common result type.
pub trait MultiFoldable5 {
    /// The result type of the fold operation.
    type Result;

    /// Folds the value into a single result.
    fn multi_fold_5(self) -> Self::Result;
}


pub struct Zero;
pub struct S<N>(std::marker::PhantomData<N>);



/// Marker trait for sum and product types.
pub trait SumAndProdType {}

/// Marker trait for product types.
pub trait ProdType: SumAndProdType {}

/// Marker trait for sum types.
pub trait SumType: SumAndProdType {}


macro_rules! impl_sum_type_for_either {
    // Implementation for each Either type with the appropriate number of type parameters
    ($($either:ident<$($T:ident),+>),*) => {
        $(
            impl<$($T),+> SumAndProdType for ($($T),+) {}
            impl<$($T),+> ProdType for ($($T),+) {}
            impl<$($T),+> SumType for $either<$($T),+> {}
            impl<$($T),+> SumAndProdType for $either<$($T),+> {}
        )*
    };
}

// Apply the macro for all Either types
impl_sum_type_for_either!(
    Either<T1, T2>,
    Either3<T1, T2, T3>,
    Either4<T1, T2, T3, T4>,
    Either5<T1, T2, T3, T4, T5>,
    Either6<T1, T2, T3, T4, T5, T6>,
    Either7<T1, T2, T3, T4, T5, T6, T7>,
    Either8<T1, T2, T3, T4, T5, T6, T7, T8>,
    Either9<T1, T2, T3, T4, T5, T6, T7, T8, T9>,
    Either10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
);

impl SumType for () {}
impl<T1> SumType for (T1,) {}
impl SumAndProdType for () {}
impl<T1> SumAndProdType for (T1,) {}
impl ProdType for () {}
impl<T1> ProdType for (T1,) {}
