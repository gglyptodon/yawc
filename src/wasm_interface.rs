use crate::yawc::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
   static YAWC: RefCell<Game> = RefCell::new(Game::new());
}

#[wasm_bindgen(js_name=getState)]
pub fn get_state() -> String {
    YAWC.with(|y| y.borrow().to_string())
}

#[wasm_bindgen(js_name=showAttempts)]
pub fn show_attempts() -> String {
    YAWC.with(|y| y.borrow().show_attemps())
}

#[wasm_bindgen(js_name=getAttemptsRemaining)]
pub fn get_attempts_remaining() -> u8 {
    YAWC.with(|y| y.borrow().attempts_remaining)
}

#[wasm_bindgen(js_name=getWord)]
pub fn get_word() -> String {
    YAWC.with(|y| y.borrow().target.clone())
}

#[wasm_bindgen(js_name=attemptWord)]
pub fn attempt_word(word: String) {
    YAWC.with(|y| {
        let _ = y.borrow_mut().attempt(word);
    })
}

#[wasm_bindgen(js_name=getKbUsed)]
pub fn get_kb_used(character: String) -> bool {
    YAWC.with(|y| {
        y.borrow().is_used(character) //todo this is not a nice workaround
    })
}

#[wasm_bindgen(js_name=getKbIncorrect)]
pub fn get_kb_incorrect(character: String) -> bool {
    YAWC.with(|y| y.borrow().is_at_incorrect_position(character))
}

#[wasm_bindgen(js_name=getKbCorrect)]
pub fn get_kb_correct(character: String) -> bool {
    YAWC.with(|y| y.borrow().is_at_correct_position(character))
}
