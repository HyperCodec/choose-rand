# choose-rand
A small crate for choosing random items from a set of weighed items.

### Installation/Setup
To import the crate into your project, simply run `cargo add choose-rand` or open your Cargo.toml and add `choose-rand = "<latest release>"`

In any file that you want to use the crate, add `use choose_rand::prelude::*;`

### Examples
1. Enum
```rust
use choose_rand::prelude::*;

enum Test {
    foo,
    bar,
    buz
}

impl Probable for Test {
    fn probability(&self) -> f64 {
        match self {
            Test::foo => 0.1,
            Test::bar => 0.2,
            Test::buz => 0.7
        }
    }
}

fn main() {
    // all probabilities must add up to 1 or else it will (sometimes) fail.
    let things = HashSet::from([Test::foo, Test::bar, Test::buz]);
    
    let chosen = choose_rand(&things);

    println!("The chosen one is: {}", chosen);
}
```

2. Struct
```rust
todo!()
```