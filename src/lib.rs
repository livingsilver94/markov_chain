use std::collections::HashMap;
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
trait Token: Clone + Eq + Hash {}

type Followers<T> = Option<HashMap<T, usize>>;

enum KeyPosition<T> {
    Beginning,
    Body(Vec<T>),
}

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
        let mut tokens = tokens.into_iter();
        // First, insert the Beginning node
        self.update_entry(KeyPosition::Beginning, tokens.next());
        let tokens_vec: Vec<_> = tokens.collect();
        // Now, the tokens in the middle
        let last_win = tokens_vec.windows(self.order+1).fold(None, |_, win|{
            self.update_entry(KeyPosition::Body(win[..self.order - 1].to_vec()), Some(win[self.order].clone()));
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
                let followers = followers.or_insert_with(|| Some(HashMap::new())).as_mut().unwrap();
                followers.entry(thing.clone()).and_modify(|counter| *counter += 1).or_insert(1);
            },
            None => {
                followers.or_insert(None);
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::{MarkovChain, Token};
//     use std::collections::HashMap;

//     fn hashmap_creator<K: Token, V>(tuples: Vec<(K, V)>) -> HashMap<K, V> {
//         let map: HashMap<K, V> = tuples.into_iter().collect();
//         map
//     }

//     #[test]
//     fn train_first_order() {
//         let mut map = MarkovChain::<&str>::new(1);
//         map.train("one fish two fish red fish red fish".split_whitespace());
//         let graph = &map.graph;
//         assert_eq!(
//             graph.get(&vec!["one"]).unwrap(),
//             &hashmap_creator(vec!(("fish", 1usize)))
//         );
//         assert_eq!(
//             graph.get(&vec!["fish"]).unwrap(),
//             &hashmap_creator(vec![("two", 1usize), ("red", 2usize)])
//         );
//     }

//     #[test]
//     fn train_second_order() {
//         let mut map = MarkovChain::<&str>::new(2);
//         map.train("one fish two fish red fish blue fish".split_whitespace());
//         let graph = &map.graph;
//         assert_eq!(
//             graph.get(&vec!["one", "fish"]).unwrap(),
//             &hashmap_creator(vec!(("two", 1usize)))
//         );
//         assert_eq!(
//             graph.get(&vec!["fish", "blue"]).unwrap(),
//             &hashmap_creator(vec![("fish", 1usize)])
//         );
//     }
// }
