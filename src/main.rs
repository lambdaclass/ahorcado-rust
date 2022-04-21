use rand::Rng;
use std::fs;
mod hangman;

const FILENAME:&str = "res/words";

fn main_loop(hangman:&mut hangman::Hangman) {
    hangman.show_game();
    let attempt_result = hangman.usr_input();

    match attempt_result {
        hangman::AttemptResult::CanKeepPlaying => main_loop(hangman),
        hangman::AttemptResult::Win => {
            hangman.show_game();
            println!("Ganaste!");
        },
        hangman::AttemptResult::Hanged => {
            hangman.show_game();
            println!("Ahorcado! X_X");
        },
    }
}

fn read_file_lines(filename: String) -> Vec<String> {
    let file: String = fs::read_to_string(filename).expect("Couldn't read file");
    file.lines().map(|l| l.to_string()).collect()
}

fn main() {
    let words = read_file_lines(String::from(FILENAME));
    let mut rng = rand::thread_rng();
    let indx = rng.gen_range(0..words.len());

    let game = &mut hangman::Hangman::new(words[indx].to_string());
    main_loop(game);
}
