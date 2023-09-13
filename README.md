# choose-rand
A small crate for choosing random items from a set of weighted items.

### Examples
```rust
use choose_rand::prelude::*;
 
 #[derive(Debug, Clone)]
 struct Foo {
     prob: f32,
 }
 
 impl Probable for Foo {
     fn probability(&self) -> f32 {
         self.prob
     }
 }
 
 fn main() -> Result<()> {
     let v: Vec<_> = choose_rand::helper::refcellify(
         vec![Foo { prob: 0.25 }, Foo { prob: 0.5 }, Foo { prob: 0.1 }, Foo { prob: 0.05 }]
     ).collect();
 
     let mut rng = rand::thread_rng();    
     dbg!(v.choose_rand(&mut rng)?);
 
     Ok(())
 }
```

with `features = ["eq_float"]` for sets:
```rust
use choose_rand::prelude::*;
use eq_float::F32;
use std::collections::HashSet;

#[derive(Clone, Debug, Hash)]
struct Foo {
    prob: F32,
}

impl Foo {
    fn new(prob: f32) -> Self {
        Self { prob: F32(prob) }
    }
}

impl Probable for Foo {
    fn probablity(&self) -> F32 {
        self.prob
    }
}

fn main() -> Result<()> {
     let v: HashSet<_> = choose_rand::helper::refcellify(
         vec![Foo::new(0.25), Foo::new(0.5), Foo::new(0.1), Foo::new(0.05)]
     ).collect();
 
     let mut rng = rand::thread_rng();    
     dbg!(v.choose_rand(&mut rng)?);
 
     Ok(())
 }
```

# License
This library is licensed under the `MIT` license.
