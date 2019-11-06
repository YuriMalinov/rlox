use std::fmt::Debug;

pub trait ErrorHandler : Debug {
    fn error(&self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: u32, position: &str, message: &str);
}

#[derive(Debug)]
pub struct StdErrErrorHandler {}

impl ErrorHandler for StdErrErrorHandler {
    fn report(&self, line: u32, position: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, position, message)
    }
}