use std::collections::HashMap;
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
trait Token: Clone + Eq + Hash {}
impl<T> Token for T where T: Clone + Eq + Hash {}

type Followers<T> = Option<HashMap<T, usize>>;

#[derive(Clone, Hash, PartialEq)]
enum KeyPosition<T> {
    Beginning,
    Body(Vec<T>),
}

impl<T> Eq for KeyPosition<T> where T: PartialEq {}

struct MarkovChain<T> {
    order: usize,
    graph: HashMap<KeyPosition<T>, Followers<T>>,
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
        let tokens_vec: Vec<_> = tokens.into_iter().collect();
        // First, insert the Beginning node
        self.update_entry(KeyPosition::Beginning, tokens_vec.first().cloned());
        // Now, the tokens in the middle
        let last_win = tokens_vec
            .windows(self.order + 1)
            .fold(None, |_, win| {
                self.update_entry(
                    KeyPosition::Body(win[..self.order].to_vec()),
                    Some(win[self.order].clone()),
                );
                Some(win)
            }).unwrap();
        // And finally, the key with no followers
        self.update_entry(KeyPosition::Body(last_win[1..].to_vec()), None);
        self
    }

    fn update_entry(&mut self, key: KeyPosition<T>, value: Option<T>) {
        let followers = self.graph.entry(key);
        match value {
            Some(thing) => {
                let followers = followers
                    .or_insert_with(|| Some(HashMap::new()))
                    .as_mut()
                    .unwrap();
                followers
                    .entry(thing)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            None => {
                followers.or_insert(None);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Followers, KeyPosition, MarkovChain, Token};
    use std::collections::HashMap;

    fn hashmap_creator<K: Token>(pairs: Vec<(K, usize)>) -> Followers<K> {
        let map: HashMap<K, usize> = pairs.into_iter().collect();
        Some(map)
    }

    #[test]
    fn train_first_order() {
        let mut map = MarkovChain::<&str>::new(1);
        map.train("one fish two fish red fish red fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph.get(&KeyPosition::Beginning).unwrap(),
            &hashmap_creator(vec!(("one", 1)))
        );
        assert_eq!(
            graph.get(&KeyPosition::Body(vec!["fish"])).unwrap(),
            &hashmap_creator(vec![("two", 1), ("red", 2)])
        );
    }

    #[test]
    fn train_second_order() {
        let mut map = MarkovChain::<&str>::new(2);
        map.train("one fish two fish red fish blue fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph.get(&KeyPosition::Body(vec!["one", "fish"])).unwrap(),
            &hashmap_creator(vec!(("two", 1)))
        );
        assert_eq!(
            graph.get(&KeyPosition::Body(vec!["fish", "blue"])).unwrap(),
            &hashmap_creator(vec![("fish", 1)])
        );
    }
}
