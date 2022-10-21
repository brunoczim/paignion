use crate::{
    colist::{Cocons, Conil},
    function::FunctionMut,
    list::{Cons, Nil},
};

pub trait Map<F> {
    type Output;

    fn map(self, mapper: F) -> Self::Output;
}

impl<F> Map<F> for Nil {
    type Output = Self;

    fn map(self, _mapper: F) -> Self::Output {
        self
    }
}

impl<F> Map<F> for Conil {
    type Output = Self;

    fn map(self, _mapper: F) -> Self::Output {
        self
    }
}

impl<H, T, F> Map<F> for Cons<H, T>
where
    T: Map<F>,
    F: FunctionMut<H>,
{
    type Output = Cons<F::Output, T::Output>;

    fn map(self, mut mapper: F) -> Self::Output {
        Cons { head: mapper.call_mut(self.head), tail: self.tail.map(mapper) }
    }
}

impl<H, T, F> Map<F> for Cocons<H, T>
where
    T: Map<F>,
    F: FunctionMut<H>,
{
    type Output = Cocons<F::Output, T::Output>;

    fn map(self, mut mapper: F) -> Self::Output {
        match self {
            Cocons::Head(head) => Cocons::Head(mapper.call_mut(head)),
            Cocons::Tail(tail) => Cocons::Tail(tail.map(mapper)),
        }
    }
}

pub trait FoldLeft<A, F> {
    fn fold_left(self, accumulator: A, folder: F) -> A;
}

impl<A, F> FoldLeft<A, F> for Nil {
    fn fold_left(self, accumulator: A, _folder: F) -> A {
        accumulator
    }
}

impl<A, F> FoldLeft<A, F> for Conil {
    fn fold_left(self, accumulator: A, _folder: F) -> A {
        accumulator
    }
}

impl<H, T, A, F> FoldLeft<A, F> for Cons<H, T>
where
    F: FunctionMut<(A, H), Output = A>,
    T: FoldLeft<A, F>,
{
    fn fold_left(self, accumulator: A, mut folder: F) -> A {
        self.tail.fold_left(folder.call_mut((accumulator, self.head)), folder)
    }
}

impl<H, T, A, F> FoldLeft<A, F> for Cocons<H, T>
where
    F: FunctionMut<(A, H), Output = A>,
    T: FoldLeft<A, F>,
{
    fn fold_left(self, accumulator: A, mut folder: F) -> A {
        match self {
            Cocons::Head(head) => folder.call_mut((accumulator, head)),
            Cocons::Tail(tail) => tail.fold_left(accumulator, folder),
        }
    }
}

pub trait FoldRight<A, F> {
    fn fold_right(self, accumulator: A, folder: F) -> A;
}

impl<A, F> FoldRight<A, F> for Nil {
    fn fold_right(self, accumulator: A, _folder: F) -> A {
        accumulator
    }
}

impl<A, F> FoldRight<A, F> for Conil {
    fn fold_right(self, accumulator: A, _folder: F) -> A {
        accumulator
    }
}

impl<H, T, A, F> FoldRight<A, F> for Cons<H, T>
where
    F: FunctionMut<(H, A), Output = A>,
    T: for<'a> FoldRight<A, &'a mut F>,
{
    fn fold_right(self, accumulator: A, mut folder: F) -> A {
        let new_accumulator = self.tail.fold_right(accumulator, &mut folder);
        folder.call_mut((self.head, new_accumulator))
    }
}

impl<H, T, A, F> FoldRight<A, F> for Cocons<H, T>
where
    F: FunctionMut<(H, A), Output = A>,
    T: FoldRight<A, F>,
{
    fn fold_right(self, accumulator: A, mut folder: F) -> A {
        match self {
            Cocons::Head(head) => folder.call_mut((head, accumulator)),
            Cocons::Tail(tail) => tail.fold_right(accumulator, folder),
        }
    }
}