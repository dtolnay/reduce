iter.reduce(fn)
===============

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/reduce-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/reduce)
[<img alt="crates.io" src="https://img.shields.io/crates/v/reduce.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/reduce)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-reduce-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/reduce)
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
