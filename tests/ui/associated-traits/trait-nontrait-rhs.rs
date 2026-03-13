#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    trait Arg = u32;
    //~^ ERROR expected trait, found builtin type `u32`
}

fn main() {}
