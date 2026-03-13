#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    trait Arg<T>;
}

impl Handler for () {
    trait Arg = Copy;
    //~^ ERROR associated type `Arg` has 0 type parameters but its trait declaration has 1 type parameter
}

fn main() {}
