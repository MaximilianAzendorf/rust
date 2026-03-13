trait Handler {
    trait Arg;
    //~^ ERROR associated traits are experimental

    trait Iter = Clone;
    //~^ ERROR associated traits are experimental
}

impl Handler for i32 {
    trait Arg = Copy;
    //~^ ERROR associated traits are experimental

    trait Iter = Clone;
    //~^ ERROR associated traits are experimental
}

fn main() {}
