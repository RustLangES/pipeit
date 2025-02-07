use std::{
    marker::PhantomData,
    ops::BitOr,
    sync::{atomic::AtomicBool, Arc},
};
pub struct Start;
pub struct End;

pub struct Pipe<State = Start> {
    token: Option<Cancellable>,
    __phantom: PhantomData<State>,
}

impl Pipe<Start> {
    pub fn new() -> Pipe<Start> {
        Pipe {
            token: None,
            __phantom: PhantomData::<Start>,
        }
    }
}

impl Pipe<Start> {
    pub fn end() -> Pipe<End> {
        Pipe {
            token: None,
            __phantom: PhantomData::<End>,
        }
    }
}

pub struct Cancellable {
    cancelled: Arc<AtomicBool>,
}

pub struct Pipelined<T> {
    value: T,
    token: Option<Cancellable>,
}

impl<T> BitOr<T> for Pipe<Start> {
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

impl<T> BitOr<Pipe<End>> for Pipelined<T> {
    type Output = T;

    fn bitor(self, _: Pipe<End>) -> T {
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
        let result = Pipe::new() | 5 | power_of_two | Pipe::end();

        assert_eq!(result, 25);
    }
}
