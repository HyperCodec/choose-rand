//! A small crate for weighed choosing from a set.

/// Contains all of the important things from this crate.
/// When using the crate, you want to do `use choose_rand::prelude::*;`
pub mod prelude;

use std::fmt;

/// Simple Error struct that has a String value for whatever reason it errored.
#[derive(Debug, Clone)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::prelude::*;

    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    enum Thing {
        A,
        B,
        C,
        D,
    }

    impl Probable for Thing {
        fn probability(&self) -> f64 {
            match self {
                Thing::A => 0.15,
                Thing::B => 0.35,
                Thing::C => 0.45,
                Thing::D => 0.05,
            }
        }
    }

    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    struct Thing2(F64);

    impl Probable for Thing2 {
        fn probability(&self) -> F64 {
            self.0
        }
    }

    #[test]
    fn test_enum() {
        let selection = HashSet::from([
            Thing::A, 
            Thing::B, 
            Thing::C, 
            Thing::D
        ]);

        let chosen = choose_rand(&selection).unwrap();

        println!("The chosen one is: {:?}", chosen);
    }

    #[test]
    fn test_struct() {
        let selection = HashSet::from([
            Thing2(F64(0.15)),
            Thing2(F64(0.35)),
            Thing2(F64(0.45)),
            Thing2(F64(0.05))
        ]);

        let chosen = choose_rand(&selection).unwrap();

        println!("The chosen one is: {:?}", chosen);
    }
}
