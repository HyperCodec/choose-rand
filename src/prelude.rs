use std::collections::{HashSet, BTreeSet};
use eq_float::F64;

pub use crate::error::*;

pub type Result<T> = std::result::Result<T, Error>;

pub use crate::rand::*;