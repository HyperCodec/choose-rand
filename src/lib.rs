
#[warn(missing_docs)]

/// Contains all of the important things from this crate.
/// When using the crate, you want to do `use choose_rand::prelude::*;`
pub mod prelude;
pub mod rand;
pub mod error;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::prelude::*;
    //use std::collections::{HashSet, BTreeSet};

    #[derive(Debug, Clone)]
    struct Foo(f32, Option<String>);

    impl Probable for Foo {
        fn probability(&self) -> f32 {
            self.0
        }
    }
    
    fn refcellify<T, C: Iterator<Item = T>>(c: C) -> impl Iterator<Item = RefCell<T>> {
        c.map(|i| RefCell::new(i))
    }

    #[test]
    fn vec() -> Result<()> {
        let v: Vec<_> = refcellify(
            vec![Foo(0.1, None), Foo(0.25, None), Foo(0.5, None), Foo(0.15, None)]
            .into_iter()
        ).collect();

        let mut chosen = Vec::with_capacity(100);

        let mut rng = rand::thread_rng();

        for _ in 0 .. 100 {
            chosen.push(v.choose_rand(&mut rng)?);
        }

        dbg!(chosen);

        Ok(())
    }

    #[test]
    fn vec_mut() -> Result<()> {
        let mut v: Vec<_> = refcellify(
            vec![Foo(0.1, Some("hi".into())), Foo(0.25, Some("hello".into())), Foo(0.5, Some("hola".into())), Foo(0.15, Some("bonjour".into()))]
            .into_iter()
        ).collect();

        let mut rng = rand::thread_rng();

        let mut chosen = Vec::with_capacity(100);

        for _ in 0 .. 100 {
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