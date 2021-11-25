use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Scan;

pub trait IterExt: Iterator {
    fn group_count(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        self.fold_ref(HashMap::new(), |map, item| {
            *map.entry(item).or_default() += 1
        })
    }

    fn scan_ref<St, F>(
        self,
        init: St,
        f: F,
    ) -> Scan<Self, (F, St), fn(&mut (F, St), Self::Item) -> Option<St>>
    where
        Self: Sized,
        St: Copy,
        F: FnMut(&mut St, Self::Item),
    {
        fn scan_ref_closure<St, F, I>(f_st: &mut (F, St), item: I) -> Option<St>
        where
            St: Copy,
            F: FnMut(&mut St, I),
        {
            (f_st.0)(&mut f_st.1, item);
            Some(f_st.1)
        }

        self.scan((f, init), scan_ref_closure)
    }

    fn fold_ref<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(&mut B, Self::Item),
    {
        self.fold(init, |mut acc, item| {
            f(&mut acc, item);
            acc
        })
    }

    fn fold_to_map<K, V, F>(self, mut f: F) -> HashMap<K, V>
    where
        Self: Sized,
        K: Eq + Hash,
        F: FnMut(&mut HashMap<K, V>, Self::Item),
    {
        self.fold_ref(HashMap::new(), |mut acc, item| f(&mut acc, item))
    }

    fn grouped<K, F>(self, mut f: F) -> HashMap<K, Vec<Self::Item>>
    where
        Self: Sized,
        K: Eq + Hash,
        F: FnMut(&Self::Item) -> K,
    {
        self.fold_ref(HashMap::new(), |map, item| {
            let key = f(&item);
            let entry = map.entry(key).or_default();
            entry.push(item);
        })
    }
}

impl<T> IterExt for T where T: Iterator {}
