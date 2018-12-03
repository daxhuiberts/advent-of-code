use std::hash::Hash;
use std::collections::HashMap;

pub trait IterExt : Iterator {
    fn group_count(self) -> HashMap<Self::Item, usize>
        where Self: Sized,
              <Self as Iterator>::Item: Eq + Hash
    {
        self.fold_ref(HashMap::new(), |map, item| {
            *map.entry(item).or_default() += 1
        })
    }

    fn fold_ref<B, F>(self, init: B, mut f: F) -> B
        where Self: Sized,
              F: FnMut(&mut B, Self::Item)
    {
        self.fold(init, |mut acc, item| {
            f(&mut acc, item);
            acc
        })
    }
}

impl<T> IterExt for T where T: Iterator { }
