//! The rhythm shader language standard math library. This module implements
//! some basic GLSL synonymous types including:
//! * The vector tuple types `V2<T>`, `V3<T>` and `V4<T>`
//! * The matrix tuple types `Mat2<T>`, `Mat2x3<T>`, `Mat2x4<T>`, `Mat3<T>`,
//! `Mat3x2<T>`, `Mat3x4<T>`, `Mat4<T>`, `Mat4x2<T>` and `Mat4x3<T>`
extern crate num;
use super::primative::Primative;
use num::{Float, Num};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub mod ops {
  pub trait DotProduct {
    type Output;
    fn dot(self, rhs: Self) -> Self::Output;
  }

  pub trait CrossProduct {
    type Output;
    fn cross(self, rhs: Self) -> Self::Output;
  }

  pub trait EntrywiseProduct {
    type Output;
    fn entrywise(self, rhs: Self) -> Self::Output;
  }

  pub trait Norm {
    type Output;
    fn norm(&self) -> Self::Output;
  }

  pub trait Normalize {
    type Output;
    fn normalize(&self) -> Self::Output;
  }
}

pub use ops::*;

trait Tuple2 {
  type Member;
  fn make(first: Self::Member, second: Self::Member) -> Self;
  fn first(&self) -> &Self::Member;
  fn second(&self) -> &Self::Member;
}

trait Tuple3 {
  type Member;
  fn make(
    first: Self::Member,
    second: Self::Member,
    third: Self::Member,
  ) -> Self;
  fn first(&self) -> &Self::Member;
  fn second(&self) -> &Self::Member;
  fn third(&self) -> &Self::Member;
}

trait Tuple4 {
  type Member;
  fn make(
    first: Self::Member,
    second: Self::Member,
    third: Self::Member,
    fourth: Self::Member,
  ) -> Self;
  fn first(&self) -> &Self::Member;
  fn second(&self) -> &Self::Member;
  fn third(&self) -> &Self::Member;
  fn fourth(&self) -> &Self::Member;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V2<T>(pub T, pub T)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V3<T>(pub T, pub T, pub T)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V4<T>(pub T, pub T, pub T, pub T)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat2<T>(pub V2<T>, pub V2<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat2x3<T>(pub V3<T>, pub V3<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat2x4<T>(pub V4<T>, pub V4<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3<T>(pub V3<T>, pub V3<T>, pub V3<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3x2<T>(pub V2<T>, pub V2<T>, pub V2<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3x4<T>(pub V4<T>, pub V4<T>, pub V4<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4<T>(pub V4<T>, pub V4<T>, pub V4<T>, pub V4<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4x2<T>(pub V2<T>, pub V2<T>, pub V2<T>, pub V2<T>)
where
  T: Primative;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4x3<T>(pub V3<T>, pub V3<T>, pub V3<T>, pub V3<T>)
where
  T: Primative;

impl<T: Primative + Num> Tuple2 for V2<T> {
  type Member = T;
  fn make(first: T, second: T) -> Self {
    Self(first, second)
  }
  fn first(&self) -> &T {
    &self.0
  }
  fn second(&self) -> &T {
    &self.1
  }
}

impl<T: Primative + Num + Clone> V2<T> {
  pub fn new(value: T) -> Self {
    Self(value.clone(), value)
  }
}

impl<T: Primative + Num + Neg<Output = T>> V2<T> {
  pub fn up() -> Self {
    Self(num::zero(), num::one())
  }

  pub fn down() -> Self {
    Self(num::zero(), -num::one::<T>())
  }

  pub fn left() -> Self {
    Self(-num::one::<T>(), num::zero())
  }

  pub fn right() -> Self {
    Self(num::one::<T>(), num::zero())
  }
}

impl<T: Primative + Num + Clone> From<(T, T)> for V2<T> {
  fn from(value: (T, T)) -> V2<T> {
    V2(value.0, value.1)
  }
}

impl<T: Primative + Num> Tuple3 for V3<T> {
  type Member = T;
  fn make(first: T, second: T, third: T) -> Self {
    Self(first, second, third)
  }
  fn first(&self) -> &T {
    &self.0
  }
  fn second(&self) -> &T {
    &self.1
  }
  fn third(&self) -> &T {
    &self.2
  }
}

impl<T: Primative + Num + Clone> V3<T> {
  pub fn new(value: T) -> Self {
    Self(value.clone(), value.clone(), value)
  }
}

impl<T: Primative + Num + Neg<Output = T>> V3<T> {
  pub fn up() -> Self {
    Self(num::zero(), num::one(), num::zero())
  }

  pub fn down() -> Self {
    Self(num::zero(), -num::one::<T>(), num::zero())
  }

  pub fn left() -> Self {
    Self(-num::one::<T>(), num::zero(), num::zero())
  }

  pub fn right() -> Self {
    Self(num::one(), num::zero(), num::zero())
  }

  pub fn front() -> Self {
    Self(num::zero(), num::zero(), num::one())
  }

  pub fn back() -> Self {
    Self(num::zero(), num::zero(), -num::one::<T>())
  }
}

impl<T: Primative + Num + Clone> From<(T, T, T)> for V3<T> {
  fn from(value: (T, T, T)) -> V3<T> {
    V3(value.0, value.1, value.2)
  }
}

impl<T: Primative + Num + Clone> From<(T, V2<T>)> for V3<T> {
  fn from(value: (T, V2<T>)) -> V3<T> {
    V3(value.0, value.1 .0, value.1 .1)
  }
}

impl<T: Primative + Num + Clone> From<(V2<T>, T)> for V3<T> {
  fn from(value: (V2<T>, T)) -> V3<T> {
    V3(value.0 .0, value.0 .1, value.1)
  }
}

impl<T: Primative + Num> Tuple4 for V4<T> {
  type Member = T;
  fn make(first: T, second: T, third: T, fourth: T) -> Self {
    Self(first, second, third, fourth)
  }
  fn first(&self) -> &T {
    &self.0
  }
  fn second(&self) -> &T {
    &self.1
  }
  fn third(&self) -> &T {
    &self.2
  }
  fn fourth(&self) -> &T {
    &self.3
  }
}

impl<T: Primative + Num + Clone> V4<T> {
  pub fn new(value: T) -> Self {
    Self(value.clone(), value.clone(), value.clone(), value)
  }
}

impl<T: Primative + Num + Neg<Output = T>> V4<T> {
  pub fn up() -> Self {
    Self(num::zero(), num::one(), num::zero(), num::one())
  }

  pub fn down() -> Self {
    Self(num::zero(), -num::one::<T>(), num::zero(), num::one())
  }

  pub fn left() -> Self {
    Self(-num::one::<T>(), num::zero(), num::zero(), num::one())
  }

  pub fn right() -> Self {
    Self(num::one(), num::zero(), num::zero(), num::one())
  }

  pub fn front() -> Self {
    Self(num::zero(), num::zero(), num::one(), num::one())
  }

  pub fn back() -> Self {
    Self(num::zero(), num::zero(), -num::one::<T>(), num::one())
  }
}

impl<T: Primative + Num + Clone> From<(T, T, T, T)> for V4<T> {
  fn from(value: (T, T, T, T)) -> V4<T> {
    V4(value.0, value.1, value.2, value.3)
  }
}

impl<T: Primative + Num + Clone> From<(T, T, V2<T>)> for V4<T> {
  fn from(value: (T, T, V2<T>)) -> V4<T> {
    V4(value.0, value.1, value.2 .0, value.2 .1)
  }
}

impl<T: Primative + Num + Clone> From<(V2<T>, T, T)> for V4<T> {
  fn from(value: (V2<T>, T, T)) -> V4<T> {
    V4(value.0 .0, value.0 .1, value.1, value.2)
  }
}

impl<T: Primative + Num + Clone> From<(V2<T>, V2<T>)> for V4<T> {
  fn from(value: (V2<T>, V2<T>)) -> V4<T> {
    V4(value.0 .0, value.0 .1, value.1 .0, value.1 .1)
  }
}

impl<T: Primative + Num + Clone> From<(T, V2<T>, T)> for V4<T> {
  fn from(value: (T, V2<T>, T)) -> V4<T> {
    V4(value.0, value.1 .0, value.1 .1, value.2)
  }
}

impl<T: Primative + Num + Clone> From<(T, V3<T>)> for V4<T> {
  fn from(value: (T, V3<T>)) -> V4<T> {
    V4(value.0, value.1 .0, value.1 .1, value.1 .2)
  }
}

impl<T: Primative + Num + Clone> From<(V3<T>, T)> for V4<T> {
  fn from(value: (V3<T>, T)) -> V4<T> {
    V4(value.0 .0, value.0 .1, value.0 .2, value.1)
  }
}

impl<T: Primative + Num> Tuple2 for Mat2<T> {
  type Member = V2<T>;
  fn make(first: V2<T>, second: V2<T>) -> Self {
    Self(first, second)
  }
  fn first(&self) -> &V2<T> {
    &self.0
  }
  fn second(&self) -> &V2<T> {
    &self.1
  }
}

impl<T: Primative + Num + Clone> Mat2<T> {
  pub fn new(value: T) -> Self {
    Self(V2(value.clone(), num::zero()), V2(num::zero(), value))
  }

  pub fn all(value: T) -> Self {
    Self(V2::new(value.clone()), V2::new(value))
  }

  pub fn row_0(&self) -> V2<T> {
    V2(self.0 .0.clone(), self.1 .0.clone())
  }

  pub fn row_1(&self) -> V2<T> {
    V2(self.0 .1.clone(), self.1 .1.clone())
  }
}

impl<T: Primative + Num> Tuple2 for Mat2x3<T> {
  type Member = V3<T>;
  fn make(first: V3<T>, second: V3<T>) -> Self {
    Self(first, second)
  }
  fn first(&self) -> &V3<T> {
    &self.0
  }
  fn second(&self) -> &V3<T> {
    &self.1
  }
}

impl<T: Primative + Num + Clone> Mat2x3<T> {
  pub fn new(value: T) -> Self {
    Self(
      V3(value.clone(), num::zero(), num::zero()),
      V3(num::zero(), value, num::zero()),
    )
  }

  pub fn all(value: T) -> Self {
    Self(V3::new(value.clone()), V3::new(value))
  }

  pub fn row_0(&self) -> V2<T> {
    V2(self.0 .0.clone(), self.1 .0.clone())
  }

  pub fn row_1(&self) -> V2<T> {
    V2(self.0 .1.clone(), self.1 .1.clone())
  }

  pub fn row_2(&self) -> V2<T> {
    V2(self.0 .2.clone(), self.1 .2.clone())
  }
}

impl<T: Primative + Num> Tuple2 for Mat2x4<T> {
  type Member = V4<T>;
  fn make(first: V4<T>, second: V4<T>) -> Self {
    Self(first, second)
  }
  fn first(&self) -> &V4<T> {
    &self.0
  }
  fn second(&self) -> &V4<T> {
    &self.1
  }
}

impl<T: Primative + Num + Clone> Mat2x4<T> {
  pub fn new(value: T) -> Self {
    Self(
      V4(value.clone(), num::zero(), num::zero(), num::zero()),
      V4(num::zero(), value, num::zero(), num::zero()),
    )
  }

  pub fn all(value: T) -> Self {
    Self(V4::new(value.clone()), V4::new(value))
  }

  pub fn row_0(&self) -> V2<T> {
    V2(self.0 .0.clone(), self.1 .0.clone())
  }

  pub fn row_1(&self) -> V2<T> {
    V2(self.0 .1.clone(), self.1 .1.clone())
  }

  pub fn row_2(&self) -> V2<T> {
    V2(self.0 .2.clone(), self.1 .2.clone())
  }

  pub fn row_3(&self) -> V2<T> {
    V2(self.0 .3.clone(), self.1 .3.clone())
  }
}

impl<T: Primative + Num> Tuple3 for Mat3<T> {
  type Member = V3<T>;
  fn make(first: V3<T>, second: V3<T>, third: V3<T>) -> Self {
    Self(first, second, third)
  }
  fn first(&self) -> &V3<T> {
    &self.0
  }
  fn second(&self) -> &V3<T> {
    &self.1
  }
  fn third(&self) -> &V3<T> {
    &self.2
  }
}

impl<T: Primative + Num + Clone> Mat3<T> {
  pub fn new(value: T) -> Self {
    Self(
      V3(value.clone(), num::zero(), num::zero()),
      V3(num::zero(), value.clone(), num::zero()),
      V3(num::zero(), num::zero(), value),
    )
  }

  pub fn all(value: T) -> Self {
    Self(
      V3::new(value.clone()),
      V3::new(value.clone()),
      V3::new(value),
    )
  }

  pub fn row_0(&self) -> V3<T> {
    V3(self.0 .0.clone(), self.1 .0.clone(), self.2 .0.clone())
  }

  pub fn row_1(&self) -> V3<T> {
    V3(self.0 .1.clone(), self.1 .1.clone(), self.2 .1.clone())
  }

  pub fn row_2(&self) -> V3<T> {
    V3(self.0 .2.clone(), self.1 .2.clone(), self.2 .2.clone())
  }
}

impl<T: Primative + Num> Tuple3 for Mat3x2<T> {
  type Member = V2<T>;
  fn make(first: V2<T>, second: V2<T>, third: V2<T>) -> Self {
    Self(first, second, third)
  }
  fn first(&self) -> &V2<T> {
    &self.0
  }
  fn second(&self) -> &V2<T> {
    &self.1
  }
  fn third(&self) -> &V2<T> {
    &self.2
  }
}

impl<T: Primative + Num + Clone> Mat3x2<T> {
  pub fn new(value: T) -> Self {
    Self(
      V2(value.clone(), num::zero()),
      V2(num::zero(), value.clone()),
      V2(num::zero(), num::zero()),
    )
  }

  pub fn all(value: T) -> Self {
    Self(
      V2::new(value.clone()),
      V2::new(value.clone()),
      V2::new(value),
    )
  }

  pub fn row_0(&self) -> V3<T> {
    V3(self.0 .0.clone(), self.1 .0.clone(), self.2 .0.clone())
  }

  pub fn row_1(&self) -> V3<T> {
    V3(self.0 .1.clone(), self.1 .1.clone(), self.2 .1.clone())
  }
}

impl<T: Primative + Num> Tuple3 for Mat3x4<T> {
  type Member = V4<T>;
  fn make(first: V4<T>, second: V4<T>, third: V4<T>) -> Self {
    Self(first, second, third)
  }
  fn first(&self) -> &V4<T> {
    &self.0
  }
  fn second(&self) -> &V4<T> {
    &self.1
  }
  fn third(&self) -> &V4<T> {
    &self.2
  }
}

impl<T: Primative + Num + Clone> Mat3x4<T> {
  pub fn new(value: T) -> Self {
    Self(
      V4(value.clone(), num::zero(), num::zero(), num::zero()),
      V4(num::zero(), value.clone(), num::zero(), num::zero()),
      V4(num::zero(), num::zero(), value, num::zero()),
    )
  }

  pub fn all(value: T) -> Self {
    Self(
      V4::new(value.clone()),
      V4::new(value.clone()),
      V4::new(value),
    )
  }

  pub fn row_0(&self) -> V3<T> {
    V3(self.0 .0.clone(), self.1 .0.clone(), self.2 .0.clone())
  }

  pub fn row_1(&self) -> V3<T> {
    V3(self.0 .1.clone(), self.1 .1.clone(), self.2 .1.clone())
  }

  pub fn row_2(&self) -> V3<T> {
    V3(self.0 .2.clone(), self.1 .2.clone(), self.2 .2.clone())
  }

  pub fn row_3(&self) -> V3<T> {
    V3(self.0 .3.clone(), self.1 .3.clone(), self.2 .3.clone())
  }
}

impl<T: Primative + Num> Tuple4 for Mat4<T> {
  type Member = V4<T>;
  fn make(first: V4<T>, second: V4<T>, third: V4<T>, fourth: V4<T>) -> Self {
    Self(first, second, third, fourth)
  }
  fn first(&self) -> &V4<T> {
    &self.0
  }
  fn second(&self) -> &V4<T> {
    &self.1
  }
  fn third(&self) -> &V4<T> {
    &self.2
  }
  fn fourth(&self) -> &V4<T> {
    &self.3
  }
}

impl<T: Primative + Num + Clone> Mat4<T> {
  pub fn new(value: T) -> Self {
    Self(
      V4(value.clone(), num::zero(), num::zero(), num::zero()),
      V4(num::zero(), value.clone(), num::zero(), num::zero()),
      V4(num::zero(), num::zero(), value.clone(), num::zero()),
      V4(num::zero(), num::zero(), num::zero(), value),
    )
  }

  pub fn all(value: T) -> Self {
    Self(
      V4::new(value.clone()),
      V4::new(value.clone()),
      V4::new(value.clone()),
      V4::new(value),
    )
  }

  pub fn row_0(&self) -> V4<T> {
    V4(
      self.0 .0.clone(),
      self.1 .0.clone(),
      self.2 .0.clone(),
      self.3 .0.clone(),
    )
  }

  pub fn row_1(&self) -> V4<T> {
    V4(
      self.0 .1.clone(),
      self.1 .1.clone(),
      self.2 .1.clone(),
      self.3 .1.clone(),
    )
  }

  pub fn row_2(&self) -> V4<T> {
    V4(
      self.0 .2.clone(),
      self.1 .2.clone(),
      self.2 .2.clone(),
      self.3 .2.clone(),
    )
  }

  pub fn row_3(&self) -> V4<T> {
    V4(
      self.0 .3.clone(),
      self.1 .3.clone(),
      self.2 .3.clone(),
      self.3 .3.clone(),
    )
  }
}

impl<T: Primative + Num> Tuple4 for Mat4x2<T> {
  type Member = V2<T>;
  fn make(first: V2<T>, second: V2<T>, third: V2<T>, fourth: V2<T>) -> Self {
    Self(first, second, third, fourth)
  }
  fn first(&self) -> &V2<T> {
    &self.0
  }
  fn second(&self) -> &V2<T> {
    &self.1
  }
  fn third(&self) -> &V2<T> {
    &self.2
  }
  fn fourth(&self) -> &V2<T> {
    &self.3
  }
}

impl<T: Primative + Num + Clone> Mat4x2<T> {
  pub fn new(value: T) -> Self {
    Self(
      V2(value.clone(), num::zero()),
      V2(num::zero(), value.clone()),
      V2(num::zero(), num::zero()),
      V2(num::zero(), num::zero()),
    )
  }

  pub fn all(value: T) -> Self {
    Self(
      V2::new(value.clone()),
      V2::new(value.clone()),
      V2::new(value.clone()),
      V2::new(value),
    )
  }

  pub fn row_0(&self) -> V4<T> {
    V4(
      self.0 .0.clone(),
      self.1 .0.clone(),
      self.2 .0.clone(),
      self.3 .0.clone(),
    )
  }

  pub fn row_1(&self) -> V4<T> {
    V4(
      self.0 .1.clone(),
      self.1 .1.clone(),
      self.2 .1.clone(),
      self.3 .1.clone(),
    )
  }
}

impl<T: Primative + Num> Tuple4 for Mat4x3<T> {
  type Member = V3<T>;
  fn make(first: V3<T>, second: V3<T>, third: V3<T>, fourth: V3<T>) -> Self {
    Self(first, second, third, fourth)
  }
  fn first(&self) -> &V3<T> {
    &self.0
  }
  fn second(&self) -> &V3<T> {
    &self.1
  }
  fn third(&self) -> &V3<T> {
    &self.2
  }
  fn fourth(&self) -> &V3<T> {
    &self.3
  }
}

impl<T: Primative + Num + Clone> Mat4x3<T> {
  pub fn new(value: T) -> Self {
    Self(
      V3(value.clone(), num::zero(), num::zero()),
      V3(num::zero(), value.clone(), num::zero()),
      V3(num::zero(), num::zero(), value.clone()),
      V3(num::zero(), num::zero(), num::zero()),
    )
  }

  pub fn all(value: T) -> Self {
    Self(
      V3::new(value.clone()),
      V3::new(value.clone()),
      V3::new(value.clone()),
      V3::new(value),
    )
  }

  pub fn row_0(&self) -> V4<T> {
    V4(
      self.0 .0.clone(),
      self.1 .0.clone(),
      self.2 .0.clone(),
      self.3 .0.clone(),
    )
  }

  pub fn row_1(&self) -> V4<T> {
    V4(
      self.0 .1.clone(),
      self.1 .1.clone(),
      self.2 .1.clone(),
      self.3 .1.clone(),
    )
  }

  pub fn row_2(&self) -> V4<T> {
    V4(
      self.0 .2.clone(),
      self.1 .2.clone(),
      self.2 .2.clone(),
      self.3 .2.clone(),
    )
  }
}

macro_rules! tuple2_op_impl {
  ($bound:tt, $trait:ty, $name:ident, $type:ty) => {
    impl<T: Primative + Num + $bound<Output = T> + Copy> $trait for $type {
      type Output = Self;
      fn $name(self, rhs: Self) -> Self {
        Self::make(
          self.first().$name(*rhs.first()),
          self.second().$name(*rhs.second()),
        )
      }
    }
  };
}

macro_rules! tuple3_op_impl {
  ($bound:tt, $trait:ty, $name:ident, $type:ty) => {
    impl<T: Primative + Num + $bound<Output = T> + Copy> $trait for $type {
      type Output = Self;
      fn $name(self, rhs: Self) -> Self {
        Self::make(
          self.first().$name(*rhs.first()),
          self.second().$name(*rhs.second()),
          self.third().$name(*rhs.third()),
        )
      }
    }
  };
}

macro_rules! tuple4_op_impl {
  ($bound:tt, $trait:ty, $name:ident, $type:ty) => {
    impl<T: Primative + Num + $bound<Output = T> + Copy> $trait for $type {
      type Output = Self;
      fn $name(self, rhs: Self) -> Self {
        Self::make(
          self.first().$name(*rhs.first()),
          self.second().$name(*rhs.second()),
          self.third().$name(*rhs.third()),
          self.fourth().$name(*rhs.fourth()),
        )
      }
    }
  };
}

tuple2_op_impl!(Add, Add, add, V2<T>);
tuple2_op_impl!(Add, Add, add, Mat2<T>);
tuple2_op_impl!(Add, Add, add, Mat2x3<T>);
tuple2_op_impl!(Add, Add, add, Mat2x4<T>);

tuple3_op_impl!(Add, Add, add, V3<T>);
tuple3_op_impl!(Add, Add, add, Mat3<T>);
tuple3_op_impl!(Add, Add, add, Mat3x2<T>);
tuple3_op_impl!(Add, Add, add, Mat3x4<T>);

tuple4_op_impl!(Add, Add, add, V4<T>);
tuple4_op_impl!(Add, Add, add, Mat4<T>);
tuple4_op_impl!(Add, Add, add, Mat4x2<T>);
tuple4_op_impl!(Add, Add, add, Mat4x3<T>);

tuple2_op_impl!(Sub, Sub, sub, V2<T>);
tuple2_op_impl!(Sub, Sub, sub, Mat2<T>);
tuple2_op_impl!(Sub, Sub, sub, Mat2x3<T>);
tuple2_op_impl!(Sub, Sub, sub, Mat2x4<T>);

tuple3_op_impl!(Sub, Sub, sub, V3<T>);
tuple3_op_impl!(Sub, Sub, sub, Mat3<T>);
tuple3_op_impl!(Sub, Sub, sub, Mat3x2<T>);
tuple3_op_impl!(Sub, Sub, sub, Mat3x4<T>);

tuple4_op_impl!(Sub, Sub, sub, V4<T>);
tuple4_op_impl!(Sub, Sub, sub, Mat4<T>);
tuple4_op_impl!(Sub, Sub, sub, Mat4x2<T>);
tuple4_op_impl!(Sub, Sub, sub, Mat4x3<T>);

macro_rules! tuple2_scalar_op_impl {
  ($bound:tt, $trait:ty, $name:ident, $type:ty) => {
    impl<T: Primative + Num + $bound<Output = T> + Copy> $trait for $type {
      type Output = Self;
      fn $name(self, rhs: T) -> Self {
        Self::make(self.first().$name(rhs), self.second().$name(rhs))
      }
    }
  };
}

macro_rules! tuple3_scalar_op_impl {
  ($bound:tt, $trait:ty, $name:ident, $type:ty) => {
    impl<T: Primative + Num + $bound<Output = T> + Copy> $trait for $type {
      type Output = Self;
      fn $name(self, rhs: T) -> Self {
        Self::make(
          self.first().$name(rhs),
          self.second().$name(rhs),
          self.third().$name(rhs),
        )
      }
    }
  };
}

macro_rules! tuple4_scalar_op_impl {
  ($bound:tt, $trait:ty, $name:ident, $type:ty) => {
    impl<T: Primative + Num + $bound<Output = T> + Copy> $trait for $type {
      type Output = Self;
      fn $name(self, rhs: T) -> Self {
        Self::make(
          self.first().$name(rhs),
          self.second().$name(rhs),
          self.third().$name(rhs),
          self.fourth().$name(rhs),
        )
      }
    }
  };
}

tuple2_scalar_op_impl!(Div, Div<T>, div, V2<T>);
tuple2_scalar_op_impl!(Div, Div<T>, div, Mat2<T>);
tuple2_scalar_op_impl!(Div, Div<T>, div, Mat2x3<T>);
tuple2_scalar_op_impl!(Div, Div<T>, div, Mat2x4<T>);

tuple3_scalar_op_impl!(Div, Div<T>, div, V3<T>);
tuple3_scalar_op_impl!(Div, Div<T>, div, Mat3<T>);
tuple3_scalar_op_impl!(Div, Div<T>, div, Mat3x2<T>);
tuple3_scalar_op_impl!(Div, Div<T>, div, Mat3x4<T>);

tuple4_scalar_op_impl!(Div, Div<T>, div, V4<T>);
tuple4_scalar_op_impl!(Div, Div<T>, div, Mat4<T>);
tuple4_scalar_op_impl!(Div, Div<T>, div, Mat4x2<T>);
tuple4_scalar_op_impl!(Div, Div<T>, div, Mat4x3<T>);

tuple2_scalar_op_impl!(Mul, Mul<T>, mul, V2<T>);
tuple3_scalar_op_impl!(Mul, Mul<T>, mul, V3<T>);
tuple4_scalar_op_impl!(Mul, Mul<T>, mul, V4<T>);

macro_rules! mat_mult_vector {
  (2, $self:ident, $rhs:ident, $comp:tt) => {
    V2(
      $self.row_0().dot($rhs.$comp.clone()),
      $self.row_1().dot($rhs.$comp),
    )
  };
  (3, $self:ident, $rhs:ident, $comp:tt) => {
    V3(
      $self.row_0().dot($rhs.$comp.clone()),
      $self.row_1().dot($rhs.$comp.clone()),
      $self.row_2().dot($rhs.$comp),
    )
  };
  (4, $self:ident, $rhs:ident, $comp:tt) => {
    V4(
      $self.row_0().dot($rhs.$comp.clone()),
      $self.row_1().dot($rhs.$comp.clone()),
      $self.row_2().dot($rhs.$comp.clone()),
      $self.row_3().dot($rhs.$comp),
    )
  };
}

impl<T> Mul<V2<T>> for Mat2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V2<T>;
  fn mul(self, rhs: V2<T>) -> Self::Output {
    V2(self.row_0().dot(rhs.clone()), self.row_1().dot(rhs))
  }
}

impl<T> Mul<Mat2<T>> for Mat2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2<T>;
  fn mul(self, rhs: Mat2<T>) -> Self::Output {
    Mat2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat3x2<T>> for Mat2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat3x2<T>;
  fn mul(self, rhs: Mat3x2<T>) -> Self::Output {
    Mat3x2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
      mat_mult_vector!(2, self, rhs, 2),
    )
  }
}

impl<T> Mul<Mat4x2<T>> for Mat2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4x2<T>;
  fn mul(self, rhs: Mat4x2<T>) -> Self::Output {
    Mat4x2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
      mat_mult_vector!(2, self, rhs, 2),
      mat_mult_vector!(2, self, rhs, 3),
    )
  }
}

impl<T> Mul<V2<T>> for Mat2x3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V3<T>;
  fn mul(self, rhs: V2<T>) -> Self::Output {
    V3(
      self.row_0().dot(rhs.clone()),
      self.row_1().dot(rhs.clone()),
      self.row_2().dot(rhs),
    )
  }
}

impl<T> Mul<Mat2<T>> for Mat2x3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2x3<T>;
  fn mul(self, rhs: Mat2<T>) -> Self::Output {
    Mat2x3(
      mat_mult_vector!(3, self, rhs, 0),
      mat_mult_vector!(3, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat3x2<T>> for Mat2x3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat3<T>;
  fn mul(self, rhs: Mat3x2<T>) -> Self::Output {
    Mat3(
      mat_mult_vector!(3, self, rhs, 0),
      mat_mult_vector!(3, self, rhs, 1),
      mat_mult_vector!(3, self, rhs, 2),
    )
  }
}

impl<T> Mul<V2<T>> for Mat2x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V4<T>;
  fn mul(self, rhs: V2<T>) -> Self::Output {
    V4(
      self.row_0().dot(rhs.clone()),
      self.row_1().dot(rhs.clone()),
      self.row_2().dot(rhs.clone()),
      self.row_3().dot(rhs),
    )
  }
}

impl<T> Mul<Mat2<T>> for Mat2x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2x4<T>;
  fn mul(self, rhs: Mat2<T>) -> Self::Output {
    Mat2x4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat3x2<T>> for Mat2x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat3x4<T>;
  fn mul(self, rhs: Mat3x2<T>) -> Self::Output {
    Mat3x4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
      mat_mult_vector!(4, self, rhs, 2),
    )
  }
}

impl<T> Mul<Mat4x2<T>> for Mat2x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4<T>;
  fn mul(self, rhs: Mat4x2<T>) -> Self::Output {
    Mat4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
      mat_mult_vector!(4, self, rhs, 2),
      mat_mult_vector!(4, self, rhs, 3),
    )
  }
}

impl<T> Mul<V3<T>> for Mat3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V3<T>;
  fn mul(self, rhs: V3<T>) -> Self::Output {
    V3(
      self.row_0().dot(rhs.clone()),
      self.row_1().dot(rhs.clone()),
      self.row_2().dot(rhs),
    )
  }
}

impl<T> Mul<Mat2x3<T>> for Mat3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2x3<T>;
  fn mul(self, rhs: Mat2x3<T>) -> Self::Output {
    Mat2x3(
      mat_mult_vector!(3, self, rhs, 0),
      mat_mult_vector!(3, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat4x3<T>> for Mat3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4x3<T>;
  fn mul(self, rhs: Mat4x3<T>) -> Self::Output {
    Mat4x3(
      mat_mult_vector!(3, self, rhs, 0),
      mat_mult_vector!(3, self, rhs, 1),
      mat_mult_vector!(3, self, rhs, 2),
      mat_mult_vector!(3, self, rhs, 3),
    )
  }
}

impl<T> Mul<V3<T>> for Mat3x2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V2<T>;
  fn mul(self, rhs: V3<T>) -> Self::Output {
    V2(self.row_0().dot(rhs.clone()), self.row_1().dot(rhs))
  }
}

impl<T> Mul<Mat2x3<T>> for Mat3x2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2<T>;
  fn mul(self, rhs: Mat2x3<T>) -> Self::Output {
    Mat2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat4x3<T>> for Mat3x2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4x2<T>;
  fn mul(self, rhs: Mat4x3<T>) -> Self::Output {
    Mat4x2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
      mat_mult_vector!(2, self, rhs, 2),
      mat_mult_vector!(2, self, rhs, 3),
    )
  }
}

impl<T> Mul<V3<T>> for Mat3x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V4<T>;
  fn mul(self, rhs: V3<T>) -> Self::Output {
    V4(
      self.row_0().dot(rhs.clone()),
      self.row_1().dot(rhs.clone()),
      self.row_2().dot(rhs.clone()),
      self.row_3().dot(rhs),
    )
  }
}

impl<T> Mul<Mat2x3<T>> for Mat3x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2x4<T>;
  fn mul(self, rhs: Mat2x3<T>) -> Self::Output {
    Mat2x4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat3<T>> for Mat3x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat3x4<T>;
  fn mul(self, rhs: Mat3<T>) -> Self::Output {
    Mat3x4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
      mat_mult_vector!(4, self, rhs, 2),
    )
  }
}

impl<T> Mul<Mat4x3<T>> for Mat3x4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4<T>;
  fn mul(self, rhs: Mat4x3<T>) -> Self::Output {
    Mat4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
      mat_mult_vector!(4, self, rhs, 2),
      mat_mult_vector!(4, self, rhs, 3),
    )
  }
}

impl<T> Mul<V4<T>> for Mat4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V4<T>;
  fn mul(self, rhs: V4<T>) -> Self::Output {
    V4(
      self.row_0().dot(rhs.clone()),
      self.row_1().dot(rhs.clone()),
      self.row_2().dot(rhs.clone()),
      self.row_3().dot(rhs),
    )
  }
}

impl<T> Mul<Mat2x4<T>> for Mat4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2x4<T>;
  fn mul(self, rhs: Mat2x4<T>) -> Self::Output {
    Mat2x4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat3x4<T>> for Mat4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat3x4<T>;
  fn mul(self, rhs: Mat3x4<T>) -> Self::Output {
    Mat3x4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
      mat_mult_vector!(4, self, rhs, 2),
    )
  }
}

impl<T> Mul<Mat4<T>> for Mat4<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4<T>;
  fn mul(self, rhs: Mat4<T>) -> Self::Output {
    Mat4(
      mat_mult_vector!(4, self, rhs, 0),
      mat_mult_vector!(4, self, rhs, 1),
      mat_mult_vector!(4, self, rhs, 2),
      mat_mult_vector!(4, self, rhs, 3),
    )
  }
}

impl<T> Mul<V4<T>> for Mat4x2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V2<T>;
  fn mul(self, rhs: V4<T>) -> Self::Output {
    V2(self.row_0().dot(rhs.clone()), self.row_1().dot(rhs.clone()))
  }
}

impl<T> Mul<Mat2x4<T>> for Mat4x2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2<T>;
  fn mul(self, rhs: Mat2x4<T>) -> Self::Output {
    Mat2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat3x4<T>> for Mat4x2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat3x2<T>;
  fn mul(self, rhs: Mat3x4<T>) -> Self::Output {
    Mat3x2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
      mat_mult_vector!(2, self, rhs, 2),
    )
  }
}

impl<T> Mul<Mat4<T>> for Mat4x2<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4x2<T>;
  fn mul(self, rhs: Mat4<T>) -> Self::Output {
    Mat4x2(
      mat_mult_vector!(2, self, rhs, 0),
      mat_mult_vector!(2, self, rhs, 1),
      mat_mult_vector!(2, self, rhs, 2),
      mat_mult_vector!(2, self, rhs, 3),
    )
  }
}

impl<T> Mul<V4<T>> for Mat4x3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = V3<T>;
  fn mul(self, rhs: V4<T>) -> Self::Output {
    V3(
      self.row_0().dot(rhs.clone()),
      self.row_1().dot(rhs.clone()),
      self.row_2().dot(rhs),
    )
  }
}

impl<T> Mul<Mat2x4<T>> for Mat4x3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat2x3<T>;
  fn mul(self, rhs: Mat2x4<T>) -> Self::Output {
    Mat2x3(
      mat_mult_vector!(3, self, rhs, 0),
      mat_mult_vector!(3, self, rhs, 1),
    )
  }
}

impl<T> Mul<Mat3x4<T>> for Mat4x3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat3<T>;
  fn mul(self, rhs: Mat3x4<T>) -> Self::Output {
    Mat3(
      mat_mult_vector!(3, self, rhs, 0),
      mat_mult_vector!(3, self, rhs, 1),
      mat_mult_vector!(3, self, rhs, 2),
    )
  }
}

impl<T> Mul<Mat4<T>> for Mat4x3<T>
where
  T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy,
{
  type Output = Mat4x3<T>;
  fn mul(self, rhs: Mat4<T>) -> Self::Output {
    Mat4x3(
      mat_mult_vector!(3, self, rhs, 0),
      mat_mult_vector!(3, self, rhs, 1),
      mat_mult_vector!(3, self, rhs, 2),
      mat_mult_vector!(3, self, rhs, 3),
    )
  }
}

impl<T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy> ops::DotProduct
  for V2<T>
{
  type Output = T;
  fn dot(self, rhs: Self) -> T {
    self.0 * rhs.0 + self.1 * rhs.1
  }
}

impl<T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy> ops::DotProduct
  for V3<T>
{
  type Output = T;
  fn dot(self, rhs: Self) -> T {
    self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
  }
}

impl<T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy> ops::DotProduct
  for V4<T>
{
  type Output = T;
  fn dot(self, rhs: Self) -> T {
    self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 + self.3 * rhs.3
  }
}

impl<T: Primative + Num + Mul<Output = T> + Sub<Output = T> + Copy> ops::CrossProduct
  for V3<T>
{
  type Output = Self;
  fn cross(self, rhs: Self) -> Self {
    Self(
      self.1 * rhs.2 - self.2 * rhs.1,
      self.2 * rhs.0 - self.0 * rhs.2,
      self.0 * rhs.1 - self.1 * rhs.0,
    )
  }
}

impl<T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy> ops::EntrywiseProduct
  for V2<T>
{
  type Output = Self;
  fn entrywise(self, rhs: Self) -> Self {
    Self(self.0 * rhs.0, self.1 * rhs.1)
  }
}

tuple2_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat2<T>);
tuple2_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat2x3<T>);
tuple2_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat2x4<T>);

impl<T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy> ops::EntrywiseProduct
  for V3<T>
{
  type Output = Self;
  fn entrywise(self, rhs: Self) -> Self {
    Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
  }
}

tuple3_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat3<T>);
tuple3_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat3x2<T>);
tuple3_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat3x4<T>);

impl<T: Primative + Num + Mul<Output = T> + Add<Output = T> + Copy> ops::EntrywiseProduct
  for V4<T>
{
  type Output = Self;
  fn entrywise(self, rhs: Self) -> Self {
    Self(
      self.0 * rhs.0,
      self.1 * rhs.1,
      self.2 * rhs.2,
      self.3 * rhs.3,
    )
  }
}

tuple4_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat4<T>);
tuple4_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat4x2<T>);
tuple4_op_impl!(Add, ops::EntrywiseProduct, entrywise, Mat4x3<T>);

impl<T: Primative + Float + Div<Output = T>> ops::Norm for V2<T> {
  type Output = T;
  fn norm(&self) -> T {
    (self.0 * self.0 + self.1 * self.1).sqrt()
  }
}

impl<T: Primative + Float + Div<Output = T>> ops::Norm for V3<T> {
  type Output = T;
  fn norm(&self) -> T {
    (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
  }
}

impl<T: Primative + Float + Div<Output = T>> ops::Norm for V4<T> {
  type Output = T;
  fn norm(&self) -> T {
    (self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3)
      .sqrt()
  }
}

impl<
    C: Float + Div<Output = C>,
    T: ops::Norm<Output = C> + Div<C, Output = T> + Copy,
  > ops::Normalize for T
{
  type Output = T;
  fn normalize(&self) -> T {
    let norm = self.norm();
    self.div(norm)
  }
}

pub type V2f = V2<f32>;
pub type V2d = V2<f64>;
pub type V2i = V2<i32>;
pub type V2u = V2<u32>;
pub type V2b = V2<bool>;

#[cfg(test)]
mod test {
  use super::ops::*;
  use super::*;

  #[test]
  fn new_test() {
    assert_eq!(V2::new(2), V2(2, 2));
    assert_eq!(V3::new(3), V3(3, 3, 3));
    assert_eq!(V4::new(4), V4(4, 4, 4, 4));
  }

  #[test]
  fn all_test() {
    assert_eq!(Mat2::all(2), Mat2(V2::new(2), V2::new(2)));
    assert_eq!(Mat2x3::all(2), Mat2x3(V3::new(2), V3::new(2)));
    assert_eq!(Mat2x4::all(2), Mat2x4(V4::new(2), V4::new(2)));
    assert_eq!(Mat3::all(3), Mat3(V3::new(3), V3::new(3), V3::new(3)));
    assert_eq!(Mat3x2::all(3), Mat3x2(V2::new(3), V2::new(3), V2::new(3)));
    assert_eq!(Mat3x4::all(3), Mat3x4(V4::new(3), V4::new(3), V4::new(3)));
    assert_eq!(
      Mat4::all(4),
      Mat4(V4::new(4), V4::new(4), V4::new(4), V4::new(4))
    );
    assert_eq!(
      Mat4x2::all(4),
      Mat4x2(V2::new(4), V2::new(4), V2::new(4), V2::new(4))
    );
    assert_eq!(
      Mat4x3::all(4),
      Mat4x3(V3::new(4), V3::new(4), V3::new(4), V3::new(4))
    );
  }

  #[test]
  fn add_test() {
    assert_eq!(V2(1, 2) + V2(3, 4), V2(4, 6));
    assert_eq!(V3(1, 2, 3) + V3(4, 5, 6), V3(5, 7, 9));
    assert_eq!(V4(1, 2, 3, 4) + V4(5, 6, 7, 8), V4(6, 8, 10, 12));
    assert_eq!(Mat2::all(1) + Mat2::all(2), Mat2::all(3));
    assert_eq!(Mat2x3::all(1) + Mat2x3::all(2), Mat2x3::all(3));
    assert_eq!(Mat2x4::all(1) + Mat2x4::all(2), Mat2x4::all(3));
    assert_eq!(Mat3::all(1) + Mat3::all(2), Mat3::all(3));
    assert_eq!(Mat3x2::all(1) + Mat3x2::all(2), Mat3x2::all(3));
    assert_eq!(Mat3x4::all(1) + Mat3x4::all(2), Mat3x4::all(3));
    assert_eq!(Mat4::all(1) + Mat4::all(2), Mat4::all(3));
    assert_eq!(Mat4x2::all(1) + Mat4x2::all(2), Mat4x2::all(3));
    assert_eq!(Mat4x3::all(1) + Mat4x3::all(2), Mat4x3::all(3));
  }

  #[test]
  fn sub_test() {
    assert_eq!(V2(1, 2) - V2(3, 4), V2(-2, -2));
    assert_eq!(V3(1, 2, 3) - V3(4, 5, 6), V3(-3, -3, -3));
    assert_eq!(V4(1, 2, 3, 4) - V4(5, 6, 7, 8), V4(-4, -4, -4, -4));
    assert_eq!(Mat2::all(1) - Mat2::all(2), Mat2::all(-1));
    assert_eq!(Mat2x3::all(1) - Mat2x3::all(2), Mat2x3::all(-1));
    assert_eq!(Mat2x4::all(1) - Mat2x4::all(2), Mat2x4::all(-1));
    assert_eq!(Mat3::all(1) - Mat3::all(2), Mat3::all(-1));
    assert_eq!(Mat3x2::all(1) - Mat3x2::all(2), Mat3x2::all(-1));
    assert_eq!(Mat3x4::all(1) - Mat3x4::all(2), Mat3x4::all(-1));
    assert_eq!(Mat4::all(1) - Mat4::all(2), Mat4::all(-1));
    assert_eq!(Mat4x2::all(1) - Mat4x2::all(2), Mat4x2::all(-1));
    assert_eq!(Mat4x3::all(1) - Mat4x3::all(2), Mat4x3::all(-1));
  }

  #[test]
  fn vec_mul_test() {
    assert_eq!(V2(1, 2) * 2, V2(2, 4));
    assert_eq!(V3(1, 2, 3) * 2, V3(2, 4, 6));
    assert_eq!(V4(1, 2, 3, 4) * 2, V4(2, 4, 6, 8));
  }

  #[test]
  fn mat2_mul_test() {
    assert_eq!(Mat2(V2(1, 2), V2(3, 4)) * V2(5, 6), V2(23, 34));
    assert_eq!(
      Mat2(V2(1, 2), V2(3, 4)) * Mat2(V2(5, 6), V2(7, 8)),
      Mat2(V2(23, 34), V2(31, 46))
    );
    assert_eq!(
      Mat2(V2(1, 2), V2(3, 4)) * Mat3x2(V2(5, 6), V2(7, 8), V2(9, 10)),
      Mat3x2(V2(23, 34), V2(31, 46), V2(39, 58))
    );
    assert_eq!(
      Mat2(V2(1, 2), V2(3, 4))
        * Mat4x2(V2(5, 6), V2(7, 8), V2(9, 10), V2(11, 12)),
      Mat4x2(V2(23, 34), V2(31, 46), V2(39, 58), V2(47, 70))
    );
  }

  #[test]
  fn mat2x3_mul_test() {
    assert_eq!(Mat2x3(V3(1, 2, 3), V3(4, 5, 6)) * V2(7, 8), V3(39, 54, 69));
    assert_eq!(
      Mat2x3(V3(1, 2, 3), V3(4, 5, 6)) * Mat2(V2(7, 8), V2(9, 10)),
      Mat2x3(V3(39, 54, 69), V3(49, 68, 87))
    );
    assert_eq!(
      Mat2x3(V3(1, 2, 3), V3(4, 5, 6))
        * Mat3x2(V2(7, 8), V2(9, 10), V2(11, 12)),
      Mat3(V3(39, 54, 69), V3(49, 68, 87), V3(59, 82, 105))
    );
  }

  #[test]
  fn mat2x4_mul_test() {
    assert_eq!(
      Mat2x4(V4(1, 2, 3, 4), V4(5, 6, 7, 8)) * V2(9, 10),
      V4(59, 78, 97, 116)
    );
    assert_eq!(
      Mat2x4(V4(1, 2, 3, 4), V4(5, 6, 7, 8))
        * Mat3x2(V2(9, 10), V2(11, 12), V2(13, 14)),
      Mat3x4(
        V4(59, 78, 97, 116),
        V4(71, 94, 117, 140),
        V4(83, 110, 137, 164)
      )
    );
    assert_eq!(
      Mat2x4(V4(1, 2, 3, 4), V4(5, 6, 7, 8))
        * Mat4x2(V2(9, 10), V2(11, 12), V2(13, 14), V2(15, 16)),
      Mat4(
        V4(59, 78, 97, 116),
        V4(71, 94, 117, 140),
        V4(83, 110, 137, 164),
        V4(95, 126, 157, 188)
      )
    );
  }

  #[test]
  fn mat3_mul_test() {
    assert_eq!(
      Mat3(V3(1, 2, 3), V3(4, 5, 6), V3(7, 8, 9)) * V3(10, 11, 12),
      V3(138, 171, 204)
    );
    assert_eq!(
      Mat3(V3(1, 2, 3), V3(4, 5, 6), V3(7, 8, 9))
        * Mat2x3(V3(10, 11, 12), V3(13, 14, 15)),
      Mat2x3(V3(138, 171, 204), V3(174, 216, 258))
    );
    assert_eq!(
      Mat3(V3(1, 2, 3), V3(4, 5, 6), V3(7, 8, 9))
        * Mat4x3(
          V3(10, 11, 12),
          V3(13, 14, 15),
          V3(16, 17, 18),
          V3(19, 20, 21)
        ),
      Mat4x3(
        V3(138, 171, 204),
        V3(174, 216, 258),
        V3(210, 261, 312),
        V3(246, 306, 366)
      )
    );
  }

  #[test]
  fn mat3x2_mul_test() {
    assert_eq!(
      Mat3x2(V2(1, 2), V2(3, 4), V2(5, 6)) * V3(7, 8, 9),
      V2(76, 100)
    );
    assert_eq!(
      Mat3x2(V2(1, 2), V2(3, 4), V2(5, 6))
        * Mat2x3(V3(10, 11, 12), V3(13, 14, 15)),
      Mat2(V2(103, 136), V2(130, 172))
    );
    assert_eq!(
      Mat3x2(V2(1, 2), V2(3, 4), V2(5, 6))
        * Mat4x3(
          V3(10, 11, 12),
          V3(13, 14, 15),
          V3(16, 17, 18),
          V3(19, 20, 21)
        ),
      Mat4x2(V2(103, 136), V2(130, 172), V2(157, 208), V2(184, 244))
    );
  }

  #[test]
  fn mat3x4_mul_test() {
    assert_eq!(
      Mat3x4(V4(1, 2, 3, 4), V4(5, 6, 7, 8), V4(9, 10, 11, 12))
        * V3(13, 14, 15),
      V4(218, 260, 302, 344)
    );
    assert_eq!(
      Mat3x4(V4(1, 2, 3, 4), V4(5, 6, 7, 8), V4(9, 10, 11, 12))
        * Mat2x3(V3(13, 14, 15), V3(16, 17, 18)),
      Mat2x4(V4(218, 260, 302, 344), V4(263, 314, 365, 416))
    );
    assert_eq!(
      Mat3x4(V4(1, 2, 3, 4), V4(5, 6, 7, 8), V4(9, 10, 11, 12))
        * Mat3(V3(13, 14, 15), V3(16, 17, 18), V3(19, 20, 21)),
      Mat3x4(
        V4(218, 260, 302, 344),
        V4(263, 314, 365, 416),
        V4(308, 368, 428, 488)
      )
    );
    assert_eq!(
      Mat3x4(V4(1, 2, 3, 4), V4(5, 6, 7, 8), V4(9, 10, 11, 12))
        * Mat4x3(
          V3(13, 14, 15),
          V3(16, 17, 18),
          V3(19, 20, 21),
          V3(22, 23, 24)
        ),
      Mat4(
        V4(218, 260, 302, 344),
        V4(263, 314, 365, 416),
        V4(308, 368, 428, 488),
        V4(353, 422, 491, 560)
      )
    );
  }

  #[test]
  fn mat4_mul_test() {
    assert_eq!(
      Mat4(
        V4(1, 2, 3, 4),
        V4(5, 6, 7, 8),
        V4(9, 10, 11, 12),
        V4(13, 14, 15, 16)
      ) * V4(17, 18, 19, 20),
      V4(538, 612, 686, 760)
    );
    assert_eq!(
      Mat4(
        V4(1, 2, 3, 4),
        V4(5, 6, 7, 8),
        V4(9, 10, 11, 12),
        V4(13, 14, 15, 16)
      ) * Mat2x4(V4(17, 18, 19, 20), V4(21, 22, 23, 24)),
      Mat2x4(V4(538, 612, 686, 760), V4(650, 740, 830, 920))
    );
    assert_eq!(
      Mat4(
        V4(1, 2, 3, 4),
        V4(5, 6, 7, 8),
        V4(9, 10, 11, 12),
        V4(13, 14, 15, 16)
      ) * Mat3x4(V4(17, 18, 19, 20), V4(21, 22, 23, 24), V4(25, 26, 27, 28)),
      Mat3x4(
        V4(538, 612, 686, 760),
        V4(650, 740, 830, 920),
        V4(762, 868, 974, 1080)
      )
    );
    assert_eq!(
      Mat4(
        V4(1, 2, 3, 4),
        V4(5, 6, 7, 8),
        V4(9, 10, 11, 12),
        V4(13, 14, 15, 16)
      ) * Mat4(
        V4(17, 18, 19, 20),
        V4(21, 22, 23, 24),
        V4(25, 26, 27, 28),
        V4(29, 30, 31, 32)
      ),
      Mat4(
        V4(538, 612, 686, 760),
        V4(650, 740, 830, 920),
        V4(762, 868, 974, 1080),
        V4(874, 996, 1118, 1240)
      )
    );
  }

  #[test]
  fn mat4x2_mul_test() {
    assert_eq!(
      Mat4x2(V2(1, 2), V2(3, 4), V2(5, 6), V2(7, 8)) * V4(9, 10, 11, 12),
      V2(178, 220)
    );
    assert_eq!(
      Mat4x2(V2(1, 2), V2(3, 4), V2(5, 6), V2(7, 8))
        * Mat2x4(V4(9, 10, 11, 12), V4(13, 14, 15, 16)),
      Mat2(V2(178, 220), V2(242, 300))
    );
    assert_eq!(
      Mat4x2(V2(1, 2), V2(3, 4), V2(5, 6), V2(7, 8))
        * Mat3x4(V4(9, 10, 11, 12), V4(13, 14, 15, 16), V4(17, 18, 19, 20)),
      Mat3x2(V2(178, 220), V2(242, 300), V2(306, 380))
    );
    assert_eq!(
      Mat4x2(V2(1, 2), V2(3, 4), V2(5, 6), V2(7, 8))
        * Mat4(
          V4(9, 10, 11, 12),
          V4(13, 14, 15, 16),
          V4(17, 18, 19, 20),
          V4(21, 22, 23, 24)
        ),
      Mat4x2(V2(178, 220), V2(242, 300), V2(306, 380), V2(370, 460))
    );
  }

  #[test]
  fn mat4x3_mul_test() {
    assert_eq!(
      Mat4x3(V3(1, 2, 3), V3(4, 5, 6), V3(7, 8, 9), V3(10, 11, 12))
        * V4(13, 14, 15, 16),
      V3(334, 392, 450)
    );
    assert_eq!(
      Mat4x3(V3(1, 2, 3), V3(4, 5, 6), V3(7, 8, 9), V3(10, 11, 12))
        * Mat2x4(V4(13, 14, 15, 16), V4(17, 18, 19, 20)),
      Mat2x3(V3(334, 392, 450), V3(422, 496, 570))
    );
    assert_eq!(
      Mat4x3(V3(1, 2, 3), V3(4, 5, 6), V3(7, 8, 9), V3(10, 11, 12))
        * Mat3x4(V4(13, 14, 15, 16), V4(17, 18, 19, 20), V4(21, 22, 23, 24)),
      Mat3(V3(334, 392, 450), V3(422, 496, 570), V3(510, 600, 690))
    );
    assert_eq!(
      Mat4x3(V3(1, 2, 3), V3(4, 5, 6), V3(7, 8, 9), V3(10, 11, 12))
        * Mat4(
          V4(13, 14, 15, 16),
          V4(17, 18, 19, 20),
          V4(21, 22, 23, 24),
          V4(25, 26, 27, 28)
        ),
      Mat4x3(
        V3(334, 392, 450),
        V3(422, 496, 570),
        V3(510, 600, 690),
        V3(598, 704, 810)
      )
    );
  }

  #[test]
  fn div_test() {
    assert_eq!(V2::new(2.0) / 2.0, V2::new(1.0));
    assert_eq!(V3::new(3.0) / 3.0, V3::new(1.0));
    assert_eq!(V4::new(4.0) / 4.0, V4::new(1.0));
    assert_eq!(Mat2::all(2.0) / 2.0, Mat2::all(1.0));
    assert_eq!(Mat2x3::all(2.0) / 2.0, Mat2x3::all(1.0));
    assert_eq!(Mat2x4::all(2.0) / 2.0, Mat2x4::all(1.0));
    assert_eq!(Mat3::all(3.0) / 3.0, Mat3::all(1.0));
    assert_eq!(Mat3x2::all(3.0) / 3.0, Mat3x2::all(1.0));
    assert_eq!(Mat3x4::all(3.0) / 3.0, Mat3x4::all(1.0));
    assert_eq!(Mat4::all(4.0) / 4.0, Mat4::all(1.0));
    assert_eq!(Mat4x2::all(4.0) / 4.0, Mat4x2::all(1.0));
    assert_eq!(Mat4x3::all(4.0) / 4.0, Mat4x3::all(1.0));
  }

  #[test]
  fn entrywise_product_test() {
    assert_eq!(Mat2::all(2.0).entrywise(Mat2::all(2.0)), Mat2::all(4.0));
    assert_eq!(
      Mat2x3::all(2.0).entrywise(Mat2x3::all(2.0)),
      Mat2x3::all(4.0)
    );
    assert_eq!(
      Mat2x4::all(2.0).entrywise(Mat2x4::all(2.0)),
      Mat2x4::all(4.0)
    );
    assert_eq!(Mat3::all(3.0).entrywise(Mat3::all(3.0)), Mat3::all(9.0));
    assert_eq!(
      Mat3x2::all(3.0).entrywise(Mat3x2::all(3.0)),
      Mat3x2::all(9.0)
    );
    assert_eq!(
      Mat3x4::all(3.0).entrywise(Mat3x4::all(3.0)),
      Mat3x4::all(9.0)
    );
    assert_eq!(Mat4::all(4.0).entrywise(Mat4::all(4.0)), Mat4::all(16.0));
    assert_eq!(
      Mat4x2::all(4.0).entrywise(Mat4x2::all(4.0)),
      Mat4x2::all(16.0)
    );
    assert_eq!(
      Mat4x3::all(4.0).entrywise(Mat4x3::all(4.0)),
      Mat4x3::all(16.0)
    );
  }

  #[test]
  fn dot_test() {
    assert_eq!(V2(1, 2).dot(V2(3, 4)), 11);
    assert_eq!(V3(1, 2, 3).dot(V3(4, 5, 6)), 32);
    assert_eq!(V4(1, 2, 3, 4).dot(V4(5, 6, 7, 8)), 70);
  }

  #[test]
  fn cross_test() {
    assert_eq!(V3(1, 0, 0).cross(V3(0, 1, 0)), V3(0, 0, 1))
  }

  #[test]
  fn norm_test() {
    assert_eq!(V2(2.0, 0.0).norm(), 2.0);
    assert_eq!(V2(0.0, 2.0).norm(), 2.0);
    assert_eq!(V3(3.0, 0.0, 0.0).norm(), 3.0);
    assert_eq!(V3(0.0, 3.0, 0.0).norm(), 3.0);
    assert_eq!(V3(0.0, 0.0, 3.0).norm(), 3.0);
    assert_eq!(V4(4.0, 0.0, 0.0, 0.0).norm(), 4.0);
    assert_eq!(V4(0.0, 4.0, 0.0, 0.0).norm(), 4.0);
    assert_eq!(V4(0.0, 0.0, 4.0, 0.0).norm(), 4.0);
    assert_eq!(V4(0.0, 0.0, 0.0, 4.0).norm(), 4.0);
  }

  #[test]
  fn normalize_test() {
    assert_eq!(V2(2.0, 0.0).normalize(), V2(1.0, 0.0));
    assert_eq!(V2(0.0, 2.0).normalize(), V2(0.0, 1.0));
    assert_eq!(V3(3.0, 0.0, 0.0).normalize(), V3(1.0, 0.0, 0.0));
    assert_eq!(V3(0.0, 3.0, 0.0).normalize(), V3(0.0, 1.0, 0.0));
    assert_eq!(V3(0.0, 0.0, 3.0).normalize(), V3(0.0, 0.0, 1.0));
    assert_eq!(V4(4.0, 0.0, 0.0, 0.0).normalize(), V4(1.0, 0.0, 0.0, 0.0));
    assert_eq!(V4(0.0, 4.0, 0.0, 0.0).normalize(), V4(0.0, 1.0, 0.0, 0.0));
    assert_eq!(V4(0.0, 0.0, 4.0, 0.0).normalize(), V4(0.0, 0.0, 1.0, 0.0));
    assert_eq!(V4(0.0, 0.0, 0.0, 4.0).normalize(), V4(0.0, 0.0, 0.0, 1.0));
  }
}
