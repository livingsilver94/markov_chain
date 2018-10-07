extern crate markov_chain;

use markov_chain::MarkovChain;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
	let path = env::args_os().nth(1);
	if path.is_none() {
		eprintln!("Did you provide a file path?");
		process::exit(1);
	}
	match File::open(path.unwrap()) {
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
			print!("{:#?}", chain.generate_from_rnd_token(100).1.collect::<Vec<_>>());
		}
		Err(err) => {
			eprintln!("{}", err);
			process::exit(2);
		}
	}
}
