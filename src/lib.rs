//! A fast and safe translation generator.
//!
//! The `reword!` macro generates types that allows for fast lookup of `&'static str` based on the
//! current selected language. As seen in the example below, the `enum` generated has a `reword`
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
//! # Generated Code
//!
//! Usage:
//!
//! ```
//! # #[macro_use]
//! # extern crate reword;
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
//! # fn main() {}
//! ```
//!
//! Generated:
//!
//! ```
//! trait Text {
//!     const EN_UK: &'static str;
//!     const EN_US: &'static str;
//!     const NO: &'static str;
//! }
//!
//! #[allow(bad_style)]
//! #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
//! enum Lang {
//!     EN_UK,
//!     EN_US,
//!     NO,
//! }
//!
//! impl Lang {
//!     #[inline]
//!     fn reword<T: Text>(self) -> &'static str {
//!         match self {
//!             Lang::EN_UK => T::EN_UK,
//!             Lang::EN_US => T::EN_US,
//!             Lang::NO => T::NO,
//!         }
//!     }
//! }
//!
//! #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
//! struct Hi;
//!
//! impl Text for Hi {
//!     const EN_UK: &'static str = "Hi";
//!     const EN_US: &'static str = "Hi";
//!     const NO: &'static str = "Hei";
//! }
//!
//! #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
//! struct HowAreYou;
//!
//! impl Text for HowAreYou {
//!     const EN_UK: &'static str = "How are you?";
//!     const EN_US: &'static str = "How you doing?";
//!     const NO: &'static str = "Hvordan går det?";
//! }
//!
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
            fn reword<T: Text>(self) -> &'static str {
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
            pub fn reword<T: Text>(self) -> &'static str {
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
                    SE = "Hej";
                }
                HowAreYou {
                    EN_UK = "How are you?";
                    EN_US = "How you doing?";
                    NO = "Hvordan går det?";
                    SE = "Hur mår du?";
                }
            }
        }

        let mut lang = Lang::EN_UK;
        assert_eq!(lang.reword::<Hi>(), "Hi");

        lang = Lang::EN_US;
        assert_eq!(lang.reword::<Hi>(), "Hi");

        lang = Lang::NO;
        assert_eq!(lang.reword::<Hi>(), "Hei");

        lang = Lang::SE;
        assert_eq!(lang.reword::<Hi>(), "Hej");

        lang = Lang::EN_UK;
        assert_eq!(lang.reword::<HowAreYou>(), "How are you?");

        lang = Lang::EN_US;
        assert_eq!(lang.reword::<HowAreYou>(), "How you doing?");

        lang = Lang::NO;
        assert_eq!(lang.reword::<HowAreYou>(), "Hvordan går det?");

        lang = Lang::SE;
        assert_eq!(lang.reword::<HowAreYou>(), "Hur mår du?")
    }
}
