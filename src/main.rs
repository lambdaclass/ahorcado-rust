use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod hangman;
use hangman::hangman::{Hangman, AttemptResult};

fn main_loop(hangman:&mut Hangman) {
    hangman.show_game();
    let attempt_result = hangman.usr_input();

    match attempt_result {
        AttemptResult::CanKeepPlaying => main_loop(hangman),
        AttemptResult::Win => {
            hangman.show_game();
            println!("Ganaste!");
        },
        AttemptResult::Hanged => {
            hangman.show_game();
            println!("Ahorcado! X_X");
        },
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut words = Vec::<String>::new();
    if let Ok(lines) = read_lines("res/words") {
        for line in lines {
            words.push(line.expect("Couldn't read file."));
        }
    }

    let mut rng = rand::thread_rng();
    let indx = rng.gen_range(0..words.len());

    let game = &mut Hangman::new(words[indx].to_string());
    main_loop(game);
}
