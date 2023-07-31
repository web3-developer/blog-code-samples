
/// Marks the Monoid and Group traits with a type
pub trait Operation {}
pub struct Addition;
pub struct Multiplication;
impl Operation for Addition {}
impl Operation for Multiplication {}

/// A monoid is a set and a binary operation (denoted by '.') which satisfies the following properties:
/// Associativity - a . (b . c) = (a . b) . c
/// Identity - a . I = a
pub trait Monoid<T: Operation, E> {
    /// The binary operation which takes elements e1 and e2 of type E and returns the result
    fn apply(&self, e1: E, e2: E) -> E;

    /// Returns the identity element
    fn identity(&self) -> E;
}

/// A group is a set and a binary operation (denoted by '.') which satisfies the following properties:
/// Closure - a . b = c where a, b, and c are in the set
/// Associativity - a . (b . c) = (a . b) . c
/// Identity - a . I = a
/// Inverse - a . a^-1 = I
pub trait Group<T: Operation, E> {
    /// The binary operation which takes elements e1 and e2 of type E and returns the result
    fn apply(&self, e1: E, e2: E) -> E;

    /// Returns the identity element
    fn identity(&self) -> E;

    /// Returns the inverse of element e
    fn inverse(&self, e: E) -> E;
}

/// A type which supports commutativity in addition to the standard group properties:
/// Commutativity - a . b = b . a
pub trait AbelianGroup<T: Operation, E>: Group<T, E> {}



/// Integers under addition form a group where plus (+) is the group operation, zero is the identity
/// and negation (-) can be used to get the inverse of an element
struct AdditiveIntegers();

impl Group<Addition, i32> for AdditiveIntegers {
    fn apply(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn identity(&self) -> i32 {
        0
    }

    fn inverse(&self, a: i32) -> i32 {
        -a
    }
}

// Integers under addition supports commutativity
impl AbelianGroup<Addition, i32> for AdditiveIntegers {}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_additive_integers() {
        let group = AdditiveIntegers();

        let a: i32 = 10;
        let b: i32 = 20;
        let c: i32 = 30;

        // Closure - a . b = c where a, b, and c are in the set
        let result: i32 = group.apply(a, b);
        assert_eq!(c, result); // result is an integer so in the set

        // Associativity - a . (b . c) = (a . b) . c
        assert_eq!(group.apply(a, group.apply(b, c)),
                   group.apply(group.apply(a , b), c));

        // Identity - a . I = a
        assert_eq!(group.apply(a, group.identity()), a);

        // Inverse - a . a^-1 = I
        assert_eq!(group.apply(a, group.inverse(a)), group.identity());

        // Commutativity - a . b = b . a
        assert_eq!(group.apply(a, b), group.apply(b, a));
    }

}





