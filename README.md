iter.reduce(fn)
===============

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/reduce-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/reduce)
[<img alt="crates.io" src="https://img.shields.io/crates/v/reduce.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/reduce)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-reduce-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/reduce)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/reduce/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/reduce/actions?query=branch%3Amaster)

This crate gives Iterators a `reduce` function that is similar to
[`fold`][std-fold] but without an initial value. The function returns `None` if
the iterator is empty and `Some(value)` otherwise. This matches the distinction
between [`reduce`][scala-reduce] and [`fold`][scala-fold] in Scala.

[std-fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
[scala-reduce]: https://www.scala-lang.org/api/current/scala/collection/Iterator.html#reduce[A1%3E:A](op:(A1,A1)=%3EA1):A1
[scala-fold]: https://www.scala-lang.org/api/current/scala/collection/Iterator.html#fold[A1%3E:A](z:A1)(op:(A1,A1)=%3EA1):A1

```toml
[dependencies]
reduce = "0.1"
```

## Examples

```rust
use reduce::Reduce;

fn main() {
    // Reduce a non-empty iterator into Some(value)
    let v = vec![1usize, 2, 3, 4, 5];
    let sum = v.into_iter().reduce(|a, b| a + b);
    assert_eq!(Some(15), sum);

    // Reduce an empty iterator into None
    let v = Vec::<usize>::new();
    let sum = v.into_iter().reduce(|a, b| a + b);
    assert_eq!(None, sum);
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
