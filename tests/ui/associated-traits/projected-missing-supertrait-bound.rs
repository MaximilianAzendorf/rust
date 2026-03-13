#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Borrower {
    trait Ref<'a>;
}

impl Borrower for () {
    trait Ref<'a> = AsRef<str> + 'a;
}

fn show<B: Borrower>(value: impl <B as Borrower>::Ref<'static>) {
    let _ = value.as_ref();
    //~^ ERROR no method named `as_ref` found
}

fn main() {}
