//@ run-pass

#![feature(associated_traits, trait_alias)]
#![allow(incomplete_features)]
trait IntoIntIterator = IntoIterator<Item = i32>;

trait Handler {
    trait Arg;

    fn handle<ArgImpl: Self::Arg>(&self, arg: ArgImpl);
}

struct MyHandler;

impl Handler for MyHandler {
    trait Arg = IntoIntIterator;

    fn handle<ArgImpl: Self::Arg>(&self, arg: ArgImpl) {
        for number in arg {
            println!("{}", number);
        }
    }
}

fn example_generic_helper<HandlerImpl, ArgImpl>(handler: HandlerImpl, args: Vec<ArgImpl>)
where
    HandlerImpl: Handler,
    ArgImpl: <HandlerImpl as Handler>::Arg,
{
    for arg in args {
        handler.handle(arg);
    }
}

fn main() {
    let handler = MyHandler;
    example_generic_helper(handler, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}
