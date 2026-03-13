#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Borrower {
    trait Ref<'a>;
}

impl Borrower for () {
    trait Ref = Copy;
    //~^ ERROR lifetime parameters or bounds on associated type `Ref` do not match the trait declaration
}

fn main() {}
