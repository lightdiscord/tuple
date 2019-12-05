use super::{Tuple, tuple, tuple_ty};

macro_rules! implement {
    (
        $(
            $_:ident($($T:ident($idx:tt)),*)
        ),*
    ) => {
        $(
            impl<$($T),*> From<($($T,)*)> for tuple_ty!($($T),*) {
                fn from(_source: ($($T,)*)) -> Self {
                    tuple!($(_source.$idx),*)
                }
            }
        )*
    }
}

implement! {
    Tuple0(),
    Tuple1(A(0)),
    Tuple2(A(0), B(1)),
    Tuple3(A(0), B(1), C(2)),
    Tuple4(A(0), B(1), C(2), D(3)),
    Tuple5(A(0), B(1), C(2), D(3), E(4)),
    Tuple6(A(0), B(1), C(2), D(3), E(4), F(5)),
    Tuple7(A(0), B(1), C(2), D(3), E(4), F(5), G(6)),
    Tuple8(A(0), B(1), C(2), D(3), E(4), F(5), G(6), H(7)),
    Tuple9(A(0), B(1), C(2), D(3), E(4), F(5), G(6), H(7), I(8)),
    Tuple10(A(0), B(1), C(2), D(3), E(4), F(5), G(6), H(7), I(8), J(9)),
    Tuple11(A(0), B(1), C(2), D(3), E(4), F(5), G(6), H(7), I(8), J(9), K(10)),
    Tuple12(A(0), B(1), C(2), D(3), E(4), F(5), G(6), H(7), I(8), J(9), K(10), L(11))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_trait() {
        assert_eq!(tuple!(true, 1, "coucou", 5.0), (true, 1, "coucou", 5.0).into());
    }
}