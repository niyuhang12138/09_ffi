use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// pretend this is a heavy operation, CPU intensive
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error: a.len()!= b.len()"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}
