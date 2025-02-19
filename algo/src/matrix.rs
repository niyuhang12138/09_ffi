use crate::vector::{dot_product, Vector};
use anyhow::{anyhow, Result};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.row != b.col"));
    }

    // generate 4 threads which receive msg and do dot product
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (sender, receiver) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in receiver {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send error: {e:?}");
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            sender
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col = Vector::new(
                b.data[j..]
                    .iter()
                    .step_by(b.col)
                    .copied()
                    .collect::<Vec<_>>(),
            );

            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (sender, receiver) = oneshot::channel();
            let msg = Msg::new(input, sender);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {e:?}");
            }
            receivers.push(receiver);
        }
    }

    // map/reduce: reduce phase
    for receiver in receivers {
        let output = receiver.recv()?;
        data[output.idx] = output.value;
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[allow(unused)]
impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T: Display> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply error")
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{c:?}"), "Matrix(row=2, col=2, {22 28, 49 64})");
        Ok(())
    }
}
