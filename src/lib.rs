#[derive(Debug)]
pub struct Tuple<A, B>(pub A, pub B);

#[macro_export]
macro_rules! tuple {
    ($value:expr) => { Tuple($value, ()) };

    () => { tuple!(()) };

    ($value:expr, $($tree:tt),*) => {
        Tuple($value, tuple!($($tree),*))
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn assert_debug() {
        let a = tuple!();
        let b = tuple!(true);
        let c = tuple!(1, "two", 3.0, false);

        assert_eq!(format!("{:?}", a), "Tuple((), ())");
        assert_eq!(format!("{:?}", b), "Tuple(true, ())");
        assert_eq!(format!("{:?}", c), "Tuple(1, Tuple(\"two\", Tuple(3.0, Tuple(false, ()))))");
    }
}