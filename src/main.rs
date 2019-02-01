use std::vec::Vec;
use std::io;

pub mod word;

fn main() {
    println!("Forth interpreter written by Trivaxy, in Rust");
    println!("Not all standard words are provided!\n");

    let mut stack: Vec<i32> = Vec::new();

    loop {
        let words = get_words();

        match words {
            Some(t) => {
                r#do(&t, &mut stack);
            },
            None => ()
        }
    }
}

// Returns a list of words from user line input to be passed into the interpreter
// This is an Option<T> type because in the rare case retrieving fails, we return Option::None
fn get_words() -> Option<Vec<String>> {
    let mut line = String::new();

    // Read line. It's type is Result<T>, so we must match for either possible scenario
    match io::stdin().read_line(&mut line) {
        // If everything went fine, we'll split string by space, and return valid words as a Option::Some<T>
        Ok(_) => {
            let mut words: Vec<String> = Vec::new();
            for s in line.to_uppercase().split(" ") {
                if !s.is_empty() {
                    words.push(s.trim().to_string());
                }
            }
            Some(words)
        },
        Err(_) => {
            println!("Error occurred!");
            None
        }
    }
}

// Loop through the code by word and carry it out.
fn r#do(words: &Vec<String>, stack: &mut Vec<i32>) {
    for w in words {
        // If the word is a number, push it onto the stack - otherwise continue
        match w.parse::<i32>() {
            Ok(t) => stack.push(t),
            Err(_) => ()
        }

        // If the word is a mathematical operator, carry it out
        if is_math_op(&w) {
            arithmetic(&w, stack);
        }

        // If the word is a pre-existing forth word, carry out its function
        // Not all forth words are provided, unfortunately. There's way too many
        if word::is_word(&w) {
            word::do_word(&w, stack);
        }
    }

    // Finally, print the stack
    println!("=> {:?}", stack);
}

// Carries out an arithmetic operation. Takes the stack in as a mutable borrow
fn arithmetic(w: &String, stack: &mut Vec<i32>) {
    // We have to make a stack_length here otherwise we'll get borrowing errors
    let stack_length = stack.len();

    if stack_length < 2 {
        println!("ERROR: Attempt to do arithmetic with insufficient numbers on the stack");
        return;
    }

    match w.as_ref() {
        "+" => {
            stack[stack_length - 2] += stack[stack_length - 1];
            stack.pop();
        },
        "-" => {
            stack[stack_length - 2] -= stack[stack_length - 1];
            stack.pop();
        },
        "*" => {
            stack[stack_length - 2] *= stack[stack_length - 1];
            stack.pop();
        },
        "/" => {
            stack[stack_length - 2] /= stack[stack_length - 1];
            stack.pop();
        },
        _ => ()
    }
}

// Prints the content of the stack provided
fn print_stack(s: &Vec<i32>) {
    for n in (0..s.len()).rev() {
        println!("{}", s[n]);
    }
}

// Is +, -, * or / ?
fn is_math_op(s: &str) -> bool {
    if s == "+" || s == "-" || s == "*" || s == "/" {
        return true
    }
    false
}