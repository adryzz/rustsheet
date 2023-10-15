use num_traits::{Bounded, PrimInt, Unsigned};

/// Greatest Common Denominator, using the euclidean algorithm
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Bounded + Unsigned + PrimInt,
{
    let mut a = a;
    let mut b = b;
    while b != num_traits::zero() {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
