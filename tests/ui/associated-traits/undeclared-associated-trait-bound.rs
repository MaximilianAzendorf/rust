#![feature(associated_traits)]
#![allow(incomplete_features, dead_code, unused_variables)]

trait Handler {
    fn handle(&self, arg: impl Self::Arg);
    //~^ ERROR cannot find associated trait `Arg` in trait `Handler`
}

fn main() {}
