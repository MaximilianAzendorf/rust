//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Handler {
    trait Arg;

    fn handle<ArgImpl: Self::Arg>(&self, arg: ArgImpl);
}

struct HandlerImpl;

impl Handler for HandlerImpl {
    trait Arg = Copy;

    fn handle<ArgImpl: Self::Arg>(&self, _: ArgImpl) {
        println!("ok");
    }
}

fn main() {
    HandlerImpl.handle(0);
}
