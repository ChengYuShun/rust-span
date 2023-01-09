#[cfg(feature = "derive-macros")]
pub use cys_span_derive::*;

/// A struct that has a known start index.
pub trait Started {
    fn start(&self) -> usize;
}

/// A struct that has a known end index.
pub trait Ended {
    fn end(&self) -> usize;
}

/// A started struct.
#[derive(Clone, Copy, Debug, Hash)]
pub struct Start<I> {
    pub inner: I,
    pub start: usize,
}

impl<I> Started for Start<I> {
    #[inline]
    fn start(&self) -> usize {
        self.start
    }
}

impl<I: Ended> Ended for Start<I> {
    #[inline]
    fn end(&self) -> usize {
        self.inner.end()
    }
}

impl<I: PartialEq> PartialEq for Start<I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.inner.ne(&other.inner)
    }
}

impl<I: Eq> Eq for Start<I> {}

/// An ended struct.
#[derive(Clone, Copy, Debug, Hash)]
pub struct End<I> {
    pub inner: I,
    pub end: usize,
}

impl<I> Ended for End<I> {
    #[inline]
    fn end(&self) -> usize {
        self.end
    }
}

impl<I: Started> Started for End<I> {
    #[inline]
    fn start(&self) -> usize {
        self.inner.start()
    }
}

impl<I: PartialEq> PartialEq for End<I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.inner.ne(&other.inner)
    }
}

impl<I: Eq> Eq for End<I> {}

/// A single token.
#[derive(Clone, Copy, Debug, Hash)]
pub struct Index<I> {
    pub inner: I,
    pub index: usize,
}

impl<I> Started for Index<I> {
    #[inline]
    fn start(&self) -> usize {
        self.index
    }
}

impl<I> Ended for Index<I> {
    #[inline]
    fn end(&self) -> usize {
        self.index + 1
    }
}

impl<I: PartialEq> PartialEq for Index<I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.inner.ne(&other.inner)
    }
}

impl<I: Eq> Eq for Index<I> {}

/// A struct with Span.
#[derive(Clone, Copy, Debug, Hash)]
pub struct Span<T> {
    pub start: usize,
    pub end: usize,
    pub inner: T,
}

impl<T> Started for Span<T> {
    #[inline]
    fn start(&self) -> usize {
        self.start
    }
}

impl<T> Ended for Span<T> {
    #[inline]
    fn end(&self) -> usize {
        self.end
    }
}

impl<I: PartialEq> PartialEq for Span<I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.inner.ne(&other.inner)
    }
}

impl<I: Eq> Eq for Span<I> {}

/// A struct with extra data.
#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct Extra<T, E> {
    pub inner: T,
    pub extra: E,
}

impl<T: Started, E> Started for Extra<T, E> {
    #[inline(always)]
    fn start(&self) -> usize {
        self.inner.start()
    }
}

impl<T: Ended, E> Ended for Extra<T, E> {
    #[inline(always)]
    fn end(&self) -> usize {
        self.inner.end()
    }
}

/// With an optional prefix.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Prefix<A, B> {
    pub prefix: Option<A>,
    pub inner: B,
}

impl<A: Started, B: Started> Started for Prefix<A, B> {
    #[inline]
    fn start(&self) -> usize {
        self.prefix
            .as_ref()
            .map(|prefix| prefix.start())
            .unwrap_or(self.inner.start())
    }
}

impl<A, B: Ended> Ended for Prefix<A, B> {
    #[inline]
    fn end(&self) -> usize {
        self.inner.end()
    }
}

/// With an optional postfix.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Postfix<A, B> {
    pub inner: A,
    pub postfix: Option<B>,
}

impl<A: Started, B> Started for Postfix<A, B> {
    #[inline]
    fn start(&self) -> usize {
        self.inner.start()
    }
}

impl<A: Ended, B: Ended> Ended for Postfix<A, B> {
    #[inline]
    fn end(&self) -> usize {
        self.postfix
            .as_ref()
            .map(|postfix| postfix.end())
            .unwrap_or(self.inner.end())
    }
}

/// `AAA...B`
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Prepended<A, B> {
    pub precedings: Vec<A>,
    pub last: B,
}

impl<A: Started, B: Started> Started for Prepended<A, B> {
    #[inline]
    fn start(&self) -> usize {
        self.precedings
            .first()
            .map(|first| first.start())
            .unwrap_or(self.last.start())
    }
}

impl<A, B: Ended> Ended for Prepended<A, B> {
    #[inline]
    fn end(&self) -> usize {
        self.last.end()
    }
}

/// `ABBB...`
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Appended<A, B> {
    pub first: A,
    pub followings: Vec<B>,
}

impl<A: Started, B> Started for Appended<A, B> {
    #[inline]
    fn start(&self) -> usize {
        self.first.start()
    }
}

impl<A: Ended, B: Ended> Ended for Appended<A, B> {
    #[inline]
    fn end(&self) -> usize {
        self.followings
            .last()
            .map(|item| item.end())
            .unwrap_or(self.first.end())
    }
}

impl<T: Started> Started for (T,) {
    #[inline(always)]
    fn start(&self) -> usize {
        self.0.start()
    }
}

impl<T: Ended> Ended for (T,) {
    #[inline(always)]
    fn end(&self) -> usize {
        self.0.end()
    }
}

impl<T1: Started, T2> Started for (T1, T2) {
    #[inline(always)]
    fn start(&self) -> usize {
        self.0.start()
    }
}

impl<T1, T2: Ended> Ended for (T1, T2) {
    #[inline(always)]
    fn end(&self) -> usize {
        self.1.end()
    }
}

impl<T1: Started, T2, T3> Started for (T1, T2, T3) {
    #[inline(always)]
    fn start(&self) -> usize {
        self.0.start()
    }
}

impl<T1, T2, T3: Ended> Ended for (T1, T2, T3) {
    #[inline(always)]
    fn end(&self) -> usize {
        self.2.end()
    }
}

impl<T1: Started, T2, T3, T4> Started for (T1, T2, T3, T4) {
    #[inline(always)]
    fn start(&self) -> usize {
        self.0.start()
    }
}

impl<T1, T2, T3, T4: Ended> Ended for (T1, T2, T3, T4) {
    #[inline(always)]
    fn end(&self) -> usize {
        self.3.end()
    }
}

impl<T1: Started, T2, T3, T4, T5> Started for (T1, T2, T3, T4, T5) {
    #[inline(always)]
    fn start(&self) -> usize {
        self.0.start()
    }
}

impl<T1, T2, T3, T4, T5: Ended> Ended for (T1, T2, T3, T4, T5) {
    #[inline(always)]
    fn end(&self) -> usize {
        self.4.end()
    }
}

impl<T: Started> Started for Box<T> {
    #[inline]
    fn start(&self) -> usize {
        self.as_ref().start()
    }
}

impl<T: Ended> Ended for Box<T> {
    #[inline]
    fn end(&self) -> usize {
        self.as_ref().end()
    }
}

impl Started for usize {
    #[inline]
    fn start(&self) -> usize {
        *self
    }
}

impl Ended for usize {
    #[inline]
    fn end(&self) -> usize {
        *self
    }
}
