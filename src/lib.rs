//! [![github]](https://github.com/dtolnay/reduce)&ensp;[![crates-io]](https://crates.io/crates/reduce)&ensp;[![docs-rs]](https://docs.rs/reduce)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This crate gives Iterators a `reduce` function that is similar to
//! [`fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
//! but without an initial value. The function returns `None` if
//! the iterator is empty and `Some(value)` otherwise. This matches the distinction
//! between
//! [`reduce`](https://www.scala-lang.org/api/current/scala/collection/Iterator.html#reduce[A1%3E:A](op:(A1,A1)=%3EA1):A1)
//! and
//! [`fold`](https://www.scala-lang.org/api/current/scala/collection/Iterator.html#fold[A1%3E:A](z:A1)(op:(A1,A1)=%3EA1):A1)
//! in Scala.
//!
//! # Examples
//!
//! ```rust
//! use reduce::Reduce;
//!
//! fn main() {
//!     // Reduce a non-empty iterator into Some(value)
//!     let v = vec![1usize, 2, 3, 4, 5];
//!     let sum = v.into_iter().reduce(|a, b| a + b);
//!     assert_eq!(Some(15), sum);
//!
//!     // Reduce an empty iterator into None
//!     let v = Vec::<usize>::new();
//!     let sum = v.into_iter().reduce(|a, b| a + b);
//!     assert_eq!(None, sum);
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/reduce/0.1.4")]
#![no_std]

pub trait Reduce<T> {
    fn reduce<F>(self, f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T;
}

impl<T, I> Reduce<T> for I
where
    I: Iterator<Item = T>,
{
    #[inline]
    fn reduce<F>(mut self, f: F) -> Option<T>
    where
        Self: Sized,
        F: FnMut(T, T) -> T,
    {
        self.next().map(|first| self.fold(first, f))
    }
}
