use std::borrow::{Borrow, Cow};

use crate::Cowabunga;

impl<'a, T> Cowabunga for Cow<'a, T>
where
    T: ToOwned + ?Sized + 'static,
{
    type Owned = Cow<'static, T>;
    type Borrowed<'b> = Cow<'b, T> where Self: 'b;

    fn to_borrowed(&self) -> <Self as Cowabunga>::Borrowed<'_> {
        let borrowed = match self {
            Cow::Borrowed(borrowed) => borrowed,
            Cow::Owned(owned) => owned.borrow(),
        };
        Cow::Borrowed(borrowed)
    }

    fn to_owned(&self) -> <Self as Cowabunga>::Owned {
        let owned = match self {
            Cow::Borrowed(borrowed) => <T as ToOwned>::to_owned(borrowed),
            Cow::Owned(owned) => <T as ToOwned>::to_owned(owned.borrow()),
        };
        Cow::Owned(owned)
    }

    fn into_owned(self) -> <Self as Cowabunga>::Owned {
        let owned = match self {
            Cow::Borrowed(borrowed) => <T as ToOwned>::to_owned(borrowed),
            Cow::Owned(owned) => owned,
        };
        Cow::Owned(owned)
    }
}
