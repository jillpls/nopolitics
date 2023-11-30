use derive_more::{Add, Display, Sub};
use num_traits::{One, Signed, Zero};
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};

pub trait NegOne: One + Zero + Sub<Self, Output = Self> {
    fn neg_one() -> Self {
        Self::zero() - Self::one()
    }
}

impl<T> NegOne for T where T: One + Zero + Sub<Self, Output = T> {}

pub trait MatrixValue:
    Signed
    + Add<Self>
    + Sub<Self>
    + Mul<Self>
    + Div<Self>
    + PartialOrd
    + PartialEq
    + Display
    + Debug
    + Clone
{
}

impl<T> MatrixValue for T where
    T: Signed
        + Add<T>
        + Sub<T>
        + Mul<T>
        + Div<T>
        + PartialOrd
        + PartialEq
        + Display
        + Debug
        + Clone
{
}

#[derive(PartialOrd, PartialEq, Display, Add, Sub, Clone, Debug)]
#[display(fmt = "({},{})", x, y)]
pub struct Point2<T: MatrixValue> {
    pub x: T,
    pub y: T,
}

impl<T: MatrixValue + Copy> Copy for Point2<T> {}

impl<T: MatrixValue> Point2<T> {
    pub fn to_array(&self) -> [T; 2] {
        [self.x.clone(), self.y.clone()]
    }
}

impl<T: MatrixValue + Clone> Mul<T> for Point2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

impl<T: MatrixValue + Clone> Div<T> for Point2<T> {
    type Output = Point2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}

impl<T: MatrixValue> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(PartialOrd, PartialEq, Display)]
#[display(fmt = "({:?})", inner)]
pub struct Matrix2<T: MatrixValue> {
    inner: [[T; 2]; 2],
}

impl<T: MatrixValue + Clone> Add<Matrix2<T>> for Matrix2<T> {
    type Output = Self;

    fn add(self, rhs: Matrix2<T>) -> Self::Output {
        Matrix2 {
            inner: [
                [
                    self.inner[0][0].clone() + rhs.inner[0][0].clone(),
                    self.inner[0][1].clone() + rhs.inner[0][1].clone(),
                ],
                [
                    self.inner[1][0].clone() + rhs.inner[1][0].clone(),
                    self.inner[1][1].clone() + rhs.inner[1][1].clone(),
                ],
            ],
        }
    }
}

impl<T: MatrixValue + Clone> Mul<Point2<T>> for Matrix2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: Point2<T>) -> Self::Output {
        &self * rhs
    }
}

impl<T: MatrixValue + Clone> Mul<Point2<T>> for &Matrix2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: Point2<T>) -> Self::Output {
        Point2 {
            x: self.inner[0][0].clone() * rhs.x.clone() + self.inner[0][1].clone() * rhs.y.clone(),
            y: self.inner[1][0].clone() * rhs.x + self.inner[1][1].clone() * rhs.y,
        }
    }
}

impl<T: MatrixValue + One + Zero> Mul<Self> for Matrix2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: MatrixValue + One + Zero> One for Matrix2<T> {
    fn one() -> Self {
        Self {
            inner: [[T::one(), T::zero()], [T::one(), T::zero()]],
        }
    }

    fn set_one(&mut self) {
        self.inner[0][0] = T::one();
        self.inner[0][1] = T::zero();
        self.inner[1][0] = T::zero();
        self.inner[1][1] = T::one();
    }

    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.inner[0][0] == T::one()
            && self.inner[0][1] == T::zero()
            && self.inner[1][0] == T::zero()
            && self.inner[1][1] == T::one()
    }
}

impl<T: MatrixValue + One + Zero + NegOne> Matrix2<T> {
    pub fn rot90() -> Self {
        Self {
            inner: [[T::zero(), T::neg_one()], [T::one(), T::zero()]],
        }
    }

    pub fn rot180() -> Self {
        Self {
            inner: [[T::neg_one(), T::zero()], [T::zero(), T::neg_one()]],
        }
    }

    pub fn rot270() -> Self {
        Self {
            inner: [[T::zero(), T::one()], [T::neg_one(), T::zero()]],
        }
    }
}
