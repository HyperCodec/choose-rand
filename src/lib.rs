//! A small crate for choosing random items from a set of weighed items.
//! 
//! ### Installation/Setup
//! To import the crate into your project, simply run `cargo add choose-rand` or open your Cargo.toml and add `choose-rand = "<latest release>"`
//! 
//! In any file that you want to use the crate, add `use choose_rand::prelude::*;` and `use eq_float::F64;` (the eq_float part is for when you need to `impl Probable`)
//! 
//! ### Examples
//! 1. Enum
//! ```rust
//! use choose_rand::prelude::*;
//! use eq_float::F64;
//! 
//! #[derive(Hash, Eq, PartialEq, Clone, Debug)]
//! enum Test {
//!     foo,
//!     bar,
//!     buz
//! }
//! 
//! impl Probable for Test {
//!     fn probability(&self) -> F64 {
//!         match self {
//!             Test::foo => F64(0.1),
//!             Test::bar => F64(0.2),
//!             Test::buz => F64(0.7)
//!         }
//!     }
//! }
//! 
//! fn main() {
//!     // all probabilities must add up to 1 or else it will (sometimes) fail.
//!     // it also works with BTreeSet
//!     let things = HashSet::from([
//!         Test::foo,
//!         Test::bar, 
//!         Test::buz
//!     ]);
//!     
//!     let chosen = choose_rand(&things).unwrap();
//! 
//!     println!("The chosen one is: {:#?}", chosen);
//! }
//! ```
//! 
//! 2. Struct
//! ```rust
//! use choose_rand::prelude::*;
//! use eq_float::F64;
//! 
//! #[derive(Hash, Eq, PartialEq, Clone, Debug)]
//! struct Test(F64);
//! 
//! impl Probable for Test {
//!     fn probability(&self) -> F64 {
//!         self.0
//!     }
//! }
//! 
//! fn main() {
//!     // all probabilities must add up to 1 or else it will (sometimes) fail.
//!     // it also works with BTreeSet
//!     let things = HashSet::from([
//!         Test(F64(0.1)),
//!         Test(F64(0.2)),
//!         Test(F64(0.7))
//!     ]);
//! 
//!     let chosen = choose_rand(&things).unwrap();
//! 
//!     println!("The chosen one is: {:#?}", chosen);
//! }
//! ```

#[warn(missing_docs)]

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
