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

    use eq_float::F64;

    use super::prelude::*;

    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    enum Thing {
        A,
        B,
        C,
        D,
    }

    impl Probable for Thing {
        fn probability(&self) -> F64 {
            match self {
                Thing::A => F64(0.15),
                Thing::B => F64(0.35),
                Thing::C => F64(0.45),
                Thing::D => F64(0.05),
            }
        }
    }

    #[test]
    fn test() {
        let mut selection = HashSet::new();

        {
            let original = [Thing::A, Thing::B, Thing::C, Thing::D];

            for o in original {
                selection.insert(o);
            }
        }

        let chosen = choose_rand(&selection).unwrap();

        println!("The chosen one is: {:?}", chosen);
    }
}
