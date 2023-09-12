use std::cell::RefCell;

pub fn refcellify<T, C: Iterator<Item = T>>(c: C) -> impl Iterator<Item = RefCell<T>> {
    c.map(|i| RefCell::new(i))
}