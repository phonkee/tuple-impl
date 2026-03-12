/// A macro to generate implementations for tuples of varying sizes.
///
/// This macro takes a callable (like another macro) and a list of identifiers,
/// and generates implementations for tuples of all combinations of those identifiers.
///
/// Example usage:
///
/// ```rust
///  macro_rules! print_tuple {
///     () => {
///         println!("Empty tuple");
///     };
///     ($($name:ident)*) => {
///         println!("Tuple: {}", stringify!($($name)*));
///     };
///  }
///  tuple_impl!(print_tuple => A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
///
/// ```
///
/// Prints this code
///
/// ```
/// Tuple: A B
/// Tuple: A B C
/// Tuple: A B C D
/// Tuple: A B C D E
/// Tuple: A B C D E F
/// Tuple: A B C D E F G
/// Tuple: A B C D E F G H
/// Tuple: A B C D E F G H I
/// Tuple: A B C D E F G H I J
/// Tuple: A B C D E F G H I J K
/// Tuple: A B C D E F G H I J K L
/// Tuple: A B C D E F G H I J K L M
/// Tuple: A B C D E F G H I J K L M N
/// Tuple: A B C D E F G H I J K L M N O
/// Tuple: A B C D E F G H I J K L M N O P
/// Tuple: A B C D E F G H I J K L M N O P Q
/// Tuple: A B C D E F G H I J K L M N O P Q R
/// Tuple: A B C D E F G H I J K L M N O P Q R S
/// Tuple: A B C D E F G H I J K L M N O P Q R S T
/// Tuple: A B C D E F G H I J K L M N O P Q R S T U
/// Tuple: A B C D E F G H I J K L M N O P Q R S T U V
/// Tuple: A B C D E F G H I J K L M N O P Q R S T U V W
/// Tuple: A B C D E F G H I J K L M N O P Q R S T U V W X
/// Tuple: A B C D E F G H I J K L M N O P Q R S T U V W X Y
/// Tuple: A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
/// ```
///
#[macro_export]
macro_rules! tuple_impl(
  ($call:ident => $first:ident $second:ident $($id: ident)+) => (
    tuple_impl!(__impl $call => $first $second; $($id)+);
  );
  (__impl $call:ident => $($current:ident)*; $head:ident $($id: ident)+) => (
    $call!($($current)*);
    tuple_impl!(__impl $call => $($current)* $head; $($id)+);
  );
  (__impl $call:ident => $($current:ident)*; $head:ident) => (
    $call!($($current)*);
    $call!($($current)* $head);
  );
);

/// A convenience macro to generate implementations for tuples of all combinations of identifiers from A to Z.
#[macro_export]
macro_rules! tuple_impl_full {
    ($call:ident) => {
        $crate::tuple_impl!($call => A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tuple_impl() {
        macro_rules! print_tuple {
            () => {
                println!("Empty tuple");
            };
            ($($name:ident)*) => {
                println!("Tuple: {}", stringify!($($name)*));
            };
        }

        // Generate implementations for tuples of all combinations of identifiers from A to Z.
        tuple_impl!(print_tuple => A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

        // Generate implementations for tuples of all combinations of identifiers from A to Z using the convenience macro.
        tuple_impl_full!(print_tuple);
    }
}
