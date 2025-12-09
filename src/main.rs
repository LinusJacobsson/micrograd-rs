use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;

#[derive(Debug, Clone, Copy)]
struct Value {
    data: i32,
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Value {
            data: self.data + other.data,
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, other: Self) {
        self.data += other.data;
    }

}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Value {
            data: self.data - other.data,
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, other: Self) {
        self.data -= other.data;
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Value {
            data: self.data * other.data,
        }
    }
}

impl MulAssign for Value {
    fn mul_assign(&mut self, other: Self) {
        self.data *= other.data;
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Value {
            data: self.data / other.data,
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, other: Self) {
        self.data /= other.data;
    }
}

fn main() {
    println!("Hello, world!");
    let value_1 = Value { data: 10 };
    let value_2 = Value { data: 5 };
    let sum = value_1 + value_2;
    let difference = value_1 - value_2;
    println!("Sum: {:?}", sum);
    println!("Difference: {:?}", difference);
}


