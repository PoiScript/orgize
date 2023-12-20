use nom::{
    error::{ErrorKind, ParseError},
    Compare, CompareResult, Err, FindSubstring, IResult, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Needed, Offset, Slice,
};
use std::{
    ops::{Deref, Range, RangeFrom, RangeFull, RangeTo},
    str::{CharIndices, Chars},
};

use super::{
    combinator::{token, GreenElement},
    SyntaxKind,
};
use crate::config::ParseConfig;

/// A custom Input struct
///
/// It helps us to pass the `ParseConfig` all the way down to each parsers
#[derive(Clone, Copy, Debug)]
pub struct Input<'a> {
    pub(crate) s: &'a str,
    pub(crate) c: &'a ParseConfig,
}

impl<'a> Input<'a> {
    #[inline]
    pub(crate) fn of(&self, i: &'a str) -> Input<'a> {
        Input { s: i, c: self.c }
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.s
    }

    #[inline]
    pub fn token(&self, kind: SyntaxKind) -> GreenElement {
        token(kind, self.s)
    }

    #[inline]
    pub fn text_token(&self) -> GreenElement {
        token(SyntaxKind::TEXT, self.s)
    }

    #[inline]
    pub fn ws_token(&self) -> GreenElement {
        token(SyntaxKind::WHITESPACE, self.s)
    }

    #[inline]
    pub fn nl_token(&self) -> GreenElement {
        token(SyntaxKind::NEW_LINE, self.s)
    }
}

impl<'a> Deref for Input<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &'a str {
        self.s
    }
}

impl<'a> From<(&'a str, &'a ParseConfig)> for Input<'a> {
    fn from(value: (&'a str, &'a ParseConfig)) -> Self {
        Input {
            s: value.0,
            c: value.1,
        }
    }
}

impl<'a> Slice<Range<usize>> for Input<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        self.of(self.s.slice(range))
    }
}

impl<'a> Slice<RangeTo<usize>> for Input<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.of(self.s.slice(range))
    }
}

impl<'a> Slice<RangeFrom<usize>> for Input<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.of(self.s.slice(range))
    }
}

impl<'a> Slice<RangeFull> for Input<'a> {
    fn slice(&self, range: RangeFull) -> Self {
        self.of(self.s.slice(range))
    }
}

impl<'a, 'b> FindSubstring<&'b str> for Input<'a> {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.s.find(substr)
    }
}

impl<'a, 'b> Compare<&'b str> for Input<'a> {
    #[inline]
    fn compare(&self, t: &'b str) -> CompareResult {
        self.s.compare(t)
    }

    #[inline]
    fn compare_no_case(&self, t: &'b str) -> CompareResult {
        self.s.compare_no_case(t)
    }
}

impl<'a> InputLength for Input<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<'a> InputIter for Input<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;
    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.s.char_indices()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.s.chars()
    }
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s.position(predicate)
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.s.slice_index(count)
    }
}

impl<'a> InputTake for Input<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        let s = self.s.take(count);
        self.of(s)
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (l, r) = self.s.take_split(count);
        (self.of(l), self.of(r))
    }
}

impl<'a> InputTakeAtPosition for Input<'a> {
    type Item = char;

    #[inline]
    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.s.split_at_position::<_, (&str, ErrorKind)>(predicate) {
            Ok((l, r)) => Ok((self.of(l), self.of(r))),
            Err(Err::Error((i, kind))) => Err(Err::Error(E::from_error_kind(self.of(i), kind))),
            Err(Err::Failure((i, kind))) => Err(Err::Failure(E::from_error_kind(self.of(i), kind))),
            Err(Err::Incomplete(x)) => Err(Err::Incomplete(x)),
        }
    }

    #[inline]
    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self
            .s
            .split_at_position1::<_, (&str, ErrorKind)>(predicate, e)
        {
            Ok((l, r)) => Ok((self.of(l), self.of(r))),
            Err(Err::Error((i, kind))) => Err(Err::Error(E::from_error_kind(self.of(i), kind))),
            Err(Err::Failure((i, kind))) => Err(Err::Failure(E::from_error_kind(self.of(i), kind))),
            Err(Err::Incomplete(x)) => Err(Err::Incomplete(x)),
        }
    }

    #[inline]
    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self
            .s
            .split_at_position_complete::<_, (&str, ErrorKind)>(predicate)
        {
            Ok((l, r)) => Ok((self.of(l), self.of(r))),
            Err(Err::Error((i, kind))) => Err(Err::Error(E::from_error_kind(self.of(i), kind))),
            Err(Err::Failure((i, kind))) => Err(Err::Failure(E::from_error_kind(self.of(i), kind))),
            Err(Err::Incomplete(x)) => Err(Err::Incomplete(x)),
        }
    }

    #[inline]
    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self
            .s
            .split_at_position1_complete::<_, (&str, ErrorKind)>(predicate, e)
        {
            Ok((l, r)) => Ok((self.of(l), self.of(r))),
            Err(Err::Error((i, kind))) => Err(Err::Error(E::from_error_kind(self.of(i), kind))),
            Err(Err::Failure((i, kind))) => Err(Err::Failure(E::from_error_kind(self.of(i), kind))),
            Err(Err::Incomplete(x)) => Err(Err::Incomplete(x)),
        }
    }
}

impl<'a> Offset for Input<'a> {
    fn offset(&self, second: &Self) -> usize {
        self.s.offset(second.s)
    }
}
