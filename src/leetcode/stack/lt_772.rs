use std::{collections::HashMap, hash::Hash};

pub trait ShrinkableToHashMap<'a, K, V>: Iterator<Item = &'a (K, V)> + Sized
where
    K: 'a + Hash + PartialEq + Eq + Clone,
    V: 'a + Clone,
{
    fn to_hashmap(self) -> HashMap<K, V> {
        self.fold(HashMap::new(), |mut map, (k, v)| {
            map.insert(k.to_owned(), v.to_owned());
            map
        })
    }
}

impl<'a, K, V, I> ShrinkableToHashMap<'a, K, V> for I
where
    K: 'a + Hash + PartialEq + Eq + Clone,
    V: 'a + Clone,
    I: Iterator<Item = &'a (K, V)>,
{
}

pub trait FixedToHashMap<K, V, const N: usize>: Sized
where
    K: Hash + PartialEq + Eq + Clone,
    V: Clone,
{
    fn to_hashmap(self) -> HashMap<K, V>;
}

impl<K, V, const N: usize> FixedToHashMap<K, V, N> for [(K, V); N]
where
    K: Hash + PartialEq + Eq + Clone,
    V: Clone,
{
    fn to_hashmap(self) -> HashMap<K, V> {
        self.iter().to_hashmap()
    }
}

#[cfg(test)]
mod test_to_hashmap {
    use super::*;

    #[test]
    fn it_works() {
        let a = [('a', 1), ('b', 2), ('c', 3)].to_hashmap();
        eprintln!("{:?}", a);
        let b = [('a', 1), ('b', 2), ('c', 3)].iter().to_hashmap();
        eprintln!("{:?}", b);
    }
}
