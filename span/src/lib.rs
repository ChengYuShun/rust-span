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
