#[cfg(derive_macros)]
#[macro_export]
use cys_span_derive;

/// A struct that has a known start index.
pub trait Started {
    fn start(&self) -> usize;
}

/// A struct that has a known end index.
pub trait Ended {
    fn end(&self) -> usize;
}

/// A started struct.
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

/// An ended struct.
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

/// A single token.
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

/// A struct with Span.
pub type Span<T> = Start<End<T>>;

/// With an optional prefix.
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
