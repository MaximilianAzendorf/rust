//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Converter {
    trait Arg<T>: Into<T>;
}

struct ConverterImpl;

impl Converter for ConverterImpl {
    trait Arg<T> = Into<T>;
}

fn print_u32<ConverterImplTy>(value: impl <ConverterImplTy as Converter>::Arg<u32>)
where
    ConverterImplTy: Converter,
{
    let value: u32 = value.into();
    println!("{value}");
}

fn main() {
    print_u32::<ConverterImpl>(5u8);
}
