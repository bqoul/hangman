mod class;
use class::{hangman::Hangman, word::Word, game_state::GameState};

fn main() {
	let mut game_state = GameState::Started;

	let mut attempts: u8 = 6;
	let hangman = Hangman::new();
	let mut word = Word::new();

	loop {
		match game_state {
			GameState::Started => {
				println!("! WELOME TO THE HANGMAN GAME !");
				game_state = GameState::Going;
			},
			GameState::Going => {
				hangman.display(&attempts);
				word.display();

				if attempts == 0 { game_state = GameState::Lost; continue; }
				else if attempts == 1 { println!("\nLAST ATTEMPT!"); }
				else { println!("\nYOU HAVE {} ATTEMPTS LEFT!", attempts); }

				word.guess(&mut attempts, &mut game_state);
			},
			GameState::Won => {
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