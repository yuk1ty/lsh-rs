pub struct LshError {
    pub message: String,
}

impl LshError {
    pub fn new(message: &str) -> LshError {
        LshError { message: message.to_string() }
    }
}

pub enum Status {
    Success,
    Exit
}