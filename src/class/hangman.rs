use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Deserialize)]
pub struct Hangman {
	figure: Vec<String>,
}

impl Hangman {
	pub fn new() -> Self {
		let mut raw_json_string = String::new();
		{
			let mut file = File::open("data/hangman.json").expect("cant open hangman.json file");
			file.read_to_string(&mut raw_json_string).expect("cant read hangman.json file");
		}
		return Hangman {figure: serde_json::from_str(&raw_json_string).unwrap()};
	}

	pub fn display(&self, attempts: &u8) {
		println!("{}", self.figure[*attempts as usize]);
	}
}