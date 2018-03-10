#![no_std]

#[macro_export]
macro_rules! lang {
    (
        $KEY:ident {
            $(
                $($LANG:ident)|+ = $STR:expr;
            )+
        }
        $(
            $KEY2:ident {
                $(
                    $($LANG2:ident)|+ = $STR2:expr;
                )+
            }
        )*
    ) => {
        pub trait Text {
            $($(
                const $LANG: &'static str;
            )+)+
        }

        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub enum Lang {
            $($(
                $LANG,
            )+)+
        }
        impl Lang {
            #[inline]
            pub fn translate<T: Text>(self) -> &'static str {
                match self {
                    $($(
                        Lang::$LANG => T::$LANG,
                    )+)+
                }
            }
        }

        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub struct $KEY;
        impl Text for $KEY {
            $($(
                const $LANG: &'static str = $STR;
            )+)+
        }
        $(
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
    #[allow(bad_style)]
    fn lang() {
        lang! {
            Hi {
                EN_UK | EN_US = "Hi";
                NO = "Hei";
            }
            HowAreYou {
                EN_UK = "How are you?";
                EN_US = "How you doing?";
                NO = "Hvordan har du det?";
            }
        }

        let mut lang = Lang::NO;
        assert_eq!(lang.translate::<Hi>(), "Hei");

        lang = Lang::EN_UK;
        assert_eq!(lang.translate::<HowAreYou>(), "How are you?");

        lang = Lang::EN_US;
        assert_eq!(lang.translate::<HowAreYou>(), "How you doing?");
    }
}
