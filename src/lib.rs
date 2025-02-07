use std::ops::BitOr;
pub struct Start;
pub struct End;

pub struct Pipe<T> {
    token: Cancellable,
    value: T,
}

impl<T> Pipe<T> {
    pub fn new(value: T) -> Pipe<T> {
        Self {
            token: Cancellable::None,
            value,
        }
    }
}

impl<T: Default> Pipe<T> {
    pub fn empty() -> Pipe<T> {
        Pipe {
            token: Cancellable::None,
            value: T::default(),
        }
    }
}

#[derive(Default)]
enum Cancellable {
    #[default]
    None,
    Stop,
}

pub struct Unwrap;

impl<T, U, F: FnOnce(T) -> U> BitOr<F> for Pipe<T> {
    type Output = Pipe<U>;

    fn bitor(self, f: F) -> Self::Output {
        Pipe {
            token: Cancellable::None,
            value: f(self.value),
        }
    }
}

impl<T> BitOr<Unwrap> for Pipe<T> {
    type Output = T;

    fn bitor(self, _: Unwrap) -> Self::Output {
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
        let result = Pipe::new(5) | power_of_two;
        assert_eq!(result.value, 25);

        let result = result | Unwrap;
        assert_eq!(result, 25);
    }
}
