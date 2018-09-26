use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
trait Token: Clone + Eq + Hash {}
impl<T> Token for T where T: Clone + Eq + Hash {}

type Followers<T> = HashMap<Option<T>, usize>;

#[derive(Clone, Hash, PartialEq)]
enum KeyPosition<T> {
    Beginning,
    Body(T),
}

impl<T> Eq for KeyPosition<T> where T: PartialEq {}

struct MarkovChain<T> {
    order: usize,
    graph: HashMap<Vec<KeyPosition<T>>, Followers<T>>,
}

impl<T> MarkovChain<T>
where
    KeyPosition<T>: Token,
    T: Token,
{
    fn new(order: usize) -> Self {
        MarkovChain {
            order,
            graph: HashMap::new(),
        }
    }

    fn train(&mut self, tokens: impl IntoIterator<Item = T>) -> &mut Self {
        let mut key = VecDeque::from(vec![KeyPosition::Beginning; self.order]);
        for item in tokens.into_iter() {
            self.update_entry(key.iter().cloned(), Some(item.clone()));
            key.pop_front();
            key.push_back(KeyPosition::Body(item.clone()));
        }
        self.update_entry(key.iter().cloned(), None);
        self
    }

    fn update_entry(&mut self, key: impl IntoIterator<Item = KeyPosition<T>>, value: Option<T>) {
        let followers = self
            .graph
            .entry(key.into_iter().collect())
            .or_insert_with(HashMap::new);
        followers
            .entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{Followers, KeyPosition, MarkovChain, Token};
    use std::collections::HashMap;

    fn hashmap_creator<K: Token>(pairs: Vec<(Option<K>, usize)>) -> Followers<K> {
        let map: HashMap<_, _> = pairs.into_iter().collect();
        map
    }

    #[test]
    fn train_first_order() {
        let mut map = MarkovChain::<&str>::new(1);
        map.train("one fish two fish red fish red fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph.get(&vec![KeyPosition::Body("fish")]).unwrap(),
            &hashmap_creator(vec![(Some("two"), 1), (Some("red"), 2), (None, 1)])
        );
    }

    #[test]
    fn train_second_order() {
        let mut map = MarkovChain::<&str>::new(2);
        map.train("one fish two fish red fish blue fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph.get(&vec![KeyPosition::Beginning, KeyPosition::Beginning]).unwrap(),
            &hashmap_creator(vec![(Some("one"), 1)])
        );
        assert_eq!(
            graph.get(&vec![KeyPosition::Beginning, KeyPosition::Body("one")]).unwrap(),
            &hashmap_creator(vec![(Some("fish"), 1)])
        );
    }
}
