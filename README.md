# choose-rand
A small crate for choosing random items from a set of weighed items.

### Installation/Setup
To import the crate into your project, simply run `cargo add choose-rand` or open your Cargo.toml and add `choose-rand = "<latest release>"`

In any file that you want to use the crate, add `use choose_rand::prelude::*;`.

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
     dbg!(v.choose_rand(&mut rng));
 
     Ok(())
 }
```

# License
This library is licensed under the `MIT` license.