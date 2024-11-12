#![allow(unused_variables)]

use std::fmt;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone)]
pub struct Data<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
}

impl<T> Data<T>
where
    T: Clone
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + PartialOrd
        + From<u8>
        + fmt::Display,
{
    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Self {
        Data { data, shape }
    }

    pub fn send(self) -> Self {
        self
    }

    pub fn slice(&self, start: usize, end: usize) -> Result<Self, &'static str> {
        if start > end || end > self.data.len() {
            return Err("Invalid slice indices");
        }
        Ok(Data {
            data: self.data[start..end].to_vec(),
            shape: self.shape.clone(),
        })
    }

    pub fn add(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn shape(&self) -> usize {
        self.data.len()
    }

    pub fn reshape(&mut self, new_shape: usize) -> Result<(), &'static str> {
        if new_shape == 0 {
            return Err("New shape must be greater than 0");
        }
        self.data.resize(new_shape, T::from(0));
        Ok(())
    }

    pub fn sum(&self) -> T {
        self.data.iter().cloned().fold(T::from(0), |acc, x| acc + x)
    }

    pub fn mean(&self) -> T {
        let sum = self.sum();
        sum / T::from(self.data.len() as u8)
    }

    pub fn min(&self) -> Option<&T> {
        self.data.iter().min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn max(&self) -> Option<&T> {
        self.data.iter().max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn dot(&self, other: &Self) -> Result<T, &'static str> {
        if self.data.len() != other.data.len() {
            return Err("Vectors must be of the same length for dot product");
        }
        Ok(self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a.clone() * b.clone())
            .fold(T::from(0), |acc, x| acc + x))
    }
}

// Implementing the Display trait for Data struct
impl<T> fmt::Display for Data<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data: [")?;
        for (i, item) in self.data.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "] , Shape: {:?}", self.shape)
    }
}
