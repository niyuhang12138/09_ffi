use algo::{multiply, Matrix};
use napi::{Error, Status};
use napi_derive::napi;

#[napi(js_name = "Matrix")]
#[derive(Debug)]
pub struct JsMatrix {
  inner: Matrix<f64>,
}

#[napi]
impl JsMatrix {
  #[napi(constructor)]
  pub fn new(data: Vec<Vec<f64>>) -> napi::Result<Self> {
    if data.is_empty() || data[0].is_empty() {
      return Err(Error::new(
        Status::InvalidArg,
        "row should not be empty or col should not be empty",
      ));
    }

    let row = data.len();
    let col = data[0].len();

    let data = data.into_iter().flatten().collect::<Vec<_>>();

    Ok(Self {
      inner: Matrix::new(data, row, col),
    })
  }

  #[napi]
  pub fn mul(&self, other: &JsMatrix) -> napi::Result<Self> {
    let result = multiply(&self.inner, &other.inner).unwrap();
    Ok(Self { inner: result })
  }

  #[napi]
  pub fn multiply(&self, other: Vec<Vec<f64>>) -> napi::Result<Self> {
    let other = JsMatrix::new(other)?;
    self.mul(&other)
  }

  #[napi]
  pub fn display(&self) -> String {
    format!("{}", self.inner)
  }
}
