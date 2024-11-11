/// Create a new set collection and insert elements.
///
/// Example:
///
/// ```
/// # use ::set::*;
/// let s = set!(1, 2, 3);
/// ```
///
/// Example with multiple lines:
///
/// ```
/// # use ::set::*;
/// let s = set!(
///     1,
///     2,
///     3,
/// );
/// ```
///
/// Equivalent Rust standard code:
///
/// ```
/// # use std::collections::HashSet;
/// let mut s = HashSet::new();
/// s.insert(1);
/// s.insert(2);
/// s.insert(3);
/// ```
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! set {
    ( $( $e:expr ),* $(,)?) => {
        {
            let mut s = ::std::collections::HashSet::new();
            $(
                s.insert($e);
            )*
            s
        }
    };
}

#[cfg(test)]
mod test {

    mod one_line {
        use std::collections::HashSet;

        #[test]
        fn trailing_comma() {
            let x = set!(1, 2, 3,);
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }

        #[test]
        fn no_trailing_comma() {
            let x = set!(1, 2, 3);
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }
    }

    mod multiple_lines {
        use std::collections::HashSet;

        #[test]
        fn trailing_comma() {
            let x = set!(
                1,
                2,
                3,
            );
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }

        #[test]
        fn no_trailing_comma() {
            let x = set!(
                1,
                2,
                3
            );
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }
    }

}
