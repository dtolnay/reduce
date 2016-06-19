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
