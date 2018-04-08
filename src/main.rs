#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

#[macro_use]
extern crate quicli;
extern crate cargo_name as lib;

use lib::Availability;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Args {
  /// The name of the crate you're targeting.
  name: String,
}

main!(|args: Args| match lib::get(&args.name)? {
  Availability::Unavailable => println!("Unavailable."),
  Availability::Available => println!("Available."),
  Availability::Unknown => println!("Unknown status code returned."),
});
