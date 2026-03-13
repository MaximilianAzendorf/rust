//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Borrower {
    trait Ref<'a>;

    fn use_ref<'a>(&self, value: impl Self::Ref<'a>);
}

struct BorrowerImpl;

impl Borrower for BorrowerImpl {
    trait Ref<'a> = AsRef<str> + 'a;

    fn use_ref<'a>(&self, value: impl Self::Ref<'a>) {
        println!("{}", value.as_ref());
    }
}

fn main() {
    let value = String::from("ok");
    BorrowerImpl.use_ref(&value);
}
