# rust_forth
A very simple executable that takes in forth code and evaluates it.
Acts like a REPL with a permanent stack - whatever code you evaluate will have its effects put on the stack permanently

This was written to help introduce myself to the Rust programming language.

Unfortunately, not all forth words are supported. However, the ones that are include:
`+ = * / mod . dup clear .s`

More words will be implemented soon.
