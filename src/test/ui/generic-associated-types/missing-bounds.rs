// run-rustfix

use std::ops::Add;

struct A<B>(B);

impl<B> Add for A<B> where B: Add {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        A(self.0 + rhs.0) //~ ERROR mismatched types
    }
}

struct C<B>(B);

impl<B: Add> Add for C<B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0) //~ ERROR mismatched types
    }
}

struct D<B>(B);

impl<B> Add for D<B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0) //~ ERROR cannot add `B` to `B`
    }
}

struct E<B>(B);

impl<B: Add> Add for E<B> where B: Add<Output = B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

fn main() {}
