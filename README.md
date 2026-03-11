# tuple-impl

Simple macro to implement traits for tuples of various sizes. 

# Example

```rust
use tuple_impl::tuple_impl;

// A simple trait to filter some data.
pub trait Filter {
    fn filter(&self, what: usize) -> bool;
}

// Implement Filter for tuples of any size, where all elements implement Filter.
macro_rules! impl_filter {
    ($($T:ident )+) => {
        #[allow(non_snake_case, missing_docs)]
        impl<$($T),+> Filter for ($($T,)+)
        where
            $(
                $T: Filter,
            )+
        {
            fn filter(&self, what: usize) -> bool
            {
                let ($(ref $T,)+) = *self;
                $(
                    if !<$T as Filter>::filter($T, what) {
                        return false;
                    }
                )+
                true
            }
        }
    };
}

// Implement Filter for tuples of size 1 to 26.
tuple_impl!(impl_filter => A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
```

## Author

Peter Vrba <phonkee@phonkee.eu>