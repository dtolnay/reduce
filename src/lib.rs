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

#[test]
fn test_reduce_some() {
    let v = vec![1usize, 2, 3, 4, 5];
    let sum = v.into_iter().reduce(|a, b| a + b);
    assert_eq!(Some(15), sum);
}

#[test]
fn test_reduce_one() {
    let v = vec![1usize];
    let sum = v.into_iter().reduce(|a, b| a + b);
    assert_eq!(Some(1), sum);
}

#[test]
fn test_reduce_none() {
    let v = Vec::<usize>::new();
    let sum = v.into_iter().reduce(|a, b| a + b);
    assert_eq!(None, sum);
}
