use crate::prelude::*;
use rand::Rng;
use std::cell::{Ref, RefCell, RefMut};
use weighted_rand::builder::*;

/// Blanket trait for all indexable collections that contain RefCells of Probable items. Adds `choose_rand` and `choose_rand_mut`.
#[cfg(feature = "normal_float")]
pub trait ChooseRand:
    IntoIterator<Item = RefCell<<Self as ChooseRand>::Item>>
    + std::ops::Index<usize, Output = RefCell<<Self as ChooseRand>::Item>>
    + Sized
    + Clone
{
    /// The type of item in the Set
    type Item: Probable;

    /// Choose a random item from the collection based on probability. Will return error if collection probabilities do not add up to 1.
    fn choose_rand(&self, rng: &mut impl Rng) -> Result<Ref<<Self as ChooseRand>::Item>> {
        let weights: Vec<_> = self
            .clone()
            .into_iter() // TODO remove clones to make more efficient
            .map(|p| p.borrow().probability())
            .collect();

        if weights.iter().sum::<f32>() != 1. {
            return Err(Error::ProbabilitySum);
        }

        let wt = WalkerTableBuilder::new(&weights).build();

        let i = wt.next_rng(rng);

        Ok(self[i].borrow())
    }

    /// Same as `choose_rand`, but it returns a RefMut so that the original item can be mutated.
    fn choose_rand_mut(
        &mut self,
        rng: &mut impl Rng,
    ) -> Result<RefMut<<Self as ChooseRand>::Item>> {
        let weights: Vec<_> = self
            .clone()
            .into_iter()
            .map(|p| p.borrow().probability())
            .collect();

        if weights.iter().sum::<f32>() != 1. {
            return Err(Error::ProbabilitySum);
        }

        let wt = WalkerTableBuilder::new(&weights).build();

        let i = wt.next_rng(rng);

        Ok(self[i].borrow_mut())
    }
}

#[cfg(feature = "eq_float")]
pub trait ChooseRand:
    IntoIterator<Item = RefCell<<Self as ChooseRand>::Item>>
    + std::ops::Index<usize, Output = RefCell<<Self as ChooseRand>::Item>>
    + Sized
    + Clone
{
    /// The type of item in the Set
    type Item: Probable;

    /// Choose a random item from the collection based on probability. Will return error if collection probabilities do not add up to 1.
    fn choose_rand(&self, rng: &mut impl Rng) -> Result<Ref<<Self as ChooseRand>::Item>> {
        let weights: Vec<_> = self
            .clone()
            .into_iter() // TODO remove clones to make more efficient
            .map(|p| p.borrow().probability())
            .collect();

        if weights.iter().sum::<eq_float::F32>() != eq_float::F32(1.) {
            return Err(Error::ProbabilitySum);
        }

        let wt = WalkerTableBuilder::new(&weights).build();

        let i = wt.next_rng(rng);

        Ok(self[i].borrow())
    }

    /// Same as `choose_rand`, but it returns a RefMut so that the original item can be mutated.
    fn choose_rand_mut(
        &mut self,
        rng: &mut impl Rng,
    ) -> Result<RefMut<<Self as ChooseRand>::Item>> {
        let weights: Vec<_> = self
            .clone()
            .into_iter()
            .map(|p| p.borrow().probability())
            .collect();

        if weights.iter().sum::<f32>() != 1. {
            return Err(Error::ProbabilitySum);
        }

        let wt = WalkerTableBuilder::new(&weights).build();

        let i = wt.next_rng(rng);

        Ok(self[i].borrow_mut())
    }
}

impl<T, V> ChooseRand for T
where
    T: IntoIterator<Item = RefCell<V>>
        + std::ops::Index<usize, Output = RefCell<V>>
        + Sized
        + Clone,
    V: Probable,
{
    type Item = V;
}

/// Required for `chooe_rand` to work.
/// Use on any items to be chosen by this crate.
#[cfg(feature = "normal_float")]
pub trait Probable {
    /// The probability that this item will be picked.
    fn probability(&self) -> f32;
}

#[cfg(feature = "eq_float")]
pub trait Probable {
    fn probability(&self) -> eq_float::F32;
}
