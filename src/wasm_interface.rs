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
