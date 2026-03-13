#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    trait Arg;

    fn handle<'a>(&self, arg: impl Self::Arg<'a>);
    //~^ ERROR associated type takes 0 lifetime arguments but 1 lifetime argument was supplied
}

fn main() {}
