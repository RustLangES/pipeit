use core::ops::BitOr;
use std::sync::{atomic::AtomicBool, Arc};

pub struct Pipe;

pub struct Cancellable {
    cancelled: Arc<AtomicBool>,
}

pub struct Pipelined<T> {
    value: T,
    token: Option<Cancellable>,
}

impl<T> BitOr<T> for Pipe {
    type Output = Pipelined<T>;

    fn bitor(self, it: T) -> Self::Output {
        Pipelined {
            value: it,
            token: None,
        }
    }
}

impl<T, U, F: FnOnce(T) -> U> BitOr<F> for Pipelined<T> {
    type Output = Pipelined<U>;

    fn bitor(self, f: F) -> Self::Output {
        Pipelined {
            value: f(self.value),
            token: self.token,
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
}
