#![deny(clippy::all)]

mod algo;

pub use algo::JsAlgo;

use napi_derive::napi;

#[napi]
pub fn plus(a: u32, b: u32) -> u32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn plus_should_work() {
    assert_eq!(plus(1, 2), 3);
  }
}
