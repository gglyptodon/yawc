pub mod lib;
mod wasm_bindings;

use crate::lib::Game;
fn main() {
    let mut g = Game::new();
    g.attempt(String::from("TESLA")).unwrap();
    print!("\n--{}--\n", g.attempts_remaining);
    for l in &g.letters {
        print!("{}", l.1);
    }
    g.attempt(String::from("AISLE")).unwrap();
    print!("\n--{}--\n", g.attempts_remaining);
    for l in &g.letters {
        print!("{}", l.1);
    }
    g.attempt(String::from("IDIOT")).unwrap();
    print!("\n--{}--\n", g.attempts_remaining);

    for l in &g.letters {
        print!("{}", l.1);
    }

    print!("\n---\n");
    g.attempt(String::from(&g.target)).unwrap();
    for l in &g.letters {
        print!("{}", l.1)
    }
    print!("\n--{}--\n", g.attempts_remaining);
    println!("\n{:^10}\n", g.target);
}
