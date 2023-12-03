use crate::math::group::{AbelianGroup, Addition, Group, Multiplication};

/// A Field is an abelian group over addition and multiplication and distributive
/// for multiplication with respect to addition.
trait Field<E>: AbelianGroup<Addition, E> + AbelianGroup<Multiplication, E> {
    /// Supports addition
    fn add(&self, e1: E, e2: E) -> E;
    /// Supports subtraction which is the same as addition with the inverse
    fn subtract(&self, e1: E, e2: E) -> E;
    /// Supports multiplication
    fn multiply(&self, e1: E, e2: E) -> E;
    /// Supports division where the divisor is not equal to zero
    fn divide(&self, e1: E, e2: E) -> E;
}

/// The natural numbers mod p where p is a prime form a field.
struct PrimeField(u32);

impl PrimeField {

    #[allow(dead_code)]
    fn new(modulus: u32) -> Self {
        if !primes::is_prime(modulus.into()) {
            panic!("modulus is not a prime")
        }

        PrimeField(modulus)
    }
}


impl AbelianGroup<Addition, u32> for PrimeField {}

impl Group<Addition, u32> for PrimeField {
    fn apply(&self, e1: u32, e2: u32) -> u32 {
        (e1 + e2) % self.0
    }

    fn identity(&self) -> u32 {
        0
    }

    fn inverse(&self, e: u32) -> u32 {
        self.0 - e % self.0
    }
}

impl AbelianGroup<Multiplication, u32> for PrimeField {}

impl Group<Multiplication, u32> for PrimeField {
    fn apply(&self, e1: u32, e2: u32) -> u32 {
        (e1 * e2) % self.0
    }

    fn identity(&self) -> u32 {
        1
    }

    fn inverse(&self, e: u32) -> u32 {
        if e % self.0 == 0 {
            panic!("Cannot calculate inverse for zero")
        }

        // this trial and error method to calculate the inverse is not efficient
        // in another post we will use the Extended Euclidean Algorithm to solve this
        for i in 0..self.0 {
            if (e * i) % self.0 == 1 {
                return i;
            }
        }

        panic!("No inverse found");
    }
}

impl Field<u32> for PrimeField {
    fn add(&self, e1: u32, e2: u32) -> u32 {
        <dyn AbelianGroup<Addition, u32>>::apply(self, e1, e2)
    }

    fn subtract(&self, e1: u32, e2: u32) -> u32 {
        let inverse = <dyn AbelianGroup<Addition, u32>>::inverse(self,e2);
        <dyn AbelianGroup<Addition, u32>>::apply(self, e1, inverse)
    }

    fn multiply(&self, e1: u32, e2: u32) -> u32 {
        <dyn AbelianGroup<Multiplication, u32>>::apply(self, e1, e2)
    }

    fn divide(&self, e1: u32, e2: u32) -> u32 {
        let inverse = <dyn AbelianGroup<Multiplication, u32>>::inverse(self,e2);
        <dyn AbelianGroup<Multiplication, u32>>::apply(self, e1, inverse)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_prime_field() {
        let field = PrimeField::new(11);

        let result = field.add(3, 10);
        assert_eq!(2, result);

        let result = field.subtract(1, 3);
        assert_eq!(9, result);

        let result = field.multiply(2, 10);
        assert_eq!(9, result);

        let result = field.divide(10, 2);
        assert_eq!(5, result);

        // Every element in the field apart from zero has an inverse
        for i in 1..11 {
            let _result = field.divide(10, i);
        }
    }

}





