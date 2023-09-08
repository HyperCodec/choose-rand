use weighted_rand::builder::*;
use rand::Rng;

/// Marker trait used by `choose_rand`.
/// Allows function to accept either type of set.
pub trait ProbabilityGroup: IntoIterator<Item = <Self as ProbabilityGroup>::Item> + std::ops::Index {
    /// The type of item in the Set
    type Item;

    fn choose_rand<T>(&self, rng: &mut impl Rng) -> Result<T>
    where
        T: Probable + Clone, // TODO remove clone functionality and do it with refcell or something
    {
        let weights: Vec<f64> = self.into_iter()
            .map(|p| p.probability())
            .collect();

        if weights.iter().sum() != 1. {
            return Err(Error("Individual probabilities must add up to 1"));
        }

        let wt = WalkerTableBuilder::new(&weights)
            .build();
        
        let i = wt.next_rng(rng);

        self[i].clone()
    }
}

impl<T> ProbabilityGroup for T
where
    T: IntoIterator<Item = <Self as ProbabilityGroup>::Item> + std::ops::Index
{
    
}

/// Required for `chooe_rand` to work.
/// Use on any items to be chosen.
pub trait Probable {
    /// The probability that this item will be picked.
    fn probability(&self) -> f64;
}

