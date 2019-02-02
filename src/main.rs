use std::vec::Vec;

pub mod stack;

use stack as st;

fn main() {
    println!("Forth interpreter written by Trivaxy, in Rust");
    println!("Not all standard words are provided!\n");

    let mut stack: Vec<i32> = Vec::new();

    loop {
        let words = st::get_words();

        match words {
            Some(t) => {
                r#do(&t, &mut stack);
            },
            None => ()
        }
    }
}

// Loop through the code by word and carry it out.
fn r#do(words: &Vec<String>, stack: &mut Vec<i32>) {
    for w in words {
        // This variable is just to know whether we should complain about a word being unknown
        let mut word_exists = false;

        // If the word is a number, push it onto the stack - otherwise continue
        match w.parse::<i32>() {
            Ok(t) => {
                stack.push(t);
                word_exists = true;
            },
            Err(_) => ()
        }

        // If the word is a mathematical operator, carry it out
        if st::is_math_op(&w) {
            st::arithmetic(&w, stack);
            word_exists = true;
        }

        // If the word is a pre-existing forth word, carry out its function
        // Not all forth words are provided, unfortunately. There's way too many
        if st::is_word(&w) {
            st::do_word(&w, stack);
            word_exists = true;
        }

        if !word_exists {
            println!("ERROR: Unknown word {}", w);
        }
    }

    // Finally, print the stack
    println!("=> {:?}", stack);
}