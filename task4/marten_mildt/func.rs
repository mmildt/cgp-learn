use std::ops::{Mul, Add};

fn main() {
    println!("{:?}", tolle_funktion(2, 3));
}

fn tolle_funktion<T: Mul + Add + Copy>(v1: T, v2: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (v1 + v2, v1 * v2)
}
