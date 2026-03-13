#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    trait Arg;

    fn handle(&self, arg: impl Self::Arg);
}

struct HandlerImpl;

impl Handler for HandlerImpl {
    //~^ ERROR not all trait items implemented, missing: `Arg`
    fn handle(&self, _: impl Self::Arg) {}
}

fn main() {}
