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
        self
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
