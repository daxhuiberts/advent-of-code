use std::hash::Hash;
use std::collections::HashMap;

pub trait IterExt : Iterator {
    fn group_count(self) -> HashMap<Self::Item, usize>
        where Self: Sized,
              <Self as Iterator>::Item: Eq + Hash
    {
        self.fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;
            map
        })
    }
}

impl<T> IterExt for T where T: Iterator { }
