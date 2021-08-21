mod hangman;
mod word;

use {hangman::Hangman, word::Word};

pub enum GameState {
	Started,
	Going,
	Won,
	Lost,
}

fn main() {
	print!("{}c", 27 as char); //clearing terminal window

	let mut game_state = GameState::Started;
	let mut attempts: u8 = 6;
	let mut word = Word::new();
	let hangman = Hangman::new();

	loop {
		match game_state {
			GameState::Started => {
				println!("! WELOME TO THE HANGMAN GAME !");
				game_state = GameState::Going;
			},

			GameState::Going => {
				hangman.display(&attempts);
				word.display();

				match attempts {
					0 => { game_state = GameState::Lost; continue; }
					1 => { println!("\n\nLAST ATTEMPT!"); }
					_ => { println!("\n\nYOU HAVE {} ATTEMPTS LEFT!", attempts); }
				}

				word.display_attempted();
				word.guess(&mut attempts, &mut game_state);
				print!("{}c", 27 as char); //clearing terminal window
			},

			GameState::Won => {
				//displaying ascii image for 6 attempts
				hangman.display(&6);
				word.display();

				println!("\n\nGOOD JOB! YOU SAVED THE HANGMAN");
				break;
			},

			GameState::Lost => {
				println!("\n\nYOU RAN OUT OF ATTEMPTS! HANGMAN IS NOW DEAD");
				break;
			},
		}
	}
}