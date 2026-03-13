//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Handler {
    trait Arg;
}

impl Handler for () {
    trait Arg = Copy;
}

fn use_handler<HandlerImpl>(arg: impl <HandlerImpl as Handler>::Arg)
where
    HandlerImpl: Handler,
{
    let _ = arg;
    println!("ok");
}

fn main() {
    use_handler::<()>(0);
}
