//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Collection {
    type Item;
    trait Elem;

    fn take(&self, value: impl Self::Elem);
}

struct CollectionImpl;

impl Collection for CollectionImpl {
    type Item = u32;
    trait Elem = Into<<Self as Collection>::Item>;

    fn take(&self, value: impl Self::Elem) {
        let value: <Self as Collection>::Item = value.into();
        println!("{value}");
    }
}

fn main() {
    CollectionImpl.take(5u8);
}
