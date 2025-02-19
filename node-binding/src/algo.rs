use algo::{Algo, AlgoType};
use napi_derive::napi;
use std::ops::Deref;

#[napi(js_name = "Algo")]
pub struct JsAlgo {
  inner: Algo,
}

#[napi]
impl JsAlgo {
  #[napi(constructor)]
  pub fn new(name: String) -> Self {
    let r#type = match name.as_str() {
      "blake3" => AlgoType::Blake3,
      _ => AlgoType::Default,
    };
    Self {
      inner: Algo::new(r#type),
    }
  }

  #[napi]
  pub fn hash(&self, v: String) -> String {
    self.inner.hash(v.as_str())
  }

  #[napi]
  pub fn get_name(&self) -> String {
    match self.inner.r#type {
      AlgoType::Blake3 => "blake3".to_string(),
      AlgoType::Default => "default".to_string(),
    }
  }
}

impl Deref for JsAlgo {
  type Target = Algo;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn js_algo_should_work() {
    let algo = JsAlgo::new("blake3".to_string());
    assert_eq!(
      algo.hash("hello".to_string()),
      "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
    );
    assert_eq!(algo.get_name(), "blake3");

    let algo = JsAlgo::new("default".to_string());
    assert_eq!(algo.hash("hello".to_string()), "16156531084128653017");
    assert_eq!(algo.get_name(), "default");
  }
}
