use std::{fs::File, io::Read};
use rand::Rng;
use std::io::{self, Write};
use super::game_state::GameState;

pub struct Word {
	letters: Vec<Letter>
}

pub struct Letter {
	character: char,
	displayed: bool,
}

impl Word {
	pub fn new() -> Word {
		let mut raw_json_string = String::new();
		{
			let mut file = File::open("data/words.json").expect("cannot open words.json file");
			file.read_to_string(&mut raw_json_string).expect("cannot read words.json file");
		}
		let all_words: Vec<String> = serde_json::from_str(&raw_json_string).expect("cannot deserialize raw_json_string");

		let mut word = Word {letters: Vec::new()};
		let mut rng = rand::thread_rng();
		for character in  all_words[rng.gen_range(0..all_words.len()) as usize].clone().chars() {
			word.letters.push(Letter { character, displayed: false });
		}

		return word;
	}

	pub fn display(&self) {
		for letter in &self.letters {
			if letter.displayed { print!(" {} ", letter.character.to_uppercase()); }
			else { print!(" _ "); }
		}
		io::stdout().flush().unwrap();
	}

	pub fn guess(&mut self, attempts: &mut u8, game_state: &mut GameState) {
		let mut input = String::new();
		print!("GUESS THE LETTER => ");
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).expect("failed to read user input");

		let guess: Vec<char> = input.chars().collect();
		let mut guessed = false;
		let mut guessed_letters = 0;
		for letter in &mut self.letters {
			if letter.character == guess[0] { letter.displayed = true; guessed = true; }
			if letter.displayed { guessed_letters += 1; }
		}

		if	guessed_letters == self.letters.len() { *game_state = GameState::Won; return; }
		if !guessed { *attempts -= 1; }
	}
}