extern crate rand;

use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
pub trait Token: Clone + Eq + Hash {}
impl<T> Token for T where T: Clone + Eq + Hash {}

#[derive(Default)]
pub struct Followers<T: Token> {
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

    pub fn random_follower(&self) -> &Option<T> {
        let mut rnd_weight = rand::thread_rng().gen_range(0, (self.freq_sum as i32) + 1);
        self.occurs
            .iter()
            .find(|tup| {
                rnd_weight -= *tup.1 as i32;
                rnd_weight <= 0
            }).unwrap()
            .0
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum KeyPosition<T: Token> {
    Beginning,
    Body(T),
}

pub struct MarkovChain<T: Token> {
    order: usize,
    graph: HashMap<Vec<KeyPosition<T>>, Followers<T>>,
}

impl<T: Token> MarkovChain<T>
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

    pub fn generate_from_token(
        &self,
        token: impl Into<Vec<KeyPosition<T>>>,
        max: usize,
    ) -> Vec<&T> {
        let mut key_queue = VecDeque::from(token.into());
        let mut ret = vec![];
        for _ in 0..max {
            let key_vec: Vec<KeyPosition<T>> = key_queue.iter().cloned().collect();
            if let Some(follow) = self.graph.get(&key_vec) {
                if let Some(tok) = follow.random_follower() {
                    ret.push(tok);
                    key_queue.pop_front();
                    key_queue.push_back(KeyPosition::Body(tok.clone()));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        ret
    }

    pub fn generate(&self, max: usize) -> Vec<&T> {
        self.generate_from_token(vec![KeyPosition::Beginning; self.order], max)
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
