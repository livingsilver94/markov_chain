use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
pub trait Token: Clone + Eq + Hash {}
impl<T> Token for T where T: Clone + Eq + Hash {}

pub struct Followers<T> {
    occurs: HashMap<Option<T>, usize>,
    freq_sum: usize,
}

impl<T: Token> Followers<T> {
    pub fn new() -> Self {
        Followers {
            occurs: HashMap::<_, _>::new(),
            freq_sum: 0,
        }
    }

    pub fn add(&mut self, follower: Option<T>) -> &Self {
        self.occurs
            .entry(follower)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        self.freq_sum += 1;
        self
    }

    pub fn occurs(&self) -> &HashMap<Option<T>, usize> {
        &self.occurs
    }

    pub fn freq_sum(&self) -> usize {
        self.freq_sum
    }
}

#[derive(Clone, Hash, PartialEq)]
pub enum KeyPosition<T> {
    Beginning,
    Body(T),
}

impl<T> Eq for KeyPosition<T> where T: PartialEq {}

pub struct MarkovChain<T> {
    order: usize,
    graph: HashMap<Vec<KeyPosition<T>>, Followers<T>>,
}

impl<T> MarkovChain<T>
where
    KeyPosition<T>: Token,
    T: Token,
{
    pub fn new(order: usize) -> Self {
        MarkovChain {
            order,
            graph: HashMap::new(),
        }
    }

    pub fn train(&mut self, tokens: impl IntoIterator<Item = T>) -> &mut Self {
        let mut key = VecDeque::from(vec![KeyPosition::Beginning; self.order]);
        for item in tokens.into_iter() {
            self.update_entry(key.iter().cloned(), Some(item.clone()));
            key.pop_front();
            key.push_back(KeyPosition::Body(item));
        }
        self.update_entry(key.into_iter(), None);
        self
    }

    fn update_entry(&mut self, key: impl IntoIterator<Item = KeyPosition<T>>, value: Option<T>) {
        let followers = self
            .graph
            .entry(key.into_iter().collect())
            .or_insert_with(Followers::new);
        followers.add(value);
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyPosition, MarkovChain, Token};
    use std::collections::HashMap;

    fn hashmap_creator<K: Token>(occurs: Vec<(Option<K>, usize)>) -> HashMap<Option<K>, usize> {
        let map: HashMap<_, _> = occurs.into_iter().collect();
        map
    }

    #[test]
    fn train_first_order() {
        let mut map = MarkovChain::<&str>::new(1);
        map.train("one fish two fish red fish red fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph
                .get(&vec![KeyPosition::Body("fish")])
                .unwrap()
                .occurs(),
            &hashmap_creator(vec![(Some("two"), 1), (Some("red"), 2), (None, 1)])
        );
    }

    #[test]
    fn train_second_order() {
        let mut map = MarkovChain::<&str>::new(2);
        map.train("one fish two fish red fish blue fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph
                .get(&vec![KeyPosition::Beginning, KeyPosition::Beginning])
                .unwrap()
                .occurs(),
            &hashmap_creator(vec![(Some("one"), 1)])
        );
        assert_eq!(
            graph
                .get(&vec![KeyPosition::Beginning, KeyPosition::Body("one")])
                .unwrap()
                .occurs(),
            &hashmap_creator(vec![(Some("fish"), 1)])
        );
    }
}
