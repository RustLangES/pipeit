use core::ops::BitOr;
use std::ops::Shr;

pub struct Pipe;
pub struct Pipelined<T> {
    value: T,
}

pub struct CollectablePipe<T, U> {
    value: Vec<Box<dyn Fn(T) -> U>>,
}

impl<T> BitOr<T> for Pipe {
    type Output = Pipelined<T>;

    fn bitor(self, it: T) -> Self::Output {
        Pipelined { value: it }
    }
}

impl<T, U, F: Fn(T) -> U + 'static> Shr<F> for Pipe {
    type Output = CollectablePipe<T, U>;

    fn shr(self, rhs: F) -> Self::Output {
        let mut collectable = CollectablePipe {
            value: Vec::with_capacity(1),
        };
        collectable.value.push(Box::new(rhs));
        collectable
    }
}

impl<T, U, F: Fn(T) -> U + 'static> BitOr<F> for CollectablePipe<T, U> {
    type Output = CollectablePipe<T, U>;

    fn bitor(mut self, rhs: F) -> Self::Output {
        self.value.push(Box::new(rhs));
        self
    }
}

impl<T, U, F: FnOnce(T) -> U> BitOr<F> for Pipelined<T> {
    type Output = Pipelined<U>;

    fn bitor(self, f: F) -> Self::Output {
        Pipelined {
            value: f(self.value),
        }
    }
}

pub struct It;
impl<T> BitOr<It> for Pipelined<T> {
    type Output = T;

    fn bitor(self, _: It) -> T {
        self.value
    }
}

impl<T: Clone, U: Default> BitOr<It> for CollectablePipe<T, U> {
    type Output = Box<dyn Fn(T) -> U + 'static>;

    fn bitor(self, _: It) -> Self::Output {
        Box::new(move |mut v| self.value.iter().fold(U::default(), |acc, f| f(v)))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn power_of_two(x: i32) -> i32 {
        x.pow(2)
    }

    #[test]
    fn first_try() {
        let result = Pipe | 5 | power_of_two | It;

        assert_eq!(result, 25);
    }

    #[test]
    fn second_try() {
        let result = Pipe >> power_of_two | It;
        let result = result(5);

        assert_eq!(result, 25);
    }
}
