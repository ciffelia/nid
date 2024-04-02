# nid

Generate and parse Nano IDs.

Nano ID is a small, secure, URL-friendly, unique string ID.
Here's an example of a Nano ID:

```
qjH-6uGrFy0QgNJtUh0_c
```

This crate is a Rust implementation of the original [Nano ID](https://github.com/ai/nanoid) library written in JavaScript.
Please refer to the original library for the detailed explanation of Nano ID.

## Getting started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
nid = "0.1.0"
```

When you want a new Nano ID, you can generate one using the [`Nanoid::new`] method.

```rust
use nid::Nanoid;
let id: Nanoid = Nanoid::new();
```

You can parse a string into a Nano ID using the [`std::str::FromStr`] or [`TryFrom`] trait.

```rust
use nid::Nanoid;
let id: Nanoid = "3hYR3muA_xvjMrrrqFWxF".parse()?;
let id: Nanoid = "iH26rJ8CpRz-gfIh7TSRu".to_string().try_into()?;
```

The length of the Nano ID is 21 by default, but you can change it by specifying the generic parameter.

```rust
use nid::Nanoid;
let id: Nanoid<10> = "j1-SOTHHxi".parse()?;
```

You can also use a different alphabet. The list of available alphabets is in the [`alphabet`] module.

```rust
use nid::{alphabet::Base62Alphabet, Nanoid};
let id: Nanoid<10, Base62Alphabet> = Nanoid::new();
```

## Examples

```rust
use nid::{alphabet::Base62Alphabet, Nanoid};

// Generate a new Nano ID and print it.
let id: Nanoid = Nanoid::new();
println!("{}", id);

// Parse a string into a Nano ID and convert it back to a string.
let id: Nanoid = "abcdefg1234567UVWXYZ_".parse()?;
let s = id.to_string();

// Parse a string into a Nano ID with a different length and alphabet.
let id: Nanoid<9, Base62Alphabet> = "abc123XYZ".parse()?;
```

## Comparison with other implementations of Nano ID

[`nanoid`](https://docs.rs/nanoid) and [`nano-id`](https://docs.rs/nano-id) are other implementations of Nano ID in Rust.
The main difference between `nid` and the other implementations is that `nid` has [`Nanoid`] type to represent Nano IDs.
This type provides a safe way to generate and parse Nano IDs.
This is similar to [`uuid`](https://docs.rs/uuid) crate, which provides [`Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html) type to represent UUIDs.

[`Nanoid::new`]: https://docs.rs/nid/latest/nid/struct.Nanoid.html#method.new
[`std::str::FromStr`]: https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html
[`TryFrom`]: https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html
[`alphabet`]: https://docs.rs/nid/latest/nid/alphabet/index.html
[`Nanoid`]: https://docs.rs/nid/latest/nid/struct.Nanoid.html

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
