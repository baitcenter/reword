//! A macro for generating types that allows for fast lookup of `const` values at runtime.
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
//!         HowAreYou {
//!             EN_UK = "How are you?";
//!             EN_US = "How you doing?";
//!             NO = "Hvordan går det?";
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut lang = Lang::NO;
//!     assert_eq!(lang.reword::<Hi>(), "Hei");
//!
//!     lang = Lang::EN_UK;
//!     assert_eq!(lang.reword::<HowAreYou>(), "How are you?");
//!
//!     lang = Lang::EN_US;
//!     assert_eq!(lang.reword::<HowAreYou>(), "How you doing?");
//! }
//! ```
//!
//! # Visibility
//! The generated types are not exported out of its module by default.
//! Use `pub` before the `enum` to export it.
//!
//! ```
//! #[macro_use]
//! extern crate reword;
//!
//! mod example {
//!     reword! {
//!         pub enum Lang: &'static str {
//!             Hi {
//!                 EN_UK | EN_US = "Hi";
//!                 NO = "Hei";
//!             }
//!             HowAreYou {
//!                 EN_UK = "How are you?";
//!                 EN_US = "How you doing?";
//!                 NO = "Hvordan går det?";
//!             }
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut lang = example::Lang::NO;
//!     assert_eq!(lang.reword::<example::Hi>(), "Hei");
//!
//!     lang = example::Lang::EN_UK;
//!     assert_eq!(lang.reword::<example::HowAreYou>(), "How are you?");
//!
//!     lang = example::Lang::EN_US;
//!     assert_eq!(lang.reword::<example::HowAreYou>(), "How you doing?");
//! }
//! ```
//!
//! # Attributes
//! Attributes can be attached to both the `enum` and the `structs` generated.
//!
//! The `Copy`, `Clone`, `Debug`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, and `Hash` traits are
//! automatically derived for the types using the `derive` attribute.
//!
//! ```
//! #[macro_use]
//! extern crate reword;
//!
//! #[repr(C)]
//! #[derive(Debug, PartialEq)]
//! enum Code {
//!     Hex(u32),
//!     Rgb(u8, u8, u8),
//! }
//!
//! reword! {
//!     #[repr(C)]
//!     enum Color: Code {
//!         #[repr(C)]
//!         Red {
//!             Rgb = Code::Rgb(255, 0, 0);
//!             Hex = Code::Hex(0xff0000);
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut color = Color::Rgb;
//!     assert_eq!(color.reword::<Red>(), Code::Rgb(255, 0, 0));
//!
//!     color = Color::Hex;
//!     assert_eq!(color.reword::<Red>(), Code::Hex(0xff0000));
//! }
//! ```

#![no_std]
#![forbid(unstable_features)]
#![deny(missing_debug_implementations, unused_import_braces, unused_qualifications, unsafe_code)]

/// The macro used to generate the lookup types.
///
/// See the [crate level docs](index.html) for more information.
#[macro_export]
macro_rules! reword {
    (
        $(#[$OUTER:meta])*
        enum $ENUM:ident : $T:ty {
            $(#[$INNER:meta])*
            $KEY:ident { $($($NAME:ident)|+ = $VAL:expr;)+ }
            $(
                $(#[$INNER2:meta])*
                $KEY2:ident { $($($NAME2:ident)|+ = $VAL2:expr;)+ }
            )*
        }
    ) => {
        trait Word {
            $($(#[allow(non_upper_case_globals)] const $NAME: $T;)+)+
        }

        $(#[$OUTER])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        enum $ENUM {
            $($(#[allow(non_camel_case_types)] $NAME,)+)+
        }

        impl $ENUM {
            #[inline]
            fn reword<W: Word>(self) -> $T {
                match self {
                    $($($ENUM::$NAME => W::$NAME,)+)+
                }
            }
        }

        $(#[$INNER])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        struct $KEY;

        impl Word for $KEY {
            $($(#[allow(non_upper_case_globals)] const $NAME: $T = $VAL;)+)+
        }

        $(
            $(#[$INNER2])*
            #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
            struct $KEY2;

            impl Word for $KEY2 {
                $($(#[allow(non_upper_case_globals)] const $NAME2: $T = $VAL2;)+)+
            }
        )*
    };
    (
        $(#[$OUTER:meta])*
        pub enum $ENUM:ident : $T:ty {
            $(#[$INNER:meta])*
            $KEY:ident { $($($NAME:ident)|+ = $VAL:expr;)+ }
            $(
                $(#[$INNER2:meta])*
                $KEY2:ident { $($($NAME2:ident)|+ = $VAL2:expr;)+ }
            )*
        }
    ) => {
        pub trait Word {
            $($(#[allow(non_upper_case_globals)] const $NAME: $T;)+)+
        }

        $(#[$OUTER])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub enum $ENUM {
            $($(#[allow(non_camel_case_types)] $NAME,)+)+
        }

        impl $ENUM {
            #[inline]
            pub fn reword<W: Word>(self) -> $T {
                match self {
                    $($($ENUM::$NAME => W::$NAME,)+)+
                }
            }
        }

        $(#[$INNER])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub struct $KEY;

        impl Word for $KEY {
            $($(#[allow(non_upper_case_globals)] const $NAME: $T = $VAL;)+)+
        }

        $(
            $(#[$INNER2])*
            #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
            pub struct $KEY2;

            impl Word for $KEY2 {
                $($(#[allow(non_upper_case_globals)] const $NAME2: $T = $VAL2;)+)+
            }
        )*
    };
}
