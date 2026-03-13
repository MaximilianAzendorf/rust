//@ run-pass

#![feature(associated_traits, trait_alias)]
#![allow(incomplete_features)]

trait Accepts<T> = Into<T>;

trait Converter {
    trait Arg<T>;

    fn take_u32(&self, value: impl Self::Arg<u32>);
}

struct ConverterImpl;

impl Converter for ConverterImpl {
    trait Arg<T> = Accepts<T>;

    fn take_u32(&self, value: impl Self::Arg<u32>) {
        let value: u32 = value.into();
        println!("{value}");
    }
}

fn main() {
    ConverterImpl.take_u32(5u8);
}
