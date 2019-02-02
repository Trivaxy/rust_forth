use std::io;

// Takes in a pre-defined word, matches it, and carries out its function
// Mathematical operators are separated from normal words because I think it'd be cleaner that way
pub fn do_word(w: &String, stack: &mut Vec<i32>) {
    let stack_length = stack.len();

    match w.to_lowercase().as_ref() {
        "mod" => {
            if stack.len() >= 2 {
                stack[stack_length - 2] %= stack[stack_length - 1];
                stack.pop();
            }
        },
        "." => {
            if stack.len() > 0 {
                stack.pop();
            }
        },
        "dup" => {
            if stack.len() > 0 {
                stack.push(stack[stack_length - 1]);
            }
        },
        "clear" => {
            stack.clear();
        },
        ".s" => {
            for i in stack {
                print!("{} ", i);
            }
            println!();
        },
        _ => ()
    }
}

// Is this word proper or available?
pub fn is_word(w: &str) -> bool {
    let words = vec!["mod", ".", "dup", "clear", ".s"];

    for word in words {
        if w == word.to_uppercase() {
            return true;
        }
    }
    false
}

// Carries out an arithmetic operation. Takes the stack in as a mutable borrow
pub fn arithmetic(w: &String, stack: &mut Vec<i32>) {
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

// Is +, -, * or / ?
pub fn is_math_op(s: &str) -> bool {
    if s == "+" || s == "-" || s == "*" || s == "/" {
        return true
    }
    false
}

// Returns a list of words from user line input to be passed into the interpreter
// This is an Option<T> type because in the rare case retrieving fails, we return Option::None
pub fn get_words() -> Option<Vec<String>> {
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