/// A type that contains cow values that can be converted between beeing owned or borrowed.
///
/// The term "cow" refers to the copy-on-write technique. The Rust standard library implements
/// this with [`Cow`](std::borrow::Cow).
pub trait Cowabunga {
    /// A variant of `Self` that has only owned cow values.
    type Owned;

    /// A variant of `Self` that has only borrowed cow values.
    type Borrowed<'b>
    where
        Self: 'b;

    /// Returns an instance that contains only cow values borrowed from `self`.
    fn to_borrowed(&self) -> Self::Borrowed<'_>;

    /// Returns an instance that contains only owned cow values by cloning values from `self`.
    fn to_owned(&self) -> Self::Owned;

    /// Returns an instance that contains only owned cow values by cloning values from `self`
    /// if necessary.
    ///
    /// Cow values that are already owned are preserved.
    fn into_owned(self) -> Self::Owned;
}

pub fn to_borrowed<T: Cowabunga>(t: &T) -> T::Borrowed<'_> {
    t.to_borrowed()
}

pub fn to_owned<T: Cowabunga>(t: &T) -> T::Owned {
    t.to_owned()
}

pub fn into_owned<T: Cowabunga>(t: T) -> T::Owned {
    t.into_owned()
}

macro_rules! impl_cowabunga_for_copy {
    ($id:ident) => {
        impl crate::Cowabunga for $id {
            type Owned = $id;

            type Borrowed<'a> = $id
            where
                Self: 'a;

            fn to_borrowed(&self) -> Self::Borrowed<'_> {
                *self
            }

            fn to_owned(&self) -> Self::Owned {
                *self
            }

            fn into_owned(self) -> Self::Owned {
                self
            }
        }
    };
}

mod impl_primitives;

mod impl_std;
