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
