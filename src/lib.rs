//! A macro for generating structures for value lookup.
//!
//! # Examples
//! ```
//! #[macro_use]
//! extern crate reword;
//!
//! reword! {
//!     enum Lang: &'static str {
//!         Hi {
//!             EN_UK | EN_US = "Hi";
//!             NO = "Hei";
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut lang = Lang::NO;
//!     assert_eq!(lang.reword::<Hi>(), "Hei");
//!
//!     lang = Lang::EN_UK;
//!     assert_eq!(lang.reword::<Hi>(), "Hi");
//!
//!     lang = Lang::EN_US;
//!     assert_eq!(lang.reword::<Hi>(), "Hi");
//! }
//! ```

#![no_std]
#![forbid(unstable_features)]
#![deny(
    missing_debug_implementations,
    unused_import_braces,
    unused_qualifications,
    unsafe_code
)]

/// The macro used to generate the lookup types.
///
/// See the [crate level docs](index.html) for more information.
#[macro_export]
macro_rules! reword {
    (
        $(#[$outer:meta])*
        enum $reword:ident : $T:ty {
            $(#[$inner:meta])*
            $key:ident { $($($name:ident)|+ = $val:expr;)+ }
            $(
                $(#[$inner2:meta])*
                $key2:ident { $($($name2:ident)|+ = $val2:expr;)+ }
            )*
        }
    ) => {
        trait Word {
            $($(#[allow(non_upper_case_globals)] const $name: $T;)+)+
        }

        $(#[$outer])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        enum $reword {
            $($(#[allow(non_camel_case_types)] $name,)+)+
        }

        impl $reword {
            #[inline]
            fn reword<W: Word>(self) -> $T {
                match self {
                    $($($reword::$name => W::$name,)+)+
                }
            }
        }

        $(#[$inner])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        struct $key;

        impl Word for $key {
            $($(#[allow(non_upper_case_globals)] const $name: $T = $val;)+)+
        }

        $(
            $(#[$inner2])*
            #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
            struct $key2;

            impl Word for $key2 {
                $($(#[allow(non_upper_case_globals)] const $name2: $T = $val2;)+)+
            }
        )*
    };
    (
        $(#[$outer:meta])*
        pub enum $reword:ident : $T:ty {
            $(#[$inner:meta])*
            $key:ident { $($($name:ident)|+ = $val:expr;)+ }
            $(
                $(#[$inner2:meta])*
                $key2:ident { $($($name2:ident)|+ = $val2:expr;)+ }
            )*
        }
    ) => {
        pub trait Word {
            $($(#[allow(non_upper_case_globals)] const $name: $T;)+)+
        }

        $(#[$outer])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub enum $reword {
            $($(#[allow(non_camel_case_types)] $name,)+)+
        }

        impl $reword {
            #[inline]
            pub fn reword<W: Word>(self) -> $T {
                match self {
                    $($($reword::$name => W::$name,)+)+
                }
            }
        }

        $(#[$inner])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub struct $key;

        impl Word for $key {
            $($(#[allow(non_upper_case_globals)] const $name: $T = $val;)+)+
        }

        $(
            $(#[$inner2])*
            #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
            pub struct $key2;

            impl Word for $key2 {
                $($(#[allow(non_upper_case_globals)] const $name2: $T = $val2;)+)+
            }
        )*
    };
}
