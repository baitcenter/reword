//! A fast and safe translation generator.
//!
//! The `reword!` macro generates types that allows for fast lookup of string literals based on the
//! current selected language. As seen in the example below, the `enum` generated has a `translate`
//! method that translates the given message based on the current selected language.
//!
//! # Examples
//! ```
//! #[macro_use]
//! extern crate reword;
//!
//! reword! {
//!     enum Lang {
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
//!     assert_eq!(lang.translate::<Hi>(), "Hei");
//!
//!     lang = Lang::EN_UK;
//!     assert_eq!(lang.translate::<HowAreYou>(), "How are you?");
//!
//!     lang = Lang::EN_US;
//!     assert_eq!(lang.translate::<HowAreYou>(), "How you doing?");
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
//!         pub enum Lang {
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
//!     assert_eq!(lang.translate::<example::Hi>(), "Hei");
//!
//!     lang = example::Lang::EN_UK;
//!     assert_eq!(lang.translate::<example::HowAreYou>(), "How are you?");
//!
//!     lang = example::Lang::EN_US;
//!     assert_eq!(lang.translate::<example::HowAreYou>(), "How you doing?");
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
//! reword! {
//!     #[repr(C)]
//!     enum Lang {
//!         #[derive(Default)]
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
//!     assert_eq!(lang.translate::<Hi>(), "Hei");
//!
//!     lang = Lang::EN_UK;
//!     assert_eq!(lang.translate::<HowAreYou>(), "How are you?");
//!
//!     lang = Lang::EN_US;
//!     assert_eq!(lang.translate::<HowAreYou>(), "How you doing?");
//! }
//! ```

#![no_std]
#![forbid(unstable_features)]
#![deny(missing_debug_implementations, unused_import_braces, unused_qualifications, unsafe_code)]

/// The macro used to generate the language structures.
///
/// See the [crate level docs](index.html) for more information.
#[macro_export]
macro_rules! reword {
    (
        $(#[$OUTER:meta])*
        enum $ENUM:ident {
            $(#[$INNER:meta])*
            $KEY:ident {
                $(
                    $($LANG:ident)|+ = $STR:expr;
                )+
            }
            $(
                $(#[$INNER2:meta])*
                $KEY2:ident {
                    $(
                        $($LANG2:ident)|+ = $STR2:expr;
                    )+
                }
            )*
        }
    ) => {
        trait Text {
            $($(
                const $LANG: &'static str;
            )+)+
        }

        $(#[$OUTER])*
        #[allow(bad_style)]
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        enum $ENUM {
            $($(
                $LANG,
            )+)+
        }
        impl $ENUM {
            #[inline]
            fn translate<T: Text>(self) -> &'static str {
                match self {
                    $($(
                        Lang::$LANG => T::$LANG,
                    )+)+
                }
            }
        }

        $(#[$INNER])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        struct $KEY;
        impl Text for $KEY {
            $($(
                const $LANG: &'static str = $STR;
            )+)+
        }
        $(
            $(#[$INNER2])*
            #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
            struct $KEY2;
            impl Text for $KEY2 {
                $($(
                    const $LANG2: &'static str = $STR2;
                )+)+
            }
        )*
    };
    (
        $(#[$OUTER:meta])*
        pub enum $ENUM:ident {
            $(#[$INNER:meta])*
            $KEY:ident {
                $(
                    $($LANG:ident)|+ = $STR:expr;
                )+
            }
            $(
                $(#[$INNER2:meta])*
                $KEY2:ident {
                    $(
                        $($LANG2:ident)|+ = $STR2:expr;
                    )+
                }
            )*
        }
    ) => {
        pub trait Text {
            $($(
                const $LANG: &'static str;
            )+)+
        }

        $(#[$OUTER])*
        #[allow(bad_style)]
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub enum $ENUM {
            $($(
                $LANG,
            )+)+
        }
        impl $ENUM {
            #[inline]
            pub fn translate<T: Text>(self) -> &'static str {
                match self {
                    $($(
                        Lang::$LANG => T::$LANG,
                    )+)+
                }
            }
        }

        $(#[$INNER])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub struct $KEY;
        impl Text for $KEY {
            $($(
                const $LANG: &'static str = $STR;
            )+)+
        }
        $(
            $(#[$INNER2])*
            #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
            pub struct $KEY2;
            impl Text for $KEY2 {
                $($(
                    const $LANG2: &'static str = $STR2;
                )+)+
            }
        )*
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn reword() {
        reword! {
            enum Lang {
                Hi {
                    EN_UK | EN_US = "Hi";
                    NO = "Hei";
                }
                HowAreYou {
                    EN_UK = "How are you?";
                    EN_US = "How you doing?";
                    NO = "Hvordan går det?";
                }
            }
        }

        let mut lang = Lang::EN_UK;
        assert_eq!(lang.translate::<Hi>(), "Hi");

        lang = Lang::EN_US;
        assert_eq!(lang.translate::<Hi>(), "Hi");

        lang = Lang::NO;
        assert_eq!(lang.translate::<Hi>(), "Hei");

        lang = Lang::EN_UK;
        assert_eq!(lang.translate::<HowAreYou>(), "How are you?");

        lang = Lang::EN_US;
        assert_eq!(lang.translate::<HowAreYou>(), "How you doing?");

        lang = Lang::NO;
        assert_eq!(lang.translate::<HowAreYou>(), "Hvordan går det?");
    }
}
