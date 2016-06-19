iter.reduce(fn)
===============

[![Build Status](https://api.travis-ci.org/dtolnay/reduce.svg?branch=master)](https://travis-ci.org/dtolnay/reduce)
[![Latest Version](https://img.shields.io/crates/v/reduce.svg)](https://crates.io/crates/reduce)

This crate gives Iterators a `reduce` function that is similar to
[`fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
but without an initial value. The function returns `None` if the iterator is
empty and `Some(value)` otherwise. This matches the distinction between
[`reduce`](http://www.scala-lang.org/api/current/index.html#scala.collection.Iterator@reduce[A1%3E:A](op:(A1,A1)=%3EA1):A1)
and
[`fold`](http://www.scala-lang.org/api/current/index.html#scala.collection.Iterator@fold[A1%3E:A](z:A1)(op:(A1,A1)=%3EA1):A1)
in Scala.

## Usage

```rust
extern crate reduce;
use reduce::Reduce;

// Reduce a non-empty iterator into Some(value)
let v = vec![1usize, 2, 3, 4, 5];
let sum = v.into_iter().reduce(|a, b| a + b);
assert_eq!(Some(15), sum);

// Reduce an empty iterator into None
let v = Vec::<usize>::new();
let sum = v.into_iter().reduce(|a, b| a + b);
assert_eq!(None, sum);
```

## Dependency

Reduce is available on [crates.io](https://crates.io/crates/reduce). Use the
following in `Cargo.toml`:

```toml
[dependencies]
reduce = "^0.1"
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Reduce by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
