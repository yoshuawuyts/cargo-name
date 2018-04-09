#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

#[macro_use]
extern crate failure;
extern crate reqwest;

use failure::Error;
use reqwest::{StatusCode, Url};

/// A description of availability on crates.io.
pub enum Availability {
  /// Crate is available.
  Available,
  /// Crate is unavailable.
  Unavailable,
  /// Availability is unknown because an unknown status code was returned.
  Unknown,
}

/// Get the availability for a crate on crates.io.
pub fn get(name: &str) -> Result<Availability, Error> {
  ensure!(
    !name.is_empty(),
    "name should be more than 0 characters"
  );
  let addr = format!("https://crates.io/api/v1/crates/{}", name);
  let url = Url::parse(&addr)?;
  let res = reqwest::get(url)?;
  let status = match res.status() {
    StatusCode::Ok => Availability::Unavailable,
    StatusCode::NotFound => Availability::Available,
    _ => Availability::Unknown,
  };
  Ok(status)
}
