extern crate core;

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};

static WORDS: &'static str = include_str!("resources/words.txt");

#[derive(Debug)]
struct Letter {
    char: char,
    used: bool,                   // false -> grey
    is_at_correct_position: bool, // true->green, else -> yellow
}

pub struct Game {
    is_won: bool,
    is_over: bool,
    attempts_remaining: u8,
    letters: HashMap<char, Letter>,
    valid_words: HashSet<String>,
    attempted_words: Vec<String>,
    current_attempt: String,
    target: String,
}
#[derive(Debug, Clone)]
pub struct InvalidEntryError;
impl Display for InvalidEntryError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "invalid word")
    }
}

impl Game {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
       // let words = WORDS
       //     .split('\n')
       //     .map(|x| String::from(x).to_ascii_uppercase())
       //     .collect::<HashSet<String>>();
        let words_vec = WORDS.split('\n').map(|x| String::from(x).to_ascii_uppercase()).collect::<Vec<String>>();
        let random_word= words_vec.get(rng.gen_range(0..words_vec.len())).unwrap().clone();
        let words = HashSet::from_iter(words_vec.into_iter());

        let letters = ('A'..='Z')
            .into_iter()
            .map(|x| Letter {
                char: x,
                used: false,
                is_at_correct_position: false,
            })
            .collect::<Vec<Letter>>();
        let mut letter_map: HashMap<char, Letter> = HashMap::new();
        for l in letters {
            letter_map.insert(l.char, l);
        }
        Game {
            is_won: false,
            is_over: false,
            attempts_remaining: 5,
            letters: letter_map,
            valid_words: words,
            attempted_words: vec![],
            current_attempt: "".to_string(),
            target: String::from(random_word)
        }
    }
    pub fn attempt(&mut self, word: String) -> Result<(), InvalidEntryError> {
        let word = word.to_ascii_uppercase();
        if word.len() != 5 {
            return Err(InvalidEntryError);
        }
        if self.valid_words.contains(&*word) {
            for c in word.chars() {
                let entry = self.letters.get_mut(&c).unwrap();
                entry.used = true;
            }
            self.attempted_words.push(word);
        }
        Ok(())
    }
}
impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for attempt in &self.attempted_words {
            writeln!(board, "|{:^10}|", attempt)?;
        }
        write!(
            f,
            "{}\n{:?}\n{:?}",
            board,
            self.letters
                .iter()
                .filter(|(_k, v)| v.used)
                .map(|x| x.1)
                .collect::<Vec<&Letter>>(),
            self.letters
                .iter()
                .filter(|(_k, v)| !v.used)
                .map(|x| x.1)
                .collect::<Vec<&Letter>>()
        )
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn test_test() {
        assert_eq!(1, 1)
    }
    #[test]
    fn test_debug_display() {
        let mut g = Game::new();
        println!("{}", g.target);
        if let Ok(_) = g.attempt(String::from("tesla")) {
            print!("{}", g);
            println!("{:?}", g.letters);
        }
         if let Ok(_) = g.attempt(String::from("AISLE")) {
            print!("{}", g);
            println!("{:?}", g.letters);
        }
    }
}
