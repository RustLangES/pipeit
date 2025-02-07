use pipeit::*;

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

fn main() {
    let result = Pipe::new()
        | 5
        | debug_with("the value of the pipe is:")
        | power_of_two
        | change_to_string
        | unwrap_or(String::from("hello, world"))
        | debug
        | Pipe::end();

    println!("{result:?}");
}
