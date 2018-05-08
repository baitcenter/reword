# reword
[![Travis](https://travis-ci.org/evenorog/reword.svg?branch=master)](https://travis-ci.org/evenorog/reword)
[![Appveyor](https://ci.appveyor.com/api/projects/status/a25ige001079fisb?svg=true)](https://ci.appveyor.com/project/evenorog/reword)
[![Crates.io](https://img.shields.io/crates/v/reword.svg)](https://crates.io/crates/reword)
[![Docs](https://docs.rs/reword/badge.svg)](https://docs.rs/reword)

A macro for generating types that allows for fast lookup of `const` values at runtime.

## Examples

Add this to `Cargo.toml`:

```toml
[dependencies]
reword = "0.2"
```

And this to `main.rs`:

```rust
#[macro_use]
extern crate reword;

reword! {
    enum Lang: &'static str {
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

fn main() {
    let mut lang = Lang::NO;
    assert_eq!(lang.reword::<Hi>(), "Hei");

    lang = Lang::EN_UK;
    assert_eq!(lang.reword::<HowAreYou>(), "How are you?");

    lang = Lang::EN_US;
    assert_eq!(lang.reword::<HowAreYou>(), "How you doing?");
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
