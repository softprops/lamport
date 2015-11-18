extern crate rustc_serialize;

pub mod rep;
pub use rep::{Context, Payload};
use std::io::{self, Read, Write};
use rustc_serialize::{Decodable, Encodable, json};

fn exec<E: Encodable, D: Decodable>(
    handler: fn(Context, Payload<D>) -> E
) {
    //
}

#[test]
fn it_works() {
}
