use std::fs;
use std::io;
use rand::Rng;

#[derive(Debug)]
pub struct Letter {
	pub character: char,
	pub is_revealed: bool
}

pub enum GameProgress {
	InProgress,
	Won,
	Lost
}

impl Letter {

	pub fn create_letter(word: &String) -> Vec<Letter> {
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

pub fn select_word() -> String {

	let data = fs::read_to_string("words.txt")
		.expect("Unable to read file!");
	
	let words: Vec<&str> = data.trim().split(',').collect();
	
	let random_index = rand::thread_rng().gen_range(0, words.len());
	
	return words[random_index].to_string();
}


pub fn accept_user_input() -> char {

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

pub fn display_progress(letters: &Vec<Letter>) {

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

pub fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {

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
