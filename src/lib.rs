#[derive(Debug)]
pub struct Tuple<A, B>(pub A, pub B);

#[macro_export]
macro_rules! tuple {
    ($value:expr) => { Tuple($value, ()) };

    () => { tuple!(()) };

    ($value:expr, $($other:expr),*) => {
        Tuple($value, tuple!($($other),*))
    };
}

#[macro_export]
macro_rules! tuple_ty {
    ($value:ty) => { Tuple<$value, ()> };

    () => { tuple_ty!(()) };

    ($value:ty, $($other:ty),*) => {
        Tuple<$value, tuple_ty!($($other),*)>
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn assert_debug() {
        let a: tuple_ty!() = tuple!();
        let b: tuple_ty!(bool) = tuple!(true);
        let c: tuple_ty!(usize, &str, f32, bool) = tuple!(1, "two", 3.0, false);

        assert_eq!(format!("{:?}", a), "Tuple((), ())");
        assert_eq!(format!("{:?}", b), "Tuple(true, ())");
        assert_eq!(format!("{:?}", c), "Tuple(1, Tuple(\"two\", Tuple(3.0, Tuple(false, ()))))");
    }
}