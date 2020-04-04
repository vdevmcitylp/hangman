use rand::Rng;
use std::fs;
use std::io;

const MAXIMUM_TURNS: u8 = 5;

#[derive(Debug)]
struct Letter {
	character: char,
	is_revealed: bool
}

enum GameProgress {
	InProgress,
	Won,
	Lost
}

fn main() {
    
    println!("\nWelcome to Hangman!\n");

    let selected_word = select_word();

    let mut letters = Letter::populate_struct(&selected_word);

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

fn select_word() -> String {

	let mut data = fs::read_to_string("words.txt").expect("Unable to read file!");
	data = data.trim().to_string();
	
	let words: Vec<&str> = data.split(',').collect();
	let random_index = rand::thread_rng().gen_range(0, words.len());
	
	return words[random_index].to_string();
}

impl Letter {

	fn populate_struct(word: &String) -> Vec<Letter> {
		let mut letters: Vec<Letter> = Vec::new();

		for c in word.chars() {
			letters.push(Letter {
				character: c, 
				is_revealed: false
			});
		}

		return letters;
	}
}

fn accept_user_input() -> char {

	let mut user_input = String::new();
	match io::stdin().read_line(&mut user_input) {
		Ok(_) => {
			match user_input.chars().next() {
				Some(c) => return c,
				None => return '*',
			}
		}
		Err(_) => {
			return '*'
		}
	}
}

fn display_progress(letters: &Vec<Letter>) {

	let mut display_string = String::from("Progress:");

	for letter in letters {
		display_string.push(' ');

		if letter.is_revealed {
			display_string.push(letter.character);
		} else {
			display_string.push('_');
		}
	}
	println!("{}\n", display_string);
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {

	let mut all_revealed = true;
	
	for letter in letters {
		if !letter.is_revealed {
			all_revealed = false;
		}
	}

	if turns_left > 0 {
		if !all_revealed {
			return GameProgress::InProgress;
		}
		return GameProgress::Won;
	}
	
	return GameProgress::Lost;
}

