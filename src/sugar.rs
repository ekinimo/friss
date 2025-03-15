//! # Parser Sugar for N-ary Operations
//!
//! This module provides syntactic sugar for working with tuple-based parsers,
//! allowing for convenient sequencing and alternation of multiple parsers.

use crate::core::{Parsable, Parser};
use crate::types::*;

/// Trait for parser sugar, providing convenient methods for parser combinators.
///
/// This trait simplifies the creation of complex parsers by enabling the
/// composition of multiple parsers in sequence or as alternatives.
pub trait ParserSugar<
    In: Parsable<AltError> + Parsable<SeqError>,
    SeqOut: ProdType + ToOrOutput<In>,
    SeqError: SumType + Clone,
    AltOut: SumType,
    AltError: ProdType + Clone,
>
{
    /// Sequences the parsers in this tuple, producing a tuple of their results.
    fn seq(self) -> impl Parser<In, SeqOut, SeqError>;

    /// Creates a parser that tries each parser in this tuple, returning the first success.
    fn alt(self) -> impl Parser<In, AltOut, AltError>;

    /// Creates a parser that tries each parser in this tuple, returning all of them.
    fn or(self) -> impl Parser<In, <SeqOut as ToOrOutput<In>>::OrOutput, AltError>
    where
        In: Clone;
}

/// Type family for converting from sequence output to or output
pub trait ToOrOutput<In> {
    /// The corresponding OrOutput type
    type OrOutput;
}

impl<T1, T2, In> ToOrOutput<In> for (T1, T2) {
    type OrOutput = (Option<(In, T1)>, Option<(In, T2)>);
}

impl<T1, T2, T3, In> ToOrOutput<In> for (T1, T2, T3) {
    type OrOutput = (Option<(In, T1)>, Option<(In, T2)>, Option<(In, T3)>);
}

impl<T1, T2, T3, T4, In> ToOrOutput<In> for (T1, T2, T3, T4) {
    type OrOutput = (
        Option<(In, T1)>,
        Option<(In, T2)>,
        Option<(In, T3)>,
        Option<(In, T4)>,
    );
}

impl<T1, T2, T3, T4, T5, In> ToOrOutput<In> for (T1, T2, T3, T4, T5) {
    type OrOutput = (
        Option<(In, T1)>,
        Option<(In, T2)>,
        Option<(In, T3)>,
        Option<(In, T4)>,
        Option<(In, T5)>,
    );
}

impl<T1, T2, T3, T4, T5, T6, In> ToOrOutput<In> for (T1, T2, T3, T4, T5, T6) {
    type OrOutput = (
        Option<(In, T1)>,
        Option<(In, T2)>,
        Option<(In, T3)>,
        Option<(In, T4)>,
        Option<(In, T5)>,
        Option<(In, T6)>,
    );
}
impl<T1, T2, T3, T4, T5, T6, T7, In> ToOrOutput<In> for (T1, T2, T3, T4, T5, T6, T7) {
    type OrOutput = (
        Option<(In, T1)>,
        Option<(In, T2)>,
        Option<(In, T3)>,
        Option<(In, T4)>,
        Option<(In, T5)>,
        Option<(In, T6)>,
        Option<(In, T7)>,
    );
}
impl<T1, T2, T3, T4, T5, T6, T7, T8, In> ToOrOutput<In> for (T1, T2, T3, T4, T5, T6, T7, T8) {
    type OrOutput = (
        Option<(In, T1)>,
        Option<(In, T2)>,
        Option<(In, T3)>,
        Option<(In, T4)>,
        Option<(In, T5)>,
        Option<(In, T6)>,
        Option<(In, T7)>,
        Option<(In, T8)>,
    );
}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, In> ToOrOutput<In>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type OrOutput = (
        Option<(In, T1)>,
        Option<(In, T2)>,
        Option<(In, T3)>,
        Option<(In, T4)>,
        Option<(In, T5)>,
        Option<(In, T6)>,
        Option<(In, T7)>,
        Option<(In, T8)>,
        Option<(In, T9)>,
    );
}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, In> ToOrOutput<In>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type OrOutput = (
        Option<(In, T1)>,
        Option<(In, T2)>,
        Option<(In, T3)>,
        Option<(In, T4)>,
        Option<(In, T5)>,
        Option<(In, T6)>,
        Option<(In, T7)>,
        Option<(In, T8)>,
        Option<(In, T9)>,
        Option<(In, T10)>,
    );
}

// Implementations for specific tuples
impl<In, Out1, Out2, Error1, Error2, P1, P2>
    ParserSugar<In, (Out1, Out2), Either<Error1, Error2>, Either<Out1, Out2>, (Error1, Error2)>
    for (P1, P2)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<(Error1, Error2)>
        + Parsable<Either<Error1, Error2>>,
    Error1: Clone,
    Error2: Clone,
{
    fn seq(self) -> impl Parser<In, (Out1, Out2), Either<Error1, Error2>> {
        let (a0, a1) = self;
        a0.seq(a1)
    }

    fn alt(self) -> impl Parser<In, Either<Out1, Out2>, (Error1, Error2)> {
        let (a0, a1) = self;
        a0.alt(a1)
    }

    fn or(self) -> impl Parser<In, (Option<(In, Out1)>, Option<(In, Out2)>), (Error1, Error2)>
    where
        In: Clone,
    {
        let (a0, a1) = self;
        a0.or(a1)
    }
}

impl<In, Out1, Out2, Out3, Error1, Error2, Error3, P1, P2, P3>
    ParserSugar<
        In,
        (Out1, Out2, Out3),
        Either3<Error1, Error2, Error3>,
        Either3<Out1, Out2, Out3>,
        (Error1, Error2, Error3),
    > for (P1, P2, P3)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<((Error1, Error2), Error3)>
        + Parsable<Either<Either<Error1, Error2>, Error3>>,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
{
    fn seq(self) -> impl Parser<In, (Out1, Out2, Out3), Either3<Error1, Error2, Error3>> {
        let (a0, a1, a2) = self;
        a0.seq(a1)
            .seq(a2)
            .map(|((a, b), y)| (a, b, y))
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either::Left(x) => Either3::Left(x),
                    Either::Right(x) => Either3::Middle(x),
                },
                Either::Right(x) => Either3::Right(x),
            })
    }

    fn alt(self) -> impl Parser<In, Either3<Out1, Out2, Out3>, (Error1, Error2, Error3)> {
        let (a0, a1, a2) = self;
        a0.alt(a1)
            .alt(a2)
            .map_err(|((a, b), y)| (a, b, y))
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either::Left(x) => Either3::Left(x),
                    Either::Right(x) => Either3::Middle(x),
                },
                Either::Right(x) => Either3::Right(x),
            })
    }

    fn or(
        self,
    ) -> impl Parser<
        In,
        (Option<(In, Out1)>, Option<(In, Out2)>, Option<(In, Out3)>),
        (Error1, Error2, Error3),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2) = self;
        a0.or(a1)
            .or(a2)
            .map_err(|((a, b), y)| (a, b, y))
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
    }
}

impl<In, Out1, Out2, Out3, Out4, Error1, Error2, Error3, Error4, P1, P2, P3, P4>
    ParserSugar<
        In,
        (Out1, Out2, Out3, Out4),
        Either4<Error1, Error2, Error3, Error4>,
        Either4<Out1, Out2, Out3, Out4>,
        (Error1, Error2, Error3, Error4),
    > for (P1, P2, P3, P4)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    P4: Parser<In, Out4, Error4>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<Error4>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<(Error1, Error2, Error3, Error4)>
        + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<Either4<Error1, Error2, Error3, Error4>>
        + Parsable<((Error1, Error2), Error3)>
        + Parsable<((Error1, Error2, Error3), Error4)>
        + Parsable<Either<Either<Error1, Error2>, Error3>>
        + Parsable<Either<Either3<Error1, Error2, Error3>, Error4>>,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
    Error4: Clone,
{
    fn seq(
        self,
    ) -> impl Parser<In, (Out1, Out2, Out3, Out4), Either4<Error1, Error2, Error3, Error4>> {
        let (a0, a1, a2, a3) = self;

        (a0, a1, a2)
            .seq()
            .seq(a3)
            .map(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either3::Left(x) => Either4::_1(x),
                    Either3::Middle(x) => Either4::_2(x),
                    Either3::Right(x) => Either4::_3(x),
                },
                Either::Right(x) => Either4::_4(x),
            })
    }

    fn alt(
        self,
    ) -> impl Parser<In, Either4<Out1, Out2, Out3, Out4>, (Error1, Error2, Error3, Error4)> {
        let (a0, a1, a2, a3) = self;

        (a0, a1, a2)
            .alt()
            .alt(a3)
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either3::Left(x) => Either4::_1(x),
                    Either3::Middle(x) => Either4::_2(x),
                    Either3::Right(x) => Either4::_3(x),
                },
                Either::Right(x) => Either4::_4(x),
            })
    }

    fn or(
        self,
    ) -> impl Parser<
        In,
        (
            Option<(In, Out1)>,
            Option<(In, Out2)>,
            Option<(In, Out3)>,
            Option<(In, Out4)>,
        ),
        (Error1, Error2, Error3, Error4),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2, a3) = self;
        a0.or(a1)
            .or(a2)
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
            .map_err(|((a, b), y)| (a, b, y))
            .or(a3)
            .map(|(a, d)| match a {
                Some((_, (a, b, c))) => (a, b, c, d),
                None => (None, None, None, d),
            })
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
    }
}

impl<
        In,
        Out1,
        Out2,
        Out3,
        Out4,
        Out5,
        Error1,
        Error2,
        Error3,
        Error4,
        Error5,
        P1,
        P2,
        P3,
        P4,
        P5,
    >
    ParserSugar<
        In,
        (Out1, Out2, Out3, Out4, Out5),
        Either5<Error1, Error2, Error3, Error4, Error5>,
        Either5<Out1, Out2, Out3, Out4, Out5>,
        (Error1, Error2, Error3, Error4, Error5),
    > for (P1, P2, P3, P4, P5)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    P4: Parser<In, Out4, Error4>,
    P5: Parser<In, Out5, Error5>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<Error4>
        + Parsable<Error5>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<(Error1, Error2, Error3, Error4)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5)>
        + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<Either4<Error1, Error2, Error3, Error4>>
        + Parsable<Either5<Error1, Error2, Error3, Error4, Error5>>
        + Parsable<((Error1, Error2), Error3)>
        + Parsable<((Error1, Error2, Error3), Error4)>
        + Parsable<((Error1, Error2, Error3, Error4), Error5)>
        + Parsable<Either<Either<Error1, Error2>, Error3>>
        + Parsable<Either<Either3<Error1, Error2, Error3>, Error4>>
        + Parsable<Either<Either4<Error1, Error2, Error3, Error4>, Error5>>,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
    Error4: Clone,
    Error5: Clone,
{
    fn seq(
        self,
    ) -> impl Parser<In, (Out1, Out2, Out3, Out4, Out5), Either5<Error1, Error2, Error3, Error4, Error5>>
    {
        let (a0, a1, a2, a3, a4) = self;

        (a0, a1, a2, a3)
            .seq()
            .seq(a4)
            .map(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either4::_1(x) => Either5::_1(x),
                    Either4::_2(x) => Either5::_2(x),
                    Either4::_3(x) => Either5::_3(x),
                    Either4::_4(x) => Either5::_4(x),
                },
                Either::Right(x) => Either5::_5(x),
            })
    }

    fn alt(
        self,
    ) -> impl Parser<In, Either5<Out1, Out2, Out3, Out4, Out5>, (Error1, Error2, Error3, Error4, Error5)>
    {
        let (a0, a1, a2, a3, a4) = self;

        (a0, a1, a2, a3)
            .alt()
            .alt(a4)
            .map_err(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either4::_1(x) => Either5::_1(x),
                    Either4::_2(x) => Either5::_2(x),
                    Either4::_3(x) => Either5::_3(x),
                    Either4::_4(x) => Either5::_4(x),
                },
                Either::Right(x) => Either5::_5(x),
            })
    }

    fn or(
        self,
    ) -> impl Parser<
        In,
        (
            Option<(In, Out1)>,
            Option<(In, Out2)>,
            Option<(In, Out3)>,
            Option<(In, Out4)>,
            Option<(In, Out5)>,
        ),
        (Error1, Error2, Error3, Error4, Error5),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2, a3, a4) = self;
        a0.or(a1)
            .or(a2)
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
            .map_err(|((a, b), y)| (a, b, y))
            .or(a3)
            .map(|(a, d)| match a {
                Some((_, (a, b, c))) => (a, b, c, d),
                None => (None, None, None, d),
            })
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .or(a4)
            .map(|(a, e)| match a {
                Some((_, (a, b, c, d))) => (a, b, c, d, e),
                None => (None, None, None, None, e),
            })
            .map_err(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
    }
}

impl<
        In,
        Out1,
        Out2,
        Out3,
        Out4,
        Out5,
        Out6,
        Error1,
        Error2,
        Error3,
        Error4,
        Error5,
        Error6,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
    >
    ParserSugar<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6),
        Either6<Error1, Error2, Error3, Error4, Error5, Error6>,
        Either6<Out1, Out2, Out3, Out4, Out5, Out6>,
        (Error1, Error2, Error3, Error4, Error5, Error6),
    > for (P1, P2, P3, P4, P5, P6)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    P4: Parser<In, Out4, Error4>,
    P5: Parser<In, Out5, Error5>,
    P6: Parser<In, Out6, Error6>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<Error4>
        + Parsable<Error5>
        + Parsable<Error6>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<(Error1, Error2, Error3, Error4)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6)>
        + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<Either4<Error1, Error2, Error3, Error4>>
        + Parsable<Either5<Error1, Error2, Error3, Error4, Error5>>
        + Parsable<Either6<Error1, Error2, Error3, Error4, Error5, Error6>>
        + Parsable<((Error1, Error2), Error3)>
        + Parsable<((Error1, Error2, Error3), Error4)>
        + Parsable<((Error1, Error2, Error3, Error4), Error5)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5), Error6)>
        + Parsable<Either<Either<Error1, Error2>, Error3>>
        + Parsable<Either<Either3<Error1, Error2, Error3>, Error4>>
        + Parsable<Either<Either4<Error1, Error2, Error3, Error4>, Error5>>
        + Parsable<Either<Either5<Error1, Error2, Error3, Error4, Error5>, Error6>>,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
    Error4: Clone,
    Error5: Clone,
    Error6: Clone,
{
    fn seq(
        self,
    ) -> impl Parser<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6),
        Either6<Error1, Error2, Error3, Error4, Error5, Error6>,
    > {
        let (a0, a1, a2, a3, a4, a5) = self;

        (a0, a1, a2, a3, a4)
            .seq()
            .seq(a5)
            .map(|((a1, a2, a3, a4, a5), a6)| (a1, a2, a3, a4, a5, a6))
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either5::_1(x) => Either6::_1(x),
                    Either5::_2(x) => Either6::_2(x),
                    Either5::_3(x) => Either6::_3(x),
                    Either5::_4(x) => Either6::_4(x),
                    Either5::_5(x) => Either6::_5(x),
                },
                Either::Right(x) => Either6::_6(x),
            })
    }

    fn alt(
        self,
    ) -> impl Parser<
        In,
        Either6<Out1, Out2, Out3, Out4, Out5, Out6>,
        (Error1, Error2, Error3, Error4, Error5, Error6),
    > {
        let (a0, a1, a2, a3, a4, a5) = self;

        (a0, a1, a2, a3, a4)
            .alt()
            .alt(a5)
            .map_err(|((a1, a2, a3, a4, a5), a6)| (a1, a2, a3, a4, a5, a6))
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either5::_1(x) => Either6::_1(x),
                    Either5::_2(x) => Either6::_2(x),
                    Either5::_3(x) => Either6::_3(x),
                    Either5::_4(x) => Either6::_4(x),
                    Either5::_5(x) => Either6::_5(x),
                },
                Either::Right(x) => Either6::_6(x),
            })
    }

    fn or(
        self,
    ) -> impl Parser<
        In,
        (
            Option<(In, Out1)>,
            Option<(In, Out2)>,
            Option<(In, Out3)>,
            Option<(In, Out4)>,
            Option<(In, Out5)>,
            Option<(In, Out6)>,
        ),
        (Error1, Error2, Error3, Error4, Error5, Error6),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2, a3, a4, a5) = self;
        a0.or(a1)
            .or(a2)
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
            .map_err(|((a, b), y)| (a, b, y))
            .or(a3)
            .map(|(a, d)| match a {
                Some((_, (a, b, c))) => (a, b, c, d),
                None => (None, None, None, d),
            })
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .or(a4)
            .map(|(a, e)| match a {
                Some((_, (a, b, c, d))) => (a, b, c, d, e),
                None => (None, None, None, None, e),
            })
            .map_err(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
            .or(a5)
            .map(|(a, f)| match a {
                Some((_, (a, b, c, d, e))) => (a, b, c, d, e, f),
                None => (None, None, None, None, None, f),
            })
            .map_err(|((a1, a2, a3, a4, a5), a6)| (a1, a2, a3, a4, a5, a6))
    }
}

impl<
        In,
        Out1,
        Out2,
        Out3,
        Out4,
        Out5,
        Out6,
        Out7,
        Error1,
        Error2,
        Error3,
        Error4,
        Error5,
        Error6,
        Error7,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
    >
    ParserSugar<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7),
        Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>,
        Either7<Out1, Out2, Out3, Out4, Out5, Out6, Out7>,
        (Error1, Error2, Error3, Error4, Error5, Error6, Error7),
    > for (P1, P2, P3, P4, P5, P6, P7)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    P4: Parser<In, Out4, Error4>,
    P5: Parser<In, Out5, Error5>,
    P6: Parser<In, Out6, Error6>,
    P7: Parser<In, Out7, Error7>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<Error4>
        + Parsable<Error5>
        + Parsable<Error6>
        + Parsable<Error7>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<(Error1, Error2, Error3, Error4)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6, Error7)>
        + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<Either4<Error1, Error2, Error3, Error4>>
        + Parsable<Either5<Error1, Error2, Error3, Error4, Error5>>
        + Parsable<Either6<Error1, Error2, Error3, Error4, Error5, Error6>>
        + Parsable<Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>>
        + Parsable<((Error1, Error2), Error3)>
        + Parsable<((Error1, Error2, Error3), Error4)>
        + Parsable<((Error1, Error2, Error3, Error4), Error5)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5), Error6)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5, Error6), Error7)>
        + Parsable<Either<Either<Error1, Error2>, Error3>>
        + Parsable<Either<Either3<Error1, Error2, Error3>, Error4>>
        + Parsable<Either<Either4<Error1, Error2, Error3, Error4>, Error5>>
        + Parsable<Either<Either5<Error1, Error2, Error3, Error4, Error5>, Error6>>
        + Parsable<Either<Either6<Error1, Error2, Error3, Error4, Error5, Error6>, Error7>>,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
    Error4: Clone,
    Error5: Clone,
    Error6: Clone,
    Error7: Clone,
{
    fn seq(
        self,
    ) -> impl Parser<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7),
        Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>,
    > {
        let (a0, a1, a2, a3, a4, a5, a6) = self;

        (a0, a1, a2, a3, a4, a5)
            .seq()
            .seq(a6)
            .map(|((a1, a2, a3, a4, a5, a6), a7)| (a1, a2, a3, a4, a5, a6, a7))
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either6::_1(x) => Either7::_1(x),
                    Either6::_2(x) => Either7::_2(x),
                    Either6::_3(x) => Either7::_3(x),
                    Either6::_4(x) => Either7::_4(x),
                    Either6::_5(x) => Either7::_5(x),
                    Either6::_6(x) => Either7::_6(x),
                },
                Either::Right(x) => Either7::_7(x),
            })
    }

    fn alt(
        self,
    ) -> impl Parser<
        In,
        Either7<Out1, Out2, Out3, Out4, Out5, Out6, Out7>,
        (Error1, Error2, Error3, Error4, Error5, Error6, Error7),
    > {
        let (a0, a1, a2, a3, a4, a5, a6) = self;

        (a0, a1, a2, a3, a4, a5)
            .alt()
            .alt(a6)
            .map_err(|((a1, a2, a3, a4, a5, a6), a7)| (a1, a2, a3, a4, a5, a6, a7))
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either6::_1(x) => Either7::_1(x),
                    Either6::_2(x) => Either7::_2(x),
                    Either6::_3(x) => Either7::_3(x),
                    Either6::_4(x) => Either7::_4(x),
                    Either6::_5(x) => Either7::_5(x),
                    Either6::_6(x) => Either7::_6(x),
                },
                Either::Right(x) => Either7::_7(x),
            })
    }
    fn or(
        self,
    ) -> impl Parser<
        In,
        (
            Option<(In, Out1)>,
            Option<(In, Out2)>,
            Option<(In, Out3)>,
            Option<(In, Out4)>,
            Option<(In, Out5)>,
            Option<(In, Out6)>,
            Option<(In, Out7)>,
        ),
        (Error1, Error2, Error3, Error4, Error5, Error6, Error7),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2, a3, a4, a5, a6) = self;
        a0.or(a1)
            .or(a2)
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
            .map_err(|((a, b), y)| (a, b, y))
            .or(a3)
            .map(|(a, d)| match a {
                Some((_, (a, b, c))) => (a, b, c, d),
                None => (None, None, None, d),
            })
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .or(a4)
            .map(|(a, e)| match a {
                Some((_, (a, b, c, d))) => (a, b, c, d, e),
                None => (None, None, None, None, e),
            })
            .map_err(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
            .or(a5)
            .map(|(a, f)| match a {
                Some((_, (a, b, c, d, e))) => (a, b, c, d, e, f),
                None => (None, None, None, None, None, f),
            })
            .map_err(|((a1, a2, a3, a4, a5), a6)| (a1, a2, a3, a4, a5, a6))
            .or(a6)
            .map(|(a, g)| match a {
                Some((_, (a, b, c, d, e, f))) => (a, b, c, d, e, f, g),
                None => (None, None, None, None, None, None, g),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6), a7)| (a1, a2, a3, a4, a5, a6, a7))
    }
}

impl<
        In,
        Out1,
        Out2,
        Out3,
        Out4,
        Out5,
        Out6,
        Out7,
        Out8,
        Error1,
        Error2,
        Error3,
        Error4,
        Error5,
        Error6,
        Error7,
        Error8,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
    >
    ParserSugar<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8),
        Either8<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8>,
        Either8<Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8>,
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
        ),
    > for (P1, P2, P3, P4, P5, P6, P7, P8)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    P4: Parser<In, Out4, Error4>,
    P5: Parser<In, Out5, Error5>,
    P6: Parser<In, Out6, Error6>,
    P7: Parser<In, Out7, Error7>,
    P8: Parser<In, Out8, Error8>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<Error4>
        + Parsable<Error5>
        + Parsable<Error6>
        + Parsable<Error7>
        + Parsable<Error8>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<(Error1, Error2, Error3, Error4)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6, Error7)>
        + Parsable<(
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
        )> + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<Either4<Error1, Error2, Error3, Error4>>
        + Parsable<Either5<Error1, Error2, Error3, Error4, Error5>>
        + Parsable<Either6<Error1, Error2, Error3, Error4, Error5, Error6>>
        + Parsable<Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>>
        + Parsable<Either8<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8>>
        + Parsable<((Error1, Error2), Error3)>
        + Parsable<((Error1, Error2, Error3), Error4)>
        + Parsable<((Error1, Error2, Error3, Error4), Error5)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5), Error6)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5, Error6), Error7)>
        + Parsable<(
            (Error1, Error2, Error3, Error4, Error5, Error6, Error7),
            Error8,
        )> + Parsable<Either<Either<Error1, Error2>, Error3>>
        + Parsable<Either<Either3<Error1, Error2, Error3>, Error4>>
        + Parsable<Either<Either4<Error1, Error2, Error3, Error4>, Error5>>
        + Parsable<Either<Either5<Error1, Error2, Error3, Error4, Error5>, Error6>>
        + Parsable<Either<Either6<Error1, Error2, Error3, Error4, Error5, Error6>, Error7>>
        + Parsable<Either<Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>, Error8>>,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
    Error4: Clone,
    Error5: Clone,
    Error6: Clone,
    Error7: Clone,
    Error8: Clone,
{
    fn seq(
        self,
    ) -> impl Parser<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8),
        Either8<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8>,
    > {
        let (a0, a1, a2, a3, a4, a5, a6, a7) = self;

        (a0, a1, a2, a3, a4, a5, a6)
            .seq()
            .seq(a7)
            .map(|((a1, a2, a3, a4, a5, a6, a7), a8)| (a1, a2, a3, a4, a5, a6, a7, a8))
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either7::_1(x) => Either8::_1(x),
                    Either7::_2(x) => Either8::_2(x),
                    Either7::_3(x) => Either8::_3(x),
                    Either7::_4(x) => Either8::_4(x),
                    Either7::_5(x) => Either8::_5(x),
                    Either7::_6(x) => Either8::_6(x),
                    Either7::_7(x) => Either8::_7(x),
                },
                Either::Right(x) => Either8::_8(x),
            })
    }

    fn alt(
        self,
    ) -> impl Parser<
        In,
        Either8<Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8>,
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
        ),
    > {
        let (a0, a1, a2, a3, a4, a5, a6, a7) = self;

        (a0, a1, a2, a3, a4, a5, a6)
            .alt()
            .alt(a7)
            .map_err(|((a1, a2, a3, a4, a5, a6, a7), a8)| (a1, a2, a3, a4, a5, a6, a7, a8))
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either7::_1(x) => Either8::_1(x),
                    Either7::_2(x) => Either8::_2(x),
                    Either7::_3(x) => Either8::_3(x),
                    Either7::_4(x) => Either8::_4(x),
                    Either7::_5(x) => Either8::_5(x),
                    Either7::_6(x) => Either8::_6(x),
                    Either7::_7(x) => Either8::_7(x),
                },
                Either::Right(x) => Either8::_8(x),
            })
    }

    fn or(
        self,
    ) -> impl Parser<
        In,
        (
            Option<(In, Out1)>,
            Option<(In, Out2)>,
            Option<(In, Out3)>,
            Option<(In, Out4)>,
            Option<(In, Out5)>,
            Option<(In, Out6)>,
            Option<(In, Out7)>,
            Option<(In, Out8)>,
        ),
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
        ),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2, a3, a4, a5, a6, a7) = self;
        a0.or(a1)
            .or(a2)
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
            .map_err(|((a, b), y)| (a, b, y))
            .or(a3)
            .map(|(a, d)| match a {
                Some((_, (a, b, c))) => (a, b, c, d),
                None => (None, None, None, d),
            })
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .or(a4)
            .map(|(a, e)| match a {
                Some((_, (a, b, c, d))) => (a, b, c, d, e),
                None => (None, None, None, None, e),
            })
            .map_err(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
            .or(a5)
            .map(|(a, f)| match a {
                Some((_, (a, b, c, d, e))) => (a, b, c, d, e, f),
                None => (None, None, None, None, None, f),
            })
            .map_err(|((a1, a2, a3, a4, a5), a6)| (a1, a2, a3, a4, a5, a6))
            .or(a6)
            .map(|(a, g)| match a {
                Some((_, (a, b, c, d, e, f))) => (a, b, c, d, e, f, g),
                None => (None, None, None, None, None, None, g),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6), a7)| (a1, a2, a3, a4, a5, a6, a7))
            .or(a7)
            .map(|(a, h)| match a {
                Some((_, (a, b, c, d, e, f, g))) => (a, b, c, d, e, f, g, h),
                None => (None, None, None, None, None, None, None, h),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6, a7), a8)| (a1, a2, a3, a4, a5, a6, a7, a8))
    }
}

impl<
        In,
        Out1,
        Out2,
        Out3,
        Out4,
        Out5,
        Out6,
        Out7,
        Out8,
        Out9,
        Error1,
        Error2,
        Error3,
        Error4,
        Error5,
        Error6,
        Error7,
        Error8,
        Error9,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
    >
    ParserSugar<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9),
        Either9<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8, Error9>,
        Either9<Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9>,
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
        ),
    > for (P1, P2, P3, P4, P5, P6, P7, P8, P9)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    P4: Parser<In, Out4, Error4>,
    P5: Parser<In, Out5, Error5>,
    P6: Parser<In, Out6, Error6>,
    P7: Parser<In, Out7, Error7>,
    P8: Parser<In, Out8, Error8>,
    P9: Parser<In, Out9, Error9>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<Error4>
        + Parsable<Error5>
        + Parsable<Error6>
        + Parsable<Error7>
        + Parsable<Error8>
        + Parsable<Error9>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<(Error1, Error2, Error3, Error4)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6, Error7)>
        + Parsable<(
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
        )> + Parsable<(
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
        )> + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<Either4<Error1, Error2, Error3, Error4>>
        + Parsable<Either5<Error1, Error2, Error3, Error4, Error5>>
        + Parsable<Either6<Error1, Error2, Error3, Error4, Error5, Error6>>
        + Parsable<Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>>
        + Parsable<Either8<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8>>
        + Parsable<Either9<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8, Error9>>
        + Parsable<((Error1, Error2), Error3)>
        + Parsable<((Error1, Error2, Error3), Error4)>
        + Parsable<((Error1, Error2, Error3, Error4), Error5)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5), Error6)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5, Error6), Error7)>
        + Parsable<(
            (Error1, Error2, Error3, Error4, Error5, Error6, Error7),
            Error8,
        )> + Parsable<(
            (
                Error1,
                Error2,
                Error3,
                Error4,
                Error5,
                Error6,
                Error7,
                Error8,
            ),
            Error9,
        )> + Parsable<Either<Either<Error1, Error2>, Error3>>
        + Parsable<Either<Either3<Error1, Error2, Error3>, Error4>>
        + Parsable<Either<Either4<Error1, Error2, Error3, Error4>, Error5>>
        + Parsable<Either<Either5<Error1, Error2, Error3, Error4, Error5>, Error6>>
        + Parsable<Either<Either6<Error1, Error2, Error3, Error4, Error5, Error6>, Error7>>
        + Parsable<Either<Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>, Error8>>
        + Parsable<
            Either<Either8<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8>, Error9>,
        >,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
    Error4: Clone,
    Error5: Clone,
    Error6: Clone,
    Error7: Clone,
    Error8: Clone,
    Error9: Clone,
{
    fn seq(
        self,
    ) -> impl Parser<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9),
        Either9<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8, Error9>,
    > {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8) = self;

        (a0, a1, a2, a3, a4, a5, a6, a7)
            .seq()
            .seq(a8)
            .map(|((a1, a2, a3, a4, a5, a6, a7, a8), a9)| (a1, a2, a3, a4, a5, a6, a7, a8, a9))
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either8::_1(x) => Either9::_1(x),
                    Either8::_2(x) => Either9::_2(x),
                    Either8::_3(x) => Either9::_3(x),
                    Either8::_4(x) => Either9::_4(x),
                    Either8::_5(x) => Either9::_5(x),
                    Either8::_6(x) => Either9::_6(x),
                    Either8::_7(x) => Either9::_7(x),
                    Either8::_8(x) => Either9::_8(x),
                },
                Either::Right(x) => Either9::_9(x),
            })
    }

    fn alt(
        self,
    ) -> impl Parser<
        In,
        Either9<Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9>,
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
        ),
    > {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8) = self;

        (a0, a1, a2, a3, a4, a5, a6, a7)
            .alt()
            .alt(a8)
            .map_err(|((a1, a2, a3, a4, a5, a6, a7, a8), a9)| (a1, a2, a3, a4, a5, a6, a7, a8, a9))
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either8::_1(x) => Either9::_1(x),
                    Either8::_2(x) => Either9::_2(x),
                    Either8::_3(x) => Either9::_3(x),
                    Either8::_4(x) => Either9::_4(x),
                    Either8::_5(x) => Either9::_5(x),
                    Either8::_6(x) => Either9::_6(x),
                    Either8::_7(x) => Either9::_7(x),
                    Either8::_8(x) => Either9::_8(x),
                },
                Either::Right(x) => Either9::_9(x),
            })
    }

    fn or(
        self,
    ) -> impl Parser<
        In,
        (
            Option<(In, Out1)>,
            Option<(In, Out2)>,
            Option<(In, Out3)>,
            Option<(In, Out4)>,
            Option<(In, Out5)>,
            Option<(In, Out6)>,
            Option<(In, Out7)>,
            Option<(In, Out8)>,
            Option<(In, Out9)>,
        ),
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
        ),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8) = self;
        a0.or(a1)
            .or(a2)
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
            .map_err(|((a, b), y)| (a, b, y))
            .or(a3)
            .map(|(a, d)| match a {
                Some((_, (a, b, c))) => (a, b, c, d),
                None => (None, None, None, d),
            })
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .or(a4)
            .map(|(a, e)| match a {
                Some((_, (a, b, c, d))) => (a, b, c, d, e),
                None => (None, None, None, None, e),
            })
            .map_err(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
            .or(a5)
            .map(|(a, f)| match a {
                Some((_, (a, b, c, d, e))) => (a, b, c, d, e, f),
                None => (None, None, None, None, None, f),
            })
            .map_err(|((a1, a2, a3, a4, a5), a6)| (a1, a2, a3, a4, a5, a6))
            .or(a6)
            .map(|(a, g)| match a {
                Some((_, (a, b, c, d, e, f))) => (a, b, c, d, e, f, g),
                None => (None, None, None, None, None, None, g),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6), a7)| (a1, a2, a3, a4, a5, a6, a7))
            .or(a7)
            .map(|(a, h)| match a {
                Some((_, (a, b, c, d, e, f, g))) => (a, b, c, d, e, f, g, h),
                None => (None, None, None, None, None, None, None, h),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6, a7), a8)| (a1, a2, a3, a4, a5, a6, a7, a8))
            .or(a8)
            .map(|(a, j)| match a {
                Some((_, (a, b, c, d, e, f, g, h))) => (a, b, c, d, e, f, g, h, j),
                None => (None, None, None, None, None, None, None, None, j),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6, a7, a8), a9)| (a1, a2, a3, a4, a5, a6, a7, a8, a9))
    }
}

impl<
        In,
        Out1,
        Out2,
        Out3,
        Out4,
        Out5,
        Out6,
        Out7,
        Out8,
        Out9,
        Out10,
        Error1,
        Error2,
        Error3,
        Error4,
        Error5,
        Error6,
        Error7,
        Error8,
        Error9,
        Error10,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
    >
    ParserSugar<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9, Out10),
        Either10<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8, Error9, Error10>,
        Either10<Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9, Out10>,
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
            Error10,
        ),
    > for (P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)
where
    P1: Parser<In, Out1, Error1>,
    P2: Parser<In, Out2, Error2>,
    P3: Parser<In, Out3, Error3>,
    P4: Parser<In, Out4, Error4>,
    P5: Parser<In, Out5, Error5>,
    P6: Parser<In, Out6, Error6>,
    P7: Parser<In, Out7, Error7>,
    P8: Parser<In, Out8, Error8>,
    P9: Parser<In, Out9, Error9>,
    P10: Parser<In, Out10, Error10>,
    In: Parsable<Error1>
        + Parsable<Error2>
        + Parsable<Error3>
        + Parsable<Error4>
        + Parsable<Error5>
        + Parsable<Error6>
        + Parsable<Error7>
        + Parsable<Error8>
        + Parsable<Error9>
        + Parsable<Error10>
        + Parsable<(Error1, Error2)>
        + Parsable<(Error1, Error2, Error3)>
        + Parsable<(Error1, Error2, Error3, Error4)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6)>
        + Parsable<(Error1, Error2, Error3, Error4, Error5, Error6, Error7)>
        + Parsable<(
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
        )> + Parsable<(
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
        )> + Parsable<(
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
            Error10,
        )> + Parsable<Either<Error1, Error2>>
        + Parsable<Either3<Error1, Error2, Error3>>
        + Parsable<Either4<Error1, Error2, Error3, Error4>>
        + Parsable<Either5<Error1, Error2, Error3, Error4, Error5>>
        + Parsable<Either6<Error1, Error2, Error3, Error4, Error5, Error6>>
        + Parsable<Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>>
        + Parsable<Either8<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8>>
        + Parsable<Either9<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8, Error9>>
        + Parsable<
            Either10<
                Error1,
                Error2,
                Error3,
                Error4,
                Error5,
                Error6,
                Error7,
                Error8,
                Error9,
                Error10,
            >,
        > + Parsable<((Error1, Error2), Error3)>
        + Parsable<((Error1, Error2, Error3), Error4)>
        + Parsable<((Error1, Error2, Error3, Error4), Error5)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5), Error6)>
        + Parsable<((Error1, Error2, Error3, Error4, Error5, Error6), Error7)>
        + Parsable<(
            (Error1, Error2, Error3, Error4, Error5, Error6, Error7),
            Error8,
        )> + Parsable<(
            (
                Error1,
                Error2,
                Error3,
                Error4,
                Error5,
                Error6,
                Error7,
                Error8,
            ),
            Error9,
        )> + Parsable<(
            (
                Error1,
                Error2,
                Error3,
                Error4,
                Error5,
                Error6,
                Error7,
                Error8,
                Error9,
            ),
            Error10,
        )> + Parsable<Either<Either<Error1, Error2>, Error3>>
        + Parsable<Either<Either3<Error1, Error2, Error3>, Error4>>
        + Parsable<Either<Either4<Error1, Error2, Error3, Error4>, Error5>>
        + Parsable<Either<Either5<Error1, Error2, Error3, Error4, Error5>, Error6>>
        + Parsable<Either<Either6<Error1, Error2, Error3, Error4, Error5, Error6>, Error7>>
        + Parsable<Either<Either7<Error1, Error2, Error3, Error4, Error5, Error6, Error7>, Error8>>
        + Parsable<
            Either<Either8<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8>, Error9>,
        > + Parsable<
            Either<
                Either9<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8, Error9>,
                Error10,
            >,
        >,
    Error1: Clone,
    Error2: Clone,
    Error3: Clone,
    Error4: Clone,
    Error5: Clone,
    Error6: Clone,
    Error7: Clone,
    Error8: Clone,
    Error9: Clone,
    Error10: Clone,
{
    fn seq(
        self,
    ) -> impl Parser<
        In,
        (Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9, Out10),
        Either10<Error1, Error2, Error3, Error4, Error5, Error6, Error7, Error8, Error9, Error10>,
    > {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) = self;

        (a0, a1, a2, a3, a4, a5, a6, a7, a8)
            .seq()
            .seq(a9)
            .map(|((a1, a2, a3, a4, a5, a6, a7, a8, a9), a10)| {
                (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10)
            })
            .map_err(|x| match x {
                Either::Left(x) => match x {
                    Either9::_1(x) => Either10::_1(x),
                    Either9::_2(x) => Either10::_2(x),
                    Either9::_3(x) => Either10::_3(x),
                    Either9::_4(x) => Either10::_4(x),
                    Either9::_5(x) => Either10::_5(x),
                    Either9::_6(x) => Either10::_6(x),
                    Either9::_7(x) => Either10::_7(x),
                    Either9::_8(x) => Either10::_8(x),
                    Either9::_9(x) => Either10::_9(x),
                },
                Either::Right(x) => Either10::_10(x),
            })
    }

    fn alt(
        self,
    ) -> impl Parser<
        In,
        Either10<Out1, Out2, Out3, Out4, Out5, Out6, Out7, Out8, Out9, Out10>,
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
            Error10,
        ),
    > {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) = self;

        (a0, a1, a2, a3, a4, a5, a6, a7, a8)
            .alt()
            .alt(a9)
            .map_err(|((a1, a2, a3, a4, a5, a6, a7, a8, a9), a10)| {
                (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10)
            })
            .map(|x| match x {
                Either::Left(x) => match x {
                    Either9::_1(x) => Either10::_1(x),
                    Either9::_2(x) => Either10::_2(x),
                    Either9::_3(x) => Either10::_3(x),
                    Either9::_4(x) => Either10::_4(x),
                    Either9::_5(x) => Either10::_5(x),
                    Either9::_6(x) => Either10::_6(x),
                    Either9::_7(x) => Either10::_7(x),
                    Either9::_8(x) => Either10::_8(x),
                    Either9::_9(x) => Either10::_9(x),
                },
                Either::Right(x) => Either10::_10(x),
            })
    }

    fn or(
        self,
    ) -> impl Parser<
        In,
        (
            Option<(In, Out1)>,
            Option<(In, Out2)>,
            Option<(In, Out3)>,
            Option<(In, Out4)>,
            Option<(In, Out5)>,
            Option<(In, Out6)>,
            Option<(In, Out7)>,
            Option<(In, Out8)>,
            Option<(In, Out9)>,
            Option<(In, Out10)>,
        ),
        (
            Error1,
            Error2,
            Error3,
            Error4,
            Error5,
            Error6,
            Error7,
            Error8,
            Error9,
            Error10,
        ),
    >
    where
        In: Clone,
    {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) = self;
        

        a0
            .or(a1)
            .or(a2)
            .map(|(a, c)| match a {
                Some((_, (a, b))) => (a, b, c),
                None => (None, None, c),
            })
            .map_err(|((a, b), y)| (a, b, y))
            .or(a3)
            .map(|(a, d)| match a {
                Some((_, (a, b, c))) => (a, b, c, d),
                None => (None, None, None, d),
            })
            .map_err(|((a1, a2, a3), a4)| (a1, a2, a3, a4))
            .or(a4)
            .map(|(a, e)| match a {
                Some((_, (a, b, c, d))) => (a, b, c, d, e),
                None => (None, None, None, None, e),
            })
            .map_err(|((a1, a2, a3, a4), a5)| (a1, a2, a3, a4, a5))
            .or(a5)
            .map(|(a, f)| match a {
                Some((_, (a, b, c, d, e))) => (a, b, c, d, e, f),
                None => (None, None, None, None, None, f),
            })
            .map_err(|((a1, a2, a3, a4, a5), a6)| (a1, a2, a3, a4, a5, a6))
            .or(a6)
            .map(|(a, g)| match a {
                Some((_, (a, b, c, d, e, f))) => (a, b, c, d, e, f, g),
                None => (None, None, None, None, None, None, g),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6), a7)| (a1, a2, a3, a4, a5, a6, a7))
            .or(a7)
            .map(|(a, h)| match a {
                Some((_, (a, b, c, d, e, f, g))) => (a, b, c, d, e, f, g, h),
                None => (None, None, None, None, None, None, None, h),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6, a7), a8)| (a1, a2, a3, a4, a5, a6, a7, a8))
            .or(a8)
            .map(|(a, j)| match a {
                Some((_, (a, b, c, d, e, f, g, h))) => (a, b, c, d, e, f, g, h, j),
                None => (None, None, None, None, None, None, None, None, j),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6, a7, a8), a9)| (a1, a2, a3, a4, a5, a6, a7, a8, a9))
            .or(a9)
            .map(|(a, k)| match a {
                Some((_, (a, b, c, d, e, f, g, h, j))) => (a, b, c, d, e, f, g, h, j, k),
                None => (None, None, None, None, None, None, None, None, None, k),
            })
            .map_err(|((a1, a2, a3, a4, a5, a6, a7, a8, a9), a10)| {
                (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10)
            })
    }
}
