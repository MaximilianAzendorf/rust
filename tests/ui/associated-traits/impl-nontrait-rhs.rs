#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    trait Arg;
}

impl Handler for () {
    trait Arg = std::rc::Rc<u32>;
    //~^ ERROR expected trait, found type
}

fn main() {}
