// from https://docs.rs/im/7.1.0/src/im/hashmap/mod.rs.html#584-590
use std::hash::Hash;
use std::collections::HashMap;

/// Merges two hashmaps, using f where the keys exist in both hashmaps
pub fn union_of<K, V>(one: &HashMap<K, V>, two: &HashMap<K, V>, f: fn(V, V) -> V) -> HashMap<K, V>
where K: Hash + Eq + Clone,
      V: Clone
{
    two
        .iter()
        .fold(one.clone(), |mut m: HashMap<K, V>, (k, v)| {
                m.insert(
                    k.clone(),
                    one.get(k).map(|v1: &V| f(v1.clone(), v.clone())).unwrap_or(v.clone())
                );
                m
            }
        )
}
