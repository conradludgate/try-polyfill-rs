use std::ops::ControlFlow;

pub trait Try {
    type Break;
    type Continue;
    fn branch(self) -> ControlFlow<Self::Break, Self::Continue>;
    fn from_break(b: Self::Break) -> Self;
    fn from_continue(c: Self::Continue) -> Self;
}

impl<T, E> Try for Result<T, E> {
    type Break = E;
    type Continue = T;

    fn branch(self) -> ControlFlow<Self::Break, Self::Continue> {
        match self {
            Ok(t) => ControlFlow::Continue(t),
            Err(e) => ControlFlow::Break(e),
        }
    }

    fn from_break(e: E) -> Self {
        Err(e)
    }
    fn from_continue(t: T) -> Self {
        Ok(t)
    }
}

impl<T> Try for Option<T> {
    type Break = ();
    type Continue = T;

    fn branch(self) -> ControlFlow<Self::Break, Self::Continue> {
        match self {
            Some(t) => ControlFlow::Continue(t),
            None => ControlFlow::Break(()),
        }
    }

    fn from_break(_: ()) -> Self {
        None
    }
    fn from_continue(t: T) -> Self {
        Some(t)
    }
}

#[cfg(feature = "macro")]
pub use try_polyfill_macro::*;

#[doc(hidden)]
pub mod __private {
    pub use std::ops::ControlFlow;

    use crate::Try;

    pub fn branch<T: Try>(t: T) -> ControlFlow<T, T::Continue> {
        match t.branch() {
            ControlFlow::Continue(c) => ControlFlow::Continue(c),
            ControlFlow::Break(b) => ControlFlow::Break(T::from_break(b)),
        }
    }
}
