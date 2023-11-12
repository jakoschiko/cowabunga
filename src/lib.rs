pub trait Cowabunga {
    type Owned;

    type Borrowed<'b>
    where
        Self: 'b;

    fn to_borrowed(&self) -> Self::Borrowed<'_>;

    fn to_owned(&self) -> Self::Owned;

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
