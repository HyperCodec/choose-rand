use weighted_rand::builder::*;
use rand::Rng;
use std::cell::RefCell;

/// Marker trait used by `choose_rand`.
/// Allows function to accept either type of set.
pub trait ChooseRand: IntoIterator<Item = RefCell<<Self as ChooseRand>::Item>> + std::ops::Index {
    /// The type of item in the Set
    type Item;

    fn choose_rand(&self, rng: &mut impl Rng) -> Result<Ref<<Self as ChooseRand>::Item>> {
        let weights: Vec<f64> = self.into_iter()
            .map(|p| p.probability())
            .collect();

        if weights.iter().sum() != 1. {
            return Err(Error("Individual probabilities must add up to 1"));
        }

        let wt = WalkerTableBuilder::new(&weights)
            .build();
        
        let i = wt.next_rng(rng);

        Ok(self[i].borrow())
    }

    fn choose_rand_mut(&mut self, rng: &mut impl Rng) -> Result<RefMut<<Self as ChooseRand>::Item>> {
        let weights: Vec<f64> = self.into_iter()
            .map(|p| p.probability())
            .collect();

        if weights.iter().sum() != 1. {
            return Err(Error("Individual probabilities must add up to 1"));
        }

        let wt = WalkerTableBuilder::new(&weights)
            .build();
        
        let i = wt.next_rng(rng);

        Ok(self[i].borrow_mut())
    }
}

impl<T> ChooseRand for T
where
    T: IntoIterator<Item = <Self as ChooseRand>::Item> + std::ops::Index
{

}

/// Required for `chooe_rand` to work.
/// Use on any items to be chosen.
pub trait Probable {
    /// The probability that this item will be picked.
    fn probability(&self) -> f64;
}

