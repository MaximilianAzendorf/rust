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

fn call_with_any_ref<BorrowerImplTy>(borrower: &BorrowerImplTy)
where
    BorrowerImplTy: Borrower,
    for<'a> &'a str: <BorrowerImplTy as Borrower>::Ref<'a>,
{
    borrower.use_ref("ok");
}

fn main() {
    call_with_any_ref(&BorrowerImpl);
}
