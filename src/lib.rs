#![no_std]

#[macro_export]
macro_rules! lang {
    (
        $KEY:ident {
            $(
                $LANG:ident = $STR:expr;
            )+
        }
        $(
            $KEY2:ident {
                $(
                    $LANG2:ident = $STR2:expr;
                )+
            }
        )*
    ) => {
        pub trait Text {
            $(
                const $LANG: &'static str;
            )+
        }

        #[allow(bad_style)]
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
        pub enum Lang {
            $(
                $LANG,
            )+
        }
        impl Lang {
            #[inline]
            pub fn to_str<T: Text>(self) -> &'static str {
                match self {
                    $(
                        Lang::$LANG => T::$LANG,
                    )+
                }
            }
        }

        pub struct $KEY;
        impl Text for $KEY {
            $(
                const $LANG: &'static str = $STR;
            )+
        }
        $(
            pub struct $KEY2;
            impl Text for $KEY2 {
                $(
                    const $LANG2: &'static str = $STR2;
                )+
            }
        )*
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn lang() {
        lang! {
            Hi {
                EN_UK = "Hi";
                NO = "Hei";
                L33T = "Yo";
            }
            HowAreYou {
                EN_UK = "How are you?";
                NO = "Hvordan har du det?";
                L33T = "How u doing?";
            }
        }

        let mut lang = Lang::NO;
        assert_eq!(lang.to_str::<Hi>(), "Hei");

        lang = Lang::EN_UK;
        assert_eq!(lang.to_str::<HowAreYou>(), "How are you?");

        lang = Lang::L33T;
        assert_eq!(lang.to_str::<HowAreYou>(), "How u doing?");
    }
}
