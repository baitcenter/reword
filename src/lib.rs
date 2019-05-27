//! Provides a macro to generate structures used for constant value lookup.
//!
//! ```
//! # use reword::reword;
//! reword! {
//!     enum Lang: &'static str {
//!         struct Hi {
//!             const NO = "Hei";
//!             const EN_UK | EN_US = "Hi";
//!         }
//!         struct Humor {
//!             const NO | EN_US = "Humor";
//!             const EN_UK = "Humour";
//!         }
//!     }
//! }
//!
//! let mut lang = Lang::NO;
//! assert_eq!(lang.get::<Hi>(), "Hei");
//! assert_eq!(lang.get::<Humor>(), "Humor");
//! lang = Lang::EN_UK;
//! assert_eq!(lang.get::<Hi>(), "Hi");
//! assert_eq!(lang.get::<Humor>(), "Humour");
//! lang = Lang::EN_US;
//! assert_eq!(lang.get::<Hi>(), "Hi");
//! assert_eq!(lang.get::<Humor>(), "Humor");
//! ```
//!
//! Attributes can be attached to both the `enum` and the structures generated.
//! The `Copy`, `Clone`, `Debug`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, and `Hash` traits are
//! automatically derived for the types using the derive attribute.
//! The structures generated are not exported out of its module by default.
//! Use `pub` before the`enum` keyword to export it.

#![no_std]
#![doc(html_root_url = "https://docs.rs/reword/latest")]
#![deny(
    bad_style,
    bare_trait_objects,
    missing_debug_implementations,
    missing_docs,
    unused_import_braces,
    unused_qualifications,
    unsafe_code,
    unstable_features
)]

/// The macro used to generate the structures used for constant value lookup.
///
/// See the [crate level docs](index.html) for more information.
#[macro_export]
macro_rules! reword {
    (
        $(#[$outer:meta])*
        $pub:vis enum $main:ident : $T:ty {
            $(#[$inner:meta])*
            struct $key:ident { $(const $($name:ident)|+ = $val:expr;)+ }
            $(
                $(#[$inner2:meta])*
                struct $key2:ident { $(const $($name2:ident)|+ = $val2:expr;)+ }
            )*
        }
    ) => {
        /// Trait used for constant value lookup.
        $pub trait Word {
            $($(#[allow(non_upper_case_globals)] const $name: $T;)+)+
        }

        $(#[$outer])*
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
        $pub enum $main {
            $($(#[allow(non_camel_case_types)] $name,)+)+
        }

        impl $main {
            /// Returns the value of `W`.
            #[inline]
            $pub fn get<W: Word + ?Sized>(self) -> $T {
                match self {
                    $($($main::$name => W::$name,)+)+
                }
            }
        }

        $(#[$inner])*
        #[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
        $pub struct $key;

        impl Word for $key {
            $($(#[allow(non_upper_case_globals)] const $name: $T = $val;)+)+
        }

        $(
            $(#[$inner2])*
            #[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
            $pub struct $key2;

            impl Word for $key2 {
                $($(#[allow(non_upper_case_globals)] const $name2: $T = $val2;)+)+
            }
        )*
    };
}
