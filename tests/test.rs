// Copyright 2016 Reduce Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate reduce;
use reduce::Reduce;

#[test]
fn test_reduce_some() {
    let v = vec![1usize, 2, 3, 4, 5];
    let sum = v.into_iter().reduce(|a, b| a + b);
    assert_eq!(Some(15), sum);
}

#[test]
fn test_reduce_none() {
    let v = Vec::<usize>::new();
    let sum = v.into_iter().reduce(|a, b| a + b);
    assert_eq!(None, sum);
}
