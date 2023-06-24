use crate::prelude;

/// its good practice to separate the code that generates errors (e.g. scanner/parser) from the
/// code that reports them. various phases of the front end will detect errors, but its not really
/// their job to know how to present that to a user. in a full-featured language implementation you
/// will likely have multiple ways errors get displayed: on stderr, in an IDE's error window,
/// logged onto a file, etc. Please don't put that code all over the scanner/parser. Ideally put it
/// in some kind of abstraction (e.g. `ErrorReporter` passed to scanner and parser)
fn report(line: u64, location: &str, message: &str) {
    eprintln!(
        "[line {}] Error{}: {}",
        line,
        location,
        message,
    );

    // tell the program to finish what its doing and then die
    let mut had_error = prelude::HAD_ERROR.lock().unwrap();
    *had_error = true;
}

/// used for syntax errors
pub fn handle_error(message: &str, line: u64) {
    report(line, "", message);
}
