/// Use an existing set collection and insert elements.
///
/// Example:
///
/// ```
/// # use ::set::*;
/// # use ::std::collections::HashSet;
/// let mut s = HashSet::new();
/// set_insert!(s, 1, 2, 3);
/// ```
///
/// Equivalent Rust std code with method `insert``:
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
macro_rules! set_insert {
    ($set:ident, $( $element:expr ),* $(,)?) => {
        {
            $(
                $set.insert($element);
            )*
        }
    };
}

#[cfg(test)]
mod test {

    mod one_line {
        use std::collections::HashSet;

        #[test]
        fn trailing_comma() {
            let mut x = ::std::collections::HashSet::new();
            set_insert!(x, 1, 2, 3,);
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }

        #[test]
        fn no_trailing_comma() {
            let mut x = ::std::collections::HashSet::new();
            set_insert!(x, 1, 2, 3);
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }
    }

    mod multiple_lines {
        use std::collections::HashSet;

        #[test]
        fn trailing_comma() {
            let mut x = ::std::collections::HashSet::new();
            set_insert!(
                x,
                1,
                2,
                3,
            );
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }

        #[test]
        fn no_trailing_comma() {
            let mut x: std::collections::HashSet<i32> = ::std::collections::HashSet::new();
            set_insert!(
                x,
                1,
                2,
                3
            );
            assert_eq!(x, HashSet::from([1, 2, 3]));
        }
    }

}
