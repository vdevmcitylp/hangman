use hangman::Letter;
use hangman::GameProgress;

use hangman::select_word;
use hangman::display_progress;
use hangman::accept_user_input;
use hangman::check_progress;

const MAXIMUM_TURNS: u8 = 5;


fn main() {
    
    println!("\nWelcome to Hangman!");

    let selected_word = select_word();

    let mut letters = Letter::create_letter(&selected_word);

    let mut turns_left = MAXIMUM_TURNS;

    loop {
    	println!("\nYou have {} turns left.\n", turns_left);
   		display_progress(&letters);

   		println!("Enter your guess.");
    	let user_input = accept_user_input();
    	
    	if user_input == '*' {
    		break;
    	}

    	let mut atleast_one_is_correct = false;
    	for mut letter in &mut letters {
    		if user_input == letter.character {
    			letter.is_revealed = true;
    			atleast_one_is_correct = true;
    		}
    	}

    	if !atleast_one_is_correct {
    		turns_left -= 1;
    	}

    	match check_progress(turns_left, &letters) {
    		GameProgress::Won => {
    			println!("\nCongratulations, you won! The word was {}.", &selected_word);
    			break;
    		}
    		GameProgress::Lost => {
    			println!("\nSorry, you lost! The word was {}.", &selected_word);
    			break;
    		}
    		GameProgress::InProgress => {
    			continue;
    		}
    	}
    }
}


