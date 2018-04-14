pub struct LshError {
    pub message: &'static str,
}

impl LshError {
    pub fn new(message: &'static str) -> LshError {
        LshError { message }
    }
}

pub enum Status {
    Success,
    Exit
}