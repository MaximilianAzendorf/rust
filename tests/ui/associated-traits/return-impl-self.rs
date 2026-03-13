//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]
#![allow(refining_impl_trait)]

trait Factory {
    trait Output;

    fn make(&self) -> impl Self::Output;
}

struct FactoryImpl;

impl Factory for FactoryImpl {
    trait Output = Copy + std::fmt::Display;

    fn make(&self) -> impl Self::Output {
        7u8
    }
}

fn main() {
    println!("{}", FactoryImpl.make());
}
