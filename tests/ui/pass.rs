use std::marker::PhantomData;

use cys_span::*;
use cys_span_derive::*;

#[derive(Started, Ended)]
struct SingleField(Span<()>);

#[derive(Started, Ended)]
struct NamedTuple<T>(Start<PhantomData<T>>, End<()>);

#[derive(Started, Ended)]
struct Struct<T> {
    a: Start<PhantomData<T>>,
    b: End<PhantomData<T>>,
}

enum Enum<T> {
    Single(NamedTuple<T>),
    Unnamed(Start<()>, End<()>),
    Named {
        start: Start<()>,
        end: End<()>,
    },
}

fn main() {}
