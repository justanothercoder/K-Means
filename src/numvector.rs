use std::num::Float;
use std::iter;
use std::iter::count;
use std::ops::{Add, Sub, Mul};

#[derive (Show)]
pub struct vector {
    pub data: Vec<f32>
}

impl vector {

    pub fn zero(size: usize) -> vector {
        vector { data: iter::count(0.0, 0.0).take(size).collect::<Vec<f32>>() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn squared_length(&self) -> f32 {
        self.data.iter().fold(0.0, |sum, x| { sum + x * x })
    }

    pub fn length(&self) -> f32 {
        return self.squared_length().sqrt()
    }

    pub fn clone(&self) -> vector {
        vector { data: self.data.clone() }
    }
}

impl<'a> Add for &'a vector {
    type Output = vector;
    fn add(self, rhs: &'a vector) -> vector {
        vector { data: self.data.iter().zip(rhs.data.iter()).map(|(a, b)| { a + b }).collect::<Vec<f32>>() }
    }
}

impl<'a> Sub for &'a vector {
    type Output = vector;
    fn sub(self, rhs: &'a vector) -> vector {
        vector { data: self.data.iter().zip(rhs.data.iter()).map(|(a, b)| { a - b }).collect::<Vec<f32>>() }
    }
}

impl<'a> Mul<f32> for &'a vector {
    type Output = vector;
    fn mul(self, f: f32) -> vector {
        vector { data: self.data.iter().map(|x| { x * f }).collect::<Vec<f32>>() }
    }
}

impl<'a> Mul<f32> for &'a mut vector {
    type Output = vector;
    fn mul(self, f: f32) -> vector {
        vector { data: self.data.iter().map(|x| { x * f }).collect::<Vec<f32>>() }
    }
}
