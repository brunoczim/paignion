use crate::{
    colist::{Cocons, Conil},
    list::{Cons, Nil},
};

pub trait ByRef<'this> {
    type Ref: 'this;

    fn by_ref(&'this self) -> Self::Ref;
}

pub trait ByMut<'this>: ByRef<'this> {
    type RefMut: 'this;

    fn by_mut(&'this mut self) -> Self::RefMut;
}

impl<'this, A> ByRef<'this> for Nil<A>
where
    A: 'this,
{
    type Ref = Nil<&'this A>;

    fn by_ref(&'this self) -> Self::Ref {
        Nil(&self.0)
    }
}

impl<'this, A> ByRef<'this> for Conil<A>
where
    A: 'this,
{
    type Ref = Conil<&'this A>;

    fn by_ref(&'this self) -> Self::Ref {
        match self.0 {}
    }
}

impl<'this, H, T> ByRef<'this> for Cons<H, T>
where
    H: 'this,
    T: ByRef<'this>,
{
    type Ref = Cons<&'this H, T::Ref>;

    fn by_ref(&'this self) -> Self::Ref {
        Cons { head: &self.head, tail: self.tail.by_ref() }
    }
}

impl<'this, H, T> ByRef<'this> for Cocons<H, T>
where
    H: 'this,
    T: ByRef<'this>,
{
    type Ref = Cocons<&'this H, T::Ref>;

    fn by_ref(&'this self) -> Self::Ref {
        match self {
            Cocons::Head(head) => Cocons::Head(head),
            Cocons::Tail(tail) => Cocons::Tail(tail.by_ref()),
        }
    }
}

impl<'this, A> ByMut<'this> for Nil<A>
where
    A: 'this,
{
    type RefMut = Nil<&'this mut A>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        Nil(&mut self.0)
    }
}

impl<'this, A> ByMut<'this> for Conil<A>
where
    A: 'this,
{
    type RefMut = Conil<&'this mut A>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        match self.0 {}
    }
}

impl<'this, H, T> ByMut<'this> for Cons<H, T>
where
    H: 'this,
    T: ByMut<'this>,
{
    type RefMut = Cons<&'this mut H, T::RefMut>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        Cons { head: &mut self.head, tail: self.tail.by_mut() }
    }
}

impl<'this, H, T> ByMut<'this> for Cocons<H, T>
where
    H: 'this,
    T: ByMut<'this>,
{
    type RefMut = Cocons<&'this mut H, T::RefMut>;

    fn by_mut(&'this mut self) -> Self::RefMut {
        match self {
            Cocons::Head(head) => Cocons::Head(head),
            Cocons::Tail(tail) => Cocons::Tail(tail.by_mut()),
        }
    }
}
