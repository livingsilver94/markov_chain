extern crate markov_chain;

use markov_chain::MarkovChain;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

#[test]
fn generate_from_file() {
	match File::open("data/A Descent into the Maelström.txt") {
		Ok(file) => {
			let reader = BufReader::new(file);
			let mut chain = MarkovChain::new(2);
			for line_result in reader.lines() {
				match line_result {
					Ok(line) => {
						chain.train(line.split_whitespace().map(|s| s.to_lowercase()));
					}
					Err(err) => {
						// TODO: DRY-fy this block with the last one
						eprintln!("{}", err);
						process::exit(2);
					}
				}
			}
			println!(
				"{}",
				chain
					.generate_string_from_rnd_token(100)
					.1
			);
		}
		Err(err) => {
			eprintln!("{}", err);
			process::exit(2);
		}
	}
}
