use std::marker::PhantomData;

/// Exact D-Bus argument-list signatures (outer tuples are not wire structs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodInfo {
    pub interface: &'static str,
    pub name: &'static str,
    pub input: &'static str,
    pub output: &'static str,
}

/// A typed descriptor: `I` and `O` are the argument and reply lists as tuples.
#[derive(Debug)]
pub struct Method<I, O> {
    info: MethodInfo,
    marker: PhantomData<fn(I) -> O>,
}

impl<I, O> Method<I, O> {
    pub(crate) const fn new(
        interface: &'static str,
        name: &'static str,
        input: &'static str,
        output: &'static str,
    ) -> Self {
        Self {
            info: MethodInfo {
                interface,
                name,
                input,
                output,
            },
            marker: PhantomData,
        }
    }

    pub const fn info(&self) -> MethodInfo {
        self.info
    }
}
