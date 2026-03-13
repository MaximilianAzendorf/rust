//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Handler {
    trait Arg;
    trait Iter = Clone;

    fn handle<ArgImpl: Self::Arg, IterImpl: Self::Iter>(&self, arg: ArgImpl, iter: IterImpl)
        -> IterImpl;
}

impl Handler for i32 {
    trait Arg = Copy + std::fmt::Display;
    trait Iter = Clone;

    fn handle<ArgImpl: Self::Arg, IterImpl: Self::Iter>(&self, arg: ArgImpl, iter: IterImpl)
        -> IterImpl
    {
        println!("{arg}");
        iter.clone()
    }
}

fn main() {
    let copied = 0i32.handle(7, String::from("ok"));
    println!("{copied}");
}
