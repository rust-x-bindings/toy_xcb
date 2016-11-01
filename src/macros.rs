
/// Defines a reflective enum that can be iterated and also used
/// as key type for hash maps
///
/// #Examples
///
/// ```
/// #[macro_use]
/// extern crate clue;
///
/// use std::collections::HashMap;
///
/// iterable_key_enum! {
///     Direction =>
///         North,
///         East,
///         South,
///         West
/// }
///
/// fn main() {
///     let mut dirs = HashMap::new();
///     for d in Direction::variants() {
///         dirs.insert(d, format!("{:?}", d));
///     }
///
///     println!("there are {} directions:", Direction::num_variants());
///     for (_, dname) in dirs {
///         println!("  - {}", dname);
///     }
/// }
/// ```
#[macro_export]
macro_rules! iterable_key_enum {

    ( $name:ident => $( $val:ident ),* ) => {
        use std::slice::Iter;

        #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
        enum $name {
            $( $val ),*
        }

        impl $name {
            fn variants() -> Iter<'static, $name> {
                static VARIANTS: &'static [$name] =
                        &[$($name::$val),*];
                VARIANTS.iter()
            }

            fn num_variants() -> usize {
                [$($name::$val),*].len()
            }
        }
    };

}
