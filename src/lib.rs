use std::collections::HashMap;
use std::hash::Hash;

// This is a workaround while we wait for https://github.com/rust-lang/rust/issues/41517 to be merged
// Copied for here: https://github.com/aatxe/markov/blob/stable/src/lib.rs#L59
trait Token: Clone + Eq + Hash  {}
impl<T> Token for T where T: Clone + Eq + Hash {}

struct MarkovChain<T> where T: Token {
    order: u8,
    graph: HashMap<Vec<T>, HashMap<T, usize>>,
}

impl<T> MarkovChain<T> where T: Token {
    fn new(order: u8) -> MarkovChain<T> {
        MarkovChain{
            order,
            graph: HashMap::new(),
        }
    }

    fn train(&mut self, tokens: &[T]) -> &Self {
        self
    }
}
