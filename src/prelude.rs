use std::collections::{HashSet, BTreeSet};
use eq_float::F64;

use crate::Error;

/// Marker trait used by `choose_rand`.
/// Allows function to accept either type of set.
pub trait Set: IntoIterator<Item = <Self as Set>::Item> {
    /// The type of item in the Set
    type Item;
}

impl<'a, P: Probable> Set for &'a HashSet<P> {
    type Item = &'a P;
}
impl<'a, P: Probable> Set for &'a BTreeSet<P> {
    type Item = &'a P;
}

/// Required for `chooe_rand` to work.
/// Use on any items to be chosen.
pub trait Probable: Clone {
    /// The probability that this item will be picked.
    fn probability(&self) -> F64;
}

/// Pick a random item from the set,
/// weighed by `item.probability()`.
/// The set can be either a HashSet or a BTreeSet
pub trait RandChoosable<P: Probable>: Set<Item = P> {
    fn choose_rand(&self) -> Result<P, Error> {
        let r = F64(fastrand::f64());

        let mut last = F64(0.);
        for choice in self {
            let p = choice.probability();

            let newlast = F64(last.0 + p.0);

            if (last..newlast).contains(&r) {
                return Ok(choice.clone());
            }

            last = newlast;
        }

        Err(Error("Probabilities must add up to 1".to_string()))
    }   
}

impl<T, P> RandChoosable<P> for T
where
    T: Set<Item = P>,
    P: Probable
{}
