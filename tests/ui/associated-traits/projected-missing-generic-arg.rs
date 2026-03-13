#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    trait Arg<T>;
}

impl Handler for () {
    trait Arg<T> = Into<T>;
}

fn use_handler<H: Handler>(_arg: impl <H as Handler>::Arg) {}
//~^ ERROR missing generics for associated type `Handler::Arg`

fn main() {}
