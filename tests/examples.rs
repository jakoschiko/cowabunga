use std::borrow::Cow;

use cowabunga::Cowabunga;

#[derive(Clone, Debug, PartialEq)]
struct Foo<'a> {
    x: u8,
    y: Cow<'a, str>,
}

impl<'a> Cowabunga for Foo<'a> {
    type Owned = Foo<'static>;

    type Borrowed<'b> = Foo<'b>
    where
        Self: 'b;

    fn to_borrowed<'b>(&'b self) -> Self::Borrowed<'b> {
        Foo {
            x: cowabunga::to_borrowed(&self.x),
            y: cowabunga::to_borrowed(&self.y),
        }
    }

    fn to_owned(&self) -> Self::Owned {
        Foo {
            x: cowabunga::to_owned(&self.x),
            y: cowabunga::to_owned(&self.y),
        }
    }

    fn into_owned(self) -> Self::Owned {
        Foo {
            x: cowabunga::into_owned(self.x),
            y: cowabunga::into_owned(self.y),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Bar<'a>(Cow<'a, [u8]>, Cow<'a, str>);

impl<'a> Cowabunga for Bar<'a> {
    type Owned = Bar<'static>;

    type Borrowed<'b> = Bar<'b>
    where
        Self: 'b;

    fn to_borrowed<'b>(&'b self) -> Self::Borrowed<'b> {
        Bar(
            cowabunga::to_borrowed(&self.0),
            cowabunga::to_borrowed(&self.1),
        )
    }

    fn to_owned(&self) -> Self::Owned {
        Bar(cowabunga::to_owned(&self.0), cowabunga::to_owned(&self.1))
    }

    fn into_owned(self) -> Self::Owned {
        Bar(cowabunga::into_owned(self.0), cowabunga::into_owned(self.1))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum FooBar<'a> {
    Foo(Foo<'a>),
    Bar(Bar<'a>),
    Nope,
}

impl<'a> Cowabunga for FooBar<'a> {
    type Owned = FooBar<'static>;

    type Borrowed<'b> = FooBar<'b>
    where
        Self: 'b;

    fn to_borrowed(&self) -> Self::Borrowed<'_> {
        match self {
            FooBar::Foo(foo) => FooBar::Foo(cowabunga::to_borrowed(foo)),
            FooBar::Bar(bar) => FooBar::Bar(cowabunga::to_borrowed(bar)),
            FooBar::Nope => FooBar::Nope,
        }
    }

    fn to_owned(&self) -> Self::Owned {
        match self {
            FooBar::Foo(foo) => FooBar::Foo(cowabunga::to_owned(foo)),
            FooBar::Bar(bar) => FooBar::Bar(cowabunga::to_owned(bar)),
            FooBar::Nope => FooBar::Nope,
        }
    }

    fn into_owned(self) -> Self::Owned {
        match self {
            FooBar::Foo(foo) => FooBar::Foo(cowabunga::into_owned(foo)),
            FooBar::Bar(bar) => FooBar::Bar(cowabunga::into_owned(bar)),
            FooBar::Nope => FooBar::Nope,
        }
    }
}

#[test]
fn foo() {
    let foo = Foo {
        x: 42,
        y: Cow::Owned("test".to_string()),
    };

    assert_eq!(foo, cowabunga::to_borrowed(&foo));
    assert_eq!(foo, cowabunga::to_owned(&foo));
    assert_eq!(foo, cowabunga::into_owned(foo.clone()));
}

#[test]
fn bar() {
    let bar = Bar(
        Cow::Owned([17, 42, 73].to_vec()),
        Cow::Owned("test".to_string()),
    );

    assert_eq!(bar, cowabunga::to_borrowed(&bar));
    assert_eq!(bar, cowabunga::to_owned(&bar));
    assert_eq!(bar, cowabunga::into_owned(bar.clone()));
}

#[test]
fn foobar() {
    let foobar1 = FooBar::Foo(Foo {
        x: 42,
        y: Cow::Owned("test".to_string()),
    });
    let foobar2 = FooBar::Bar(Bar(
        Cow::Owned([17, 42, 73].to_vec()),
        Cow::Owned("test".to_string()),
    ));
    let foobar3 = FooBar::Nope;

    assert_eq!(foobar1, cowabunga::to_borrowed(&foobar1));
    assert_eq!(foobar1, cowabunga::to_owned(&foobar1));
    assert_eq!(foobar1, cowabunga::into_owned(foobar1.clone()));

    assert_eq!(foobar2, cowabunga::to_borrowed(&foobar2));
    assert_eq!(foobar2, cowabunga::to_owned(&foobar2));
    assert_eq!(foobar2, cowabunga::into_owned(foobar2.clone()));

    assert_eq!(foobar3, cowabunga::to_borrowed(&foobar3));
    assert_eq!(foobar3, cowabunga::to_owned(&foobar3));
    assert_eq!(foobar3, cowabunga::into_owned(foobar3.clone()));
}
