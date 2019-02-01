use std::vec::Vec;
use std::io;

fn main() {
    println!("Forth interpreter written by Trivaxy, in Rust");
    println!("Not all standard words are provided!\n");

    let words = get_words();

    match words {
        Some(t) => print_stack(&r#do(&t)),
        None => ()
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

// Loop through the code by word and carry it out. Return final stack
fn r#do(words: &Vec<String>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for w in words {
        // If the word is a number, push it onto the stack - otherwise continue
        match w.parse::<i32>() {
            Ok(t) => stack.push(t),
            Err(_) => ()
        }

        // If the word is a mathematical operator, carry it out
        if is_math_op(&w) {
            arithmetic(&w, &mut stack);
        }
    }
    stack
}

// Carries out an arithmetic operation. Takes the stack in as a mutable borrow
fn arithmetic(w: &String, stack: &mut Vec<i32>) {
    // We have to make a stack_length here otherwise we'll get borrowing errors
    let stack_length = stack.len();

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
    for n in s {
        println!("{}", n);
    }
}

// Is +, -, / or * ?
fn is_math_op(s: &str) -> bool {
    if s == "+" || s == "-" || s == "*" || s == "/" {
        return true
    }
    false
}