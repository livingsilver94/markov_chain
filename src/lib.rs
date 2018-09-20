use std::collections::HashMap;
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
trait Token: Clone + Eq + Hash {}
impl<T> Token for T where T: Clone + Eq + Hash {}

struct MarkovChain<T> {
    order: usize,
    graph: HashMap<Vec<T>, HashMap<T, usize>>,
}

impl<T: Token> MarkovChain<T> {
    // TODO: filter on order >= 1
    fn new(order: usize) -> MarkovChain<T> {
        MarkovChain {
            order,
            graph: HashMap::new(),
        }
    }

    fn train(&mut self, tokens: impl IntoIterator<Item = T>) -> &mut Self {
        for list in (tokens.into_iter().collect::<Vec<T>>()).windows(self.order + 1) {
            let children = self
                .graph
                .entry(list[..self.order].to_vec())
                .or_insert_with(HashMap::new);
            children
                .entry(list[self.order].clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{MarkovChain, Token};
    use std::collections::HashMap;

    fn hashmap_creator<K: Token, V>(tuples: Vec<(K, V)>) -> HashMap<K, V> {
        let map: HashMap<K, V> = tuples.into_iter().collect();
        map
    }

    #[test]
    fn train_first_order() {
        let mut map = MarkovChain::<&str>::new(1);
        map.train("one fish two fish red fish red fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph.get(&vec!["one"]).unwrap(),
            &hashmap_creator(vec!(("fish", 1usize)))
        );
        assert_eq!(
            graph.get(&vec!["fish"]).unwrap(),
            &hashmap_creator(vec![("two", 1usize), ("red", 2usize)])
        );
    }

    #[test]
    fn train_second_order() {
        let mut map = MarkovChain::<&str>::new(2);
        map.train("one fish two fish red fish blue fish".split_whitespace());
        let graph = &map.graph;
        assert_eq!(
            graph.get(&vec!["one", "fish"]).unwrap(),
            &hashmap_creator(vec!(("two", 1usize)))
        );
        assert_eq!(
            graph.get(&vec!["fish", "blue"]).unwrap(),
            &hashmap_creator(vec![("fish", 1usize)])
        );
    }
}
