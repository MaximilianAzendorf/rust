#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    trait Arg: Copy;
}

impl Handler for () {
    trait Arg = Clone;
    //~^ ERROR associated trait alias does not satisfy the bounds of `Handler::Arg`
}

fn main() {}
