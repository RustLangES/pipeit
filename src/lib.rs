use core::ops::BitOr;
use std::sync::Arc;

struct Pipe;

struct Cancel {
    cancelled: Arc<AtomicBool>,
}

struct Pipelined {
    value: T,
    token: Option<Cancellable>,
}

impl<T> BitOr<T> for Pipe {
    type Output = Pipelined<T>;

    fn bitor(self, it: T) -> Self::Output {
        Pipelined(it)
    }
}

impl<T, U, F: FnOnce(T) -> U> BitOr<F> for Pipelined<T> {
    type Output = Pipelined<U>;

    fn bitor(self, f: F) -> Self::Output {
        Pipelined(f(self.0))
    }
}

struct It;

impl<T> BitOr<It> for Pipelined<T> {
    type Output = T;

    fn bitor(self, _: It) -> T {
        self.0
    }
}

fn power_of_two(x: i32) -> i32 {
    x.pow(2)
}

fn change_to_string(x: i32) -> Option<String> {
    if x == 25 {
        Some(String::from("hello, world!"))
    } else {
        None
    }
}

fn unwrap_or<T: Clone>(or: T) -> impl FnOnce(Option<T>) -> T {
    move |v: Option<T>| -> _ { v.unwrap_or(or) }
}

fn debug<T: std::fmt::Debug>(x: T) -> T {
    println!("{x:?}");
    x
}

fn debug_with<T: std::fmt::Debug, U: ToString>(msg: U) -> impl Fn(T) -> T {
    move |x: T| -> _ {
        println!("{} {x:?}", msg.to_string());
        x
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_try() {
        let result = Pipe
            | 5
            | debug_with("the value of the pipe is:")
            | power_of_two
            | change_to_string
            | unwrap_or(String::from("hello, world"))
            | debug
            | It;

        assert_eq!(result, String::from("hello, world!"));
    }
}
