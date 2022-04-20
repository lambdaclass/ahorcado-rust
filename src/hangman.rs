#[derive(Default)]
pub struct Hangman {
    word: String,
    attempts: usize,
    misses: [char; 5],
    hits: Vec<char>,
}

pub enum AttemptResult {
    CanKeepPlaying,
    Win,
    Hanged,
}

const SYMBOL_LETTER_TO_COMPLETE:char = '_';

impl Hangman {
    pub fn new(word: String) -> Hangman {
        let vec_size = word.len();
        Hangman { 
            word, 
            attempts: 0, 
            misses: Default::default(), 
            hits: vec![SYMBOL_LETTER_TO_COMPLETE; vec_size],
        }
    }

    fn guess_letter(&mut self, l:char) -> AttemptResult {
        
        if self.hits.contains(&l) || self.misses.contains(&l) {
            return AttemptResult::CanKeepPlaying;
        }

        let mut guessed_letter = false;
        let mut missing_letters = false;

        for (i, c) in self.word.chars().enumerate(){
            if c == l {
                self.hits[i] = l;
                guessed_letter = true;
            }

            if self.hits[i] == SYMBOL_LETTER_TO_COMPLETE {
                missing_letters = true;
            }
        }

        if !missing_letters {
            AttemptResult::Win
        } else if guessed_letter {
            AttemptResult::CanKeepPlaying
        } else {
            self.misses[self.attempts] = l;
            self.attempts += 1;

            if self.attempts == 5 {
                AttemptResult::Hanged
            } else {
                AttemptResult::CanKeepPlaying
            }
        }
    }

    pub fn texto_juego(&self) {
        const TITLE:&str  = "Bienvenido al hangman de LAMBDA!";
        const WORD:&str = "La word hasta el momento es:";
        const ATTEMPTS:(&str, &str) = ("Te quedan", "attempts.");
        const LETTERS:&str = "Letras usadas:";
        const PROMPT:&str = "Ingresa una letra:";


        let mut guessed_vec:Vec<char> = Vec::<char>::new();

        for l in &self.hits {
            guessed_vec.push(*l);
            guessed_vec.push(' ');
        }
        guessed_vec.pop();

        let guessed:String = guessed_vec.into_iter().collect();

        
        let len_misses = self.misses.len();
        let mut misses_vec = vec![' '; len_misses * 2 - 1];

        for (i, l) in self.misses[..len_misses - 1].into_iter().enumerate() {
            misses_vec[i * 2] = *l; 
        }

        let misses:String = misses_vec.into_iter().collect();


        std::process::Command::new("clear").status().expect("Error when opening the file.");
        println!("{}", TITLE);
        println!("{} {:#?}", WORD, guessed);
        println!("{} {} {}", ATTEMPTS.0, 5 - self.attempts, ATTEMPTS.1);
        println!("{} {}", LETTERS, misses);
        println!("{}", PROMPT);
    }

    pub fn entrada_usr(&mut self) -> AttemptResult {
        let mut v = String::from("");
        std::io::stdin().read_line(&mut v).expect("Error reading the line.");
        self.guess_letter(v.as_bytes()[0] as char)
    }
}
