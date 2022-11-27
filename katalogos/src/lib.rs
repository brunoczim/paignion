pub mod coproduct;
pub mod function;
pub mod combinator;
pub mod by_ref;

pub trait IntoIterRef {
    type Item;
    type IntoIter<'item>: Iterator<Item = &'item Self::Item>
    where
        Self: 'item;

    fn iter<'item>(&'item self) -> Self::IntoIter<'item>;
}

impl<T, I> IntoIterRef for T
where
    for<'this> &'this T: IntoIterator<Item = &'this I>,
{
    type Item = I;
    type IntoIter<'item> = <&'item T as IntoIterator>::IntoIter where T: 'item;

    fn iter<'item>(&'item self) -> Self::IntoIter<'item> {
        self.into_iter()
    }
}

#[macro_export]
macro_rules! hiter {
    [(): $m:ty] => {
        ::std::iter::empty<$crate::coproduct::Conil<$m>>()
    };
    [($elem:expr $(, $elems:expr)*): $m:ty] => {
        ::std::iter::once($crate::coproduct::Cocons::Head($elem))
            .chain(
                $crate::hiter![($($elems),*): $m]
                .map($ccrate::coproduct::Cocons::Tail)
            )
    };
    [($($elems:expr,)*): $m:ty] => {
        $crate::hiter![($($elems),*): $m]
    };

    [] => {
        ::std::iter::empty<$crate::coproduct::Conil>()
    };
    [$elem:expr $(, $elems:expr)*] => {
        ::std::iter::once($crate::coproduct::Cocons::Head($elem))
            .chain(
                $crate::hiter![$($elems),*]
                .map($ccrate::coproduct::Cocons::Tail)
            )
    };
    [$($elems:expr,)*] => {
        $crate::hiter![$($elems),*]
    };
}

#[macro_export]
macro_rules! hvec {
    [($($elems:expr),*): $m:ty] => {
        $crate::hiter![($($elems),*): $m].collect::<Vec<_>>()
    };
    [($($elems:expr,)*): $m:ty] => {
        $crate::hvec![($($elems),*): $m]
    };

    [$($elems:expr),*] => {
        $crate::hiter![$($elems),*].collect::<Vec<_>>()
    };
    [$($elems:expr,)*] => {
        $crate::hvec![$($elems),*]
    };
}

#[macro_export]
macro_rules! harray {
    [($($elems:expr),*): $m:ty] => {
        $crate::harray![
            @done_in = []
            @done_out = []
            @buf = []
            @todo = [$($elems),*]
            @count = [0]
            @ty  = [$crate::coproduct::Conil<$m>]
        ]
    };

    [$($elems:expr),*] => {
        $crate::harray![
            @done_in = []
            @done_out = []
            @buf = []
            @todo = [$($elems),*]
            @count = [0]
            @ty  = [$crate::coproduct::Conil<_>]
        ]
    };

    [$($elems:expr,)*] => {
        $crate::harray![$($elems),*]
    };

    [($($elems:expr,)*): $m:ty] => {
        $crate::harray![($($elems),*): $m]
    };

    [
        @done_in = []
        @done_out = [$($done:expr),*]
        @buf = []
        @todo = [$elem:expr $(,$elems:expr)*]
        @count = [$n:expr]
        @ty = [$ty:ty]
    ] => {
        $crate::harray![
            @done_in = [$($done),*]
            @done_out = []
            @buf = [$crate::coproduct::Cocons::Head($elem)]
            @todo = [$($elems),*]
            @count = [$n + 1]
            @ty = [$crate::coproduct::Cocons<_, $ty>]
        ]
    };


    [
        @done_in = []
        @done_out = [$($done:expr),*]
        @buf = []
        @todo = []
        @count = [$n:expr]
        @ty = [$ty:ty]
    ] => {{
        let array: [$ty; $n] = [$($done),*];
        array
    }};

    [
        @done_in = []
        @done_out = [$($done_out:expr),*]
        @buf = [$buf:expr]
        @todo = [$($elems:expr),*]
        @count = [$n:expr]
        @ty = [$ty:ty]
    ] => {
        $crate::harray![
            @done_in = []
            @done_out = [$($done_out,)* $buf]
            @buf = []
            @todo = [$($elems),*]
            @count = [$n]
            @ty = [$ty]
        ]
    };

    [
        @done_in = [$done:expr $(, $done_in:expr)*]
        @done_out = [$($done_out:expr),*]
        @buf = [$buf:expr]
        @todo = [$($elems:expr),*]
        @count = [$n:expr]
        @ty = [$ty:ty]
    ] => {
        $crate::harray![
            @done_in = [$($done_in),*]
            @done_out = [$($done_out,)* $done]
            @buf = [$crate::coproduct::Cocons::Tail($buf)]
            @todo = [$($elems),*]
            @count = [$n]
            @ty = [$ty]
        ]
    };
}

#[macro_export]
macro_rules! HArray {
    [($($tys:ty),*): $m:ty] => {
        $crate::HArray![
            @revert = [$($tys),*]
            @done = []
            @meta = [$m]
        ]
    };

    [$($tys:ty),*] => {
        $crate::HArray![
            @revert = [$($tys),*]
            @done = []
            @meta = []
        ]
    };

    [
        @revert = [$input:ty $(,$inputs:ty)*]
        @done = [$($tys:ty),*]
        @meta = [$($tt:tt)*]
    ] => {
        $crate::HArray![
            @revert = [$($inputs),*]
            @done = [$input $(,$tys)*]
            @meta = [$($tt)*]
        ]
    };

    [
        @revert = []
        @done = [$($tys:ty),*]
        @meta = []
    ] => {
        $crate::HArray![
            @count = [0]
            @buf = [$($tys),*]
            @done = [$crate::coproduct::Conil<_>]
        ]
    };

    [
        @revert = []
        @done = [$($tys:ty),*]
        @meta = [$m:ty]
    ] => {
        $crate::HArray![
            @count = [0]
            @buf = [$($tys),*]
            @done = [$crate::coproduct::Conil<$m>]
        ]
    };

    [
        @count = [$n:expr]
        @buf = [$ty:ty $(,$tys:ty)*]
        @done = [$done:ty]
    ] => {
        $crate::HArray![
            @count = [$n + 1]
            @buf = [$($tys),*]
            @done = [$crate::coproduct::Cocons<$ty, $done>]
        ]
    };

    [@count = [$n:expr] @buf = []  @done = [$ty:ty]] => {
        [$ty; $n]
    };
}

#[macro_export]
macro_rules! Coproduct {
    [(): $m:ty] => { $crate::coproduct::Conil<$m> };
    [($elem:ty $(, $elems:ty)*): $m:ty] => {
        $crate::coproduct::Cocons<$elem, $crate::Coproduct![($($elems),*): $m]>
    };
    [($($elems:ty,)*): $m:ty] => {
        $crate::Coproduct![($($elems),*): $m]
    };

    [] => { $crate::coproduct::Conil };
    [$elem:ty $(, $elems:ty)*] => {
        $crate::coproduct::Cocons<$elem, $crate::Coproduct![$($elems),*]>
    };
    [$($elems:ty,)*] => {
        $crate::Coproduct![$($elems),*]
    };
}

#[cfg(test)]
mod test {
    #[allow(dead_code)]
    const UNIT_META_LIST: HArray![(&str, i32): ()] = harray!["a", 2];

    #[allow(dead_code)]
    const F64_META_LIST: HArray![(bool, &str, i32): f64] =
        harray![false, "a", 2];

    #[allow(dead_code)]
    const MANUAL_F64_META_LIST: [Coproduct![(bool, &str, i32): f64]; 3] =
        harray![false, "a", 2];
}
