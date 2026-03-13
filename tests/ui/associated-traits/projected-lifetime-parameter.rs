//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Borrower {
    trait Ref<'a>: AsRef<str> + 'a;
}

struct BorrowerImpl;

impl Borrower for BorrowerImpl {
    trait Ref<'a> = AsRef<str> + 'a;
}

fn print_ref<BorrowerImplTy>(value: impl <BorrowerImplTy as Borrower>::Ref<'static>)
where
    BorrowerImplTy: Borrower,
{
    println!("{}", value.as_ref());
}

fn main() {
    print_ref::<BorrowerImpl>("ok");
}
