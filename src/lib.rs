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

struct Game {
    is_won: bool,
    is_over: bool,
    letters: HashMap<char, Letter>,
    valid_words: HashSet<String>,
    attempted_words: Vec<String>,
    current_attempt: String,
}
impl Game {
    pub fn new() -> Self {
        //let dictionary = PROJECT_DIR.get_file("resources/words.txt").unwrap();
        let words = WORDS.split("\n").map(|x|String::from(x).to_ascii_uppercase()).collect::<HashSet<String>>();

        let letters = ('a'..='z')
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
            letters: letter_map,
            valid_words: words,
            attempted_words: vec![],
            current_attempt: "".to_string(),
        }
    }
    pub fn attempt(&mut self, word: String) {
        let word = word.to_ascii_uppercase();
        if self.valid_words.contains(&*word) {
            //todo
            for c in word.chars() {
                let c= c.to_ascii_lowercase();
                println!("c: {}", c);
                let entry = self.letters.get_mut(&c).unwrap();
                entry.used = true;
                println!("entry {:?}", self.letters.get(&c).unwrap());
            }

        self.attempted_words.push(word);
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for attempt in &self.attempted_words {
            write!(board, "{}\n", attempt)?;
        }
        write!(
            f,
            "{}\n{:?}\n{:?}",
            board,
            self.letters
                .iter()
                .filter(|(k,v)| v.used)
                .map(|x|x.1)
                .collect::<Vec<&Letter>>(),
            self.letters
                .iter()
                .filter(|x| !x.1.used).map(|x|x.1)
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
        println!("{:?}", g.valid_words);

        println!("{}", g);

        g.attempt(String::from("cognac"));
        print!("{}", g);
        println!("{:?}", g.letters);
    }
}
