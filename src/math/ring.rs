use crate::math::group::{AbelianGroup, Addition, Group, Monoid, Multiplication};

/// A Ring is an abelian group over addition, a monoid under multiplication and distributive
/// for multiplication with respect to addition.
///
/// It has the following properties over multiplication:
/// Closure - a x b = c where a, b, and c are in the set
/// Associativity - a x (b x c) = (a x b) x c
/// Identity - a x I = a
///
/// Multiplication is distributive with respect to addition:
/// Left distributive - a x (b + c) = (a x b) + (a x c)
/// Right distributive - (b + c) x a = (b x a) + (c x a)
trait Ring<E>: AbelianGroup<Addition, E> + Monoid<Multiplication, E> {
    /// Supports addition
    fn add(&self, e1: E, e2: E) -> E;
    /// Supports subtraction which is the same as adding the inverse
    fn subtract(&self, e1: E, e2: E) -> E;
    /// Supports multiplication
    fn multiply(&self, e1: E, e2: E) -> E;
}


/// The natural numbers mod n form a ring.
struct NaturalNumbersModN(u32);

impl AbelianGroup<Addition, u32> for NaturalNumbersModN {}

impl Group<Addition, u32> for NaturalNumbersModN {
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

impl Monoid<Multiplication, u32> for NaturalNumbersModN {
    fn apply(&self, e1: u32, e2: u32) -> u32 {
        (e1 * e2) % self.0
    }

    fn identity(&self) -> u32 {
        1
    }
}

impl Ring<u32> for NaturalNumbersModN {
    fn add(&self, e1: u32, e2: u32) -> u32 {
        <dyn AbelianGroup<Addition, u32>>::apply(self, e1, e2)
    }

    fn subtract(&self, e1: u32, e2: u32) -> u32 {
        <dyn AbelianGroup<Addition, u32>>::apply(self, e1, self.inverse(e2))
    }

    fn multiply(&self, e1: u32, e2: u32) -> u32 {
        Monoid::apply(self, e1, e2)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run() {

    }

    #[test]
    fn run_natural_numbers_modn() {
        let ring = NaturalNumbersModN(4);

        let result = ring.add(3, 2);
        assert_eq!(1, result);

        let result = ring.subtract(3, 3);
        assert_eq!(0, result);

        let result = ring.multiply(2, 3);
        assert_eq!(2, result);
    }

}





