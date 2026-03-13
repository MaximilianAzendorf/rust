//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Handler {
    trait Arg;

    fn handle(&self, arg: impl Self::Arg);
}

struct HandlerImpl;

impl Handler for HandlerImpl {
    trait Arg = Copy;

    fn handle(&self, _: impl Self::Arg) {
        println!("ok");
    }
}

fn main() {
    HandlerImpl.handle(0);
}
