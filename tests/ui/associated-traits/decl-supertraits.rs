//@ run-pass

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Service {
    trait Request: Send + Sync;
}

impl Service for () {
    trait Request = Send + Sync;
}

fn requires_request<RequestImpl>(request: RequestImpl)
where
    RequestImpl: <() as Service>::Request,
{
    let _ = request;
    println!("ok");
}

fn main() {
    requires_request(0);
}
