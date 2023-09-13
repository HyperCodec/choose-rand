//! # choose-rand
//! A small crate for choosing random items from a list of weighted items.
//!
//! ### Example
//! ```rust
//! use choose_rand::prelude::*;
//!
//! #[derive(Debug, Clone)]
//! struct Foo {
//!     prob: f32,
//! }
//!
//! impl Probable for Foo {
//!     fn probability(&self) -> f32 {
//!         self.prob
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let v: Vec<_> = choose_rand::helper::refcellify(
//!         vec![Foo { prob: 0.25 }, Foo { prob: 0.5 }, Foo { prob: 0.1 }, Foo { prob: 0.05 }]
//!     ).collect();
//!
//!     let mut rng = rand::thread_rng();    
//!     dbg!(v.choose_rand(&mut rng));
//!
//!     Ok(())
//! }
//! ```

#[warn(missing_docs)]

/// Contains all of the important things from this crate.
/// When using the crate, you want to do `use choose_rand::prelude::*;`
pub mod prelude;

/// Adds all the main parts of the crate, including the traits required for the crate to work
pub mod rand;

/// Contains the error enum for this crate.
pub mod error;

/// Contains some simple helper functions to make life easier when using this crate.
pub mod helper;

#[cfg(all(feature = "normal_float", feature = "eq_float"))]
compile_error!("`normal_float` and `eq_float` features can not be enabled at the same time");

#[cfg(test)]
mod tests {

    use super::helper::*;
    use super::prelude::*;

    #[derive(Debug, Clone)]
    struct Foo(f32, Option<String>);

    impl Probable for Foo {
        fn probability(&self) -> f32 {
            self.0
        }
    }

    #[test]
    fn vec() -> Result<()> {
        let v: Vec<_> = refcellify(vec![
            Foo(0.1, None),
            Foo(0.25, None),
            Foo(0.5, None),
            Foo(0.15, None),
        ])
        .collect();

        let mut chosen = Vec::with_capacity(100);

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            chosen.push(v.choose_rand(&mut rng)?);
        }

        dbg!(chosen);

        Ok(())
    }

    #[test]
    fn vec_mut() -> Result<()> {
        let mut v: Vec<_> = refcellify(
            vec![
                Foo(0.1, Some("hi".into())),
                Foo(0.25, Some("hello".into())),
                Foo(0.5, Some("hola".into())),
                Foo(0.15, Some("bonjour".into())),
            ]
            .into_iter(),
        )
        .collect();

        let mut rng = rand::thread_rng();

        let mut chosen = Vec::with_capacity(100);

        for _ in 0..100 {
            let mut c = v.choose_rand_mut(&mut rng)?;

            if let Some(s) = &mut c.1 {
                s.push('!');
            }

            chosen.push(c.clone()); // cloned so we can see each version of it. without clone, all copies of it should change in `chosen`
        }

        dbg!(chosen, v);

        Ok(())
    }
}
