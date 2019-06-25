# reword

[![Travis](https://travis-ci.org/evenorog/reword.svg?branch=master)](https://travis-ci.org/evenorog/reword)
[![Crates.io](https://img.shields.io/crates/v/reword.svg)](https://crates.io/crates/reword)
[![Docs](https://docs.rs/reword/badge.svg)](https://docs.rs/reword)

Provides a macro to generate structures used for constant value lookup.

```rust
reword! {
    enum Lang: &'static str {
        struct Hi {
            const NO = "Hei";
            const EN_UK | EN_US = "Hi";
        }
        struct Humor {
            const NO | EN_US = "Humor";
            const EN_UK = "Humour";
        }
    }
}

let mut lang = Lang::NO;
assert_eq!(lang.get::<Hi>(), "Hei");
assert_eq!(lang.get::<Humor>(), "Humor");
lang = Lang::EN_UK;
assert_eq!(lang.get::<Hi>(), "Hi");
assert_eq!(lang.get::<Humor>(), "Humour");
lang = Lang::EN_US;
assert_eq!(lang.get::<Hi>(), "Hi");
assert_eq!(lang.get::<Humor>(), "Humor");
```

The structures generated are not exported out of its module by default.
Use `pub` before the`enum` keyword to export it.
Attributes can be attached to both the `enum` and the structures generated.
The `Copy`, `Clone`, `Debug`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, and `Hash` traits are
automatically derived for the types using the derive attribute. At the moment, the reword macro
can only be used once per module, so if you need to define multiple structures you should
put them in separate submodules.

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
