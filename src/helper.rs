use std::cell::RefCell;

pub fn refcellify<T, C: IntoIterator<Item = T>>(c: C) -> impl Iterator<Item = RefCell<T>> {
    c.into_iter().map(|i| RefCell::new(i))
}
